use ai_digest::{process_files, utils::read_ignore_file};
use clap::{Arg, Command,ArgAction};

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

    let input_dir = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();
    let no_default_ignores = matches.contains_id("no-default-ignores");
    let whitespace_removal = matches.contains_id("whitespace-removal");
    let show_output_files = matches.contains_id("show-output-files");
    let ignore_file = matches.get_one::<String>("ignore-file").unwrap();
    println!("Input directory: {}", input_dir);
    println!("Output file: {}", output_file);
    println!("No default ignores: {}", no_default_ignores);
    println!("Whitespace removal: {}", whitespace_removal);
    println!("Show output files: {}", show_output_files);
    println!("Ignore file: {}", ignore_file);
    let mut ignore_patterns: Vec<String> = if !no_default_ignores {
        vec![
            "target",                 // Rust
            "node_modules",           // Node.js
            ".git",                   // Git
            "package-lock.json",      // Node.js
            "npm-debug.log",          // Node.js
            "yarn.lock",              // Yarn
            "yarn-error.log",         // Yarn
            "pnpm-lock.yaml",         // pnpm
            "bun.lockb",              // Bun
            "deno.lock",              // Deno
            "vendor",                 // PHP (Composer)
            "composer.lock",          // PHP (Composer)
            "__pycache__",            // Python
            "*.pyc",                  // Python bytecode
            "*.pyo",                  // Python bytecode
            "*.pyd",                  // Python dynamic modules
            ".Python",                // Python environment
            "pip-log.txt",            // Python pip logs
            "pip-delete-this-directory.txt", // Python pip
            ".venv",                  // Python virtual environment
            "venv",                   // Python virtual environment
            "ENV",                    // Python environment
            "env",                    // Python environment
            ".godot",                 // Godot
            "*.import",               // Godot import files
            "Gemfile.lock",           // Ruby
            ".bundle",                // Ruby
            "*.class",                // Java
            ".gradle",                // Gradle
            "build",                  // Gradle build output
            "pom.xml.tag",            // Maven backup
            "pom.xml.releaseBackup",  // Maven backup
            "pom.xml.versionsBackup", // Maven backup
            "pom.xml.next",           // Maven backup
            "bin",                    // .NET
            "obj",                    // .NET
            "*.suo",                  // .NET
            "*.user",                 // .NET
            "go.sum",                 // Go
            "Cargo.lock",             // Rust
            ".svn",                   // SVN
            ".hg",                    // Mercurial
            ".DS_Store",              // macOS
            "Thumbs.db",              // Windows
            ".env",                   // Environment variable files
            ".env.local",             // Environment variable files
            ".env.development.local", // Environment variable files
            ".env.test.local",        // Environment variable files
            ".env.production.local",  // Environment variable files
            "*.env",                  // Environment variable files
            "*.env.*",                // Environment variable files
            ".svelte-kit",            // SvelteKit cache
            ".next",                  // Next.js cache
            ".nuxt",                  // Nuxt.js cache
            ".vuepress",              // VuePress cache
            ".cache",                 // Common framework cache
            "dist",                   // Build output
            "tmp",                    // Temporary files
            "codebase.md",            // The output Markdown file
            ".turbo",                 // Turborepo cache
            ".vercel",                // Vercel cache
            ".netlify",               // Netlify cache
            "LICENSE",                // License file
        ]
            .into_iter()
            .map(String::from)
            .collect()
    } else {
        vec![]
    };
    ignore_patterns.extend(read_ignore_file(ignore_file));

    process_files(
        input_dir,
        output_file,
        &ignore_patterns,
        whitespace_removal,
        show_output_files,
    )
}