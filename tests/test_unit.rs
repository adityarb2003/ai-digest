// tests/test_unit.rs
use ai_digest::{utils::read_ignore_file};
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_read_ignore_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join(".aidigestignore");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "target\nnode_modules\n.git").unwrap();

    let patterns = read_ignore_file(file_path.to_str().unwrap());
    assert_eq!(patterns, vec!["target", "node_modules", ".git"]);
}

#[test]
fn test_is_binary() {
    let dir = tempdir().unwrap();
    let binary_file_path = dir.path().join("binary_file");
    let mut file = File::create(&binary_file_path).unwrap();
    file.write_all(&[0, 159, 146, 150]).unwrap();

    assert!(ai_digest::utils::is_binary(&binary_file_path));

    let text_file_path = dir.path().join("text_file.txt");
    let mut file = File::create(&text_file_path).unwrap();
    writeln!(file, "This is a text file.").unwrap();

    assert!(!ai_digest::utils::is_binary(&text_file_path));
}

#[test]
fn test_handle_file_content() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("file.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "   Line 1\n   Line 2   \nLine 3   ").unwrap();

    let content = ai_digest::utils::handle_file_content(&file_path, true).unwrap();
    assert_eq!(content, "Line 1\nLine 2\nLine 3");

    let content = ai_digest::utils::handle_file_content(&file_path, false).unwrap();
    assert_eq!(content, "   Line 1\n   Line 2   \nLine 3   \n");
}

#[test]
fn test_process_files() {
    let dir = tempdir().unwrap();

    // Create test directory structure
    let input_dir = dir.path().join("input");
    fs::create_dir(&input_dir).unwrap();

    // Create a test file
    let file_path = input_dir.join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello, world!").unwrap();

    // Create output file path
    let output_file = dir.path().join("output.md");

    // Test processing
    let ignore_patterns = vec![];
    let result = ai_digest::process_files(
        input_dir.to_str().unwrap(),
        output_file.to_str().unwrap(),
        &ignore_patterns,
        false,
        false,
    );

    assert!(result.is_ok());
    assert!(output_file.exists());

    // Verify output content
    let output_content = fs::read_to_string(output_file).unwrap();
    assert!(output_content.contains("# Codebase Aggregation"));
    assert!(output_content.contains("test.txt"));
    assert!(output_content.contains("Hello, world!"));
}
