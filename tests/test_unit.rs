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
fn test_ignore_with_custom_exclude() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join(".exclude");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "target\nnode_modules\n.git").unwrap();

    let input_dir = dir.path().join("input");
    fs::create_dir(&input_dir).unwrap();

    let output_file = dir.path().join("output.md");
    let result = ai_digest::process_files(
        input_dir.to_str().unwrap(),
        output_file.to_str().unwrap(),
        &vec!["target",                 // Rust
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
              "LICENSE",].iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        false,
        false,
    );

    assert!(result.is_ok());
    assert!(output_file.exists());
}


#[test]
fn test_process_files_with_default_ignore() {
    let dir = tempdir().unwrap();

    // Create input directory and files
    let input_dir = dir.path().join("input");
    fs::create_dir_all(&input_dir).unwrap();

    // Creating files
    let file_path = input_dir.join("test.js");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "console.log('Hello, World!');").unwrap();

    let output_file = dir.path().join("output.md");

    let result = ai_digest::process_files(
        input_dir.to_str().unwrap(),
        output_file.to_str().unwrap(),
        &vec!["target",                 // Rust
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
              "LICENSE",].iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        false,
        false,
    );

    assert!(result.is_ok());
    assert!(output_file.exists());
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
    assert!(output_content.contains("test.txt"));
    assert!(output_content.contains("Hello, world!"));
}
