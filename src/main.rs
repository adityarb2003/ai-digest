use ai_digest::{process_files, utils::read_ignore_file};
use clap::{Arg, Command, ArgAction};

fn main() -> std::io::Result<()> {
    let matches = Command::new("ai-digest")
        .version("1.0")
        .about("Aggregates your codebase into a Markdown file")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("DIRECTORY")
                .help("Specify input directory")
                .default_value("."),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Specify output file")
                .default_value("codebase.md"),
        )
        .arg(
            Arg::new("no-default-ignores")
                .long("no-default-ignores")
                .help("Disable default ignore patterns")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("whitespace-removal")
                .long("whitespace-removal")
                .help("Enable whitespace removal")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-output-files")
                .long("show-output-files")
                .help("Display a list of files included in the output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ignore-file")
                .long("ignore-file")
                .value_name("FILE")
                .help("Specify a custom ignore file")
                .default_value(".exclude"),
        )
        .get_matches();

    // Extract CLI arguments
    let input_dir = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();
    let no_default_ignores = matches.get_flag("no-default-ignores");
    let whitespace_removal = matches.get_flag("whitespace-removal");
    let show_output_files = matches.get_flag("show-output-files");
    let ignore_file = matches.get_one::<String>("ignore-file").unwrap();

    println!("Input directory: {}", input_dir);
    println!("Output file: {}", output_file);
    println!("No default ignores: {}", no_default_ignores);
    println!("Whitespace removal: {}", whitespace_removal);
    println!("Show output files: {}", show_output_files);
    println!("Ignore file: {}", ignore_file);

    // Default ignore patterns
    let mut ignore_patterns: Vec<String> = if !no_default_ignores {
        vec![
            "idea","exclude","target", "node_modules", ".git", "package-lock.json", "npm-debug.log", "yarn.lock",
            "yarn-error.log", "pnpm-lock.yaml", "bun.lockb", "deno.lock", "vendor",
            "composer.lock", "__pycache__", "*.pyc", "*.pyo", "*.pyd", ".Python", "pip-log.txt",
            "pip-delete-this-directory.txt", ".venv", "venv", "ENV", "env", ".godot", "*.import",
            "Gemfile.lock", ".bundle", "*.class", ".gradle", "build", "pom.xml.tag",
            "pom.xml.releaseBackup", "pom.xml.versionsBackup", "pom.xml.next", "bin", "obj",
            "*.suo", "*.user", "go.sum", "Cargo.lock", ".svn", ".hg", ".DS_Store", "Thumbs.db",
            ".env", ".env.local", ".env.development.local", ".env.test.local", ".env.production.local",
            "*.env", "*.env.*", ".svelte-kit", ".next", ".nuxt", ".vuepress", ".cache", "dist", "tmp",
            "codebase.md", ".turbo", ".vercel", ".netlify", "LICENSE",
        ]
            .into_iter()
            .map(String::from)
            .collect()
    } else {
        vec![]
    };

    ignore_patterns.extend(read_ignore_file(ignore_file));
    println!("Final ignore patterns: {:?}", ignore_patterns);

    process_files(
        input_dir,
        output_file,
        &ignore_patterns,
        whitespace_removal,
        show_output_files,
    )
}
