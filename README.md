# ai-digest

`ai-digest` is a CLI tool that aggregates your codebase into a single Markdown file. It is designed for use with Claude Projects or custom ChatGPTs, providing a summarized, clean, and easily shareable format of your project's code.

## Installation

To install `ai-digest` globally on your system, use the following command:

### Install via Cargo

If you have Rust and Cargo installed, you can install `ai-digest` globally using this command:

```bash
cargo install ai-digest
```

Once installed, the `ai-digest` command will be available from anywhere on your system.

## Usage

Once installed, you can use `ai-digest` from any project or directory by running:

```bash
ai-digest --input <directory> --output <output-file> [options]
```

### Options

* `--input` or `-i`: Specify the input directory to scan for files. Default is the current directory (`.`)
* `--output` or `-o`: Specify the output file for the generated Markdown. Default is `codebase.md`
* `--no-default-ignores`: Disable default ignore patterns (`target`, `node_modules`, `.git`)
* `--whitespace-removal`: Enable whitespace removal from file contents
* `--show-output-files`: Display a list of files included in the output
* `--ignore-file`: Specify a custom ignore file. Default is `.aidigestignore`

### Example

```bash
ai-digest --input ./path/to/project --output output.md --whitespace-removal --show-output-files
```

This will generate a Markdown file named `output.md`, summarizing the project in the `./path/to/project` directory. The output will have whitespace removed, and all included files will be listed in the output.

## Development

To contribute to `ai-digest` or modify it for your needs, clone the repository and build it locally:

```bash
git clone https://github.com/adityarb2003/ai-digest.git
cd ai-digest
cargo build
```

### Running Tests

To run the unit tests:

```bash
cargo test
```

### Publish to Crates.io

To publish `ai-digest` to Crates.io:

1. Log in to your Crates.io account:
```bash
cargo login
```

2. Publish the crate:
```bash
cargo publish
```

This will upload the crate to Crates.io, making it available for global installation via `cargo install ai-digest`.

## License

Distributed under the MIT License. See `LICENSE` for more information.