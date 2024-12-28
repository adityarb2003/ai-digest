use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use walkdir::WalkDir;
use std::sync::mpsc;
use rayon::prelude::*;

pub use crate::utils::*;

pub mod utils {
    use super::*;

    pub fn read_ignore_file(ignore_file: &str) -> Vec<String> {
        if let Ok(content) = fs::read_to_string(ignore_file) {
            content.lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn is_binary(file_path: &Path) -> bool {
        if let Ok(metadata) = fs::metadata(file_path) {
            metadata.is_file() && fs::read(file_path).ok().map_or(false, |data| data.contains(&0))
        } else {
            false
        }
    }

    pub fn handle_file_content(path: &Path, whitespace_removal: bool) -> io::Result<String> {
        let content = fs::read_to_string(path)?;
        let processed_content = if whitespace_removal {
            // Remove extra leading/trailing whitespaces but preserve indentation
            content
                .lines()
                .map(str::trim_start)
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            content
        };
        Ok(processed_content)
    }
}

pub fn process_files(
    input_dir: &str,
    output_file: &str,
    ignore_patterns: &[String],
    whitespace_removal: bool,
    show_output_files: bool,
) -> io::Result<()> {
    let (tx, rx) = mpsc::channel();

    WalkDir::new(input_dir)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            let path = entry.path();
            !path.is_dir()
                && !ignore_patterns.iter().any(|p| path.to_str().unwrap_or_default().contains(p))
        })
        .for_each_with(tx.clone(), |tx, entry| {
            let path = entry.path();

            if utils::is_binary(path) {
                let _ = tx.send((path.to_path_buf(), "[Binary file omitted]".to_string()));
            } else {
                match utils::handle_file_content(path, whitespace_removal) {
                    Ok(content) => {
                        let _ = tx.send((path.to_path_buf(), content));
                    }
                    Err(_) => {
                        let _ = tx.send((path.to_path_buf(), "[Error reading file]".to_string()));
                    }
                }
            }
        });

    drop(tx);

    let mut output = File::create(output_file)?;

    for (path, content) in rx {
        if show_output_files {
            println!("Including file: {}", path.display());
        }
        writeln!(output, "## File: {}\n", path.display())?;
        writeln!(output, "```\n{}\n```\n", content)?;
    }

    Ok(())
}