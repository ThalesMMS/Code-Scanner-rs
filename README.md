# Code Scanner (Rust)

CLI that walks projects, lists relevant files, and generates a text report with a directory tree, file contents, and summary statistics. Respects `.gitignore` by default and allows overrides via `.scanner-config.json` in the target project.

## Requirements
- Rust 1.70+ (Cargo)

## Project structure
- `src/main.rs`: Single CLI entry point; argument parsing, filtering, and report generation.
- `Cargo.toml`: Package metadata and dependencies.
- `output/`: Generated reports (`<project>_project_code.txt`).

## How to run
```bash
# Default (uses ./input and ./output)
cargo run -- --input-dir ./input --output-dir ./output

# Verbose and ignoring the target project's .gitignore
cargo run -- --input-dir ./input --output-dir ./output --verbose --no-gitignore

# Optimized binary
cargo build --release
./target/release/code_scanner --input-dir ./input --output-dir ./output
```

## Main arguments
- `--input, -i`: Directory to scan (a single project or a folder containing multiple projects).
- `--output, -o`: Where to save reports.
- `--no-gitignore`: Ignore the target project's `.gitignore` rules.
- `--verbose, -v`: Extra logs during the scan.

## Optional configuration
Create `.scanner-config.json` in the target project to adjust filters:
```json
{
  "code_extensions": ["rs", "toml"],
  "ignore_dirs": ["target", "node_modules"],
  "max_file_size": 1048576
}
```

## Development
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`
- Tests: `cargo test` (add integration fixtures under `tests/`)
