# Code Scanner

Toolkit with three independent scanners that produce text bundles of your code: a Rust CLI, a Bash script, and Python scripts (one unified and three specialized). Pick the version you want by running the corresponding binary or script—no wrapper CLI required.

## What’s inside
- `src/` – Rust CLI that walks projects, respects `.gitignore`, and writes combined reports.
- `bash/scan_project.sh` – Bash scanner with project-type detection, `.gitignore` support, and verbose/debug modes.
- `python/unified_scanner.py` – Unified Python scanner with auto-detection of project types and `.gitignore` support.
- `python/generate_project_summary*.py` – Specialized Python scanners for web, build/package, and Django/Python backends.
- `input/` – Drop projects to scan (kept by `.gitkeep`).
- `output/` – Generated reports (ignored except for `.gitkeep`).
- `.scanner-config.example.json` – Example configuration shared by the scanners.

## Requirements
- Rust CLI: Rust 1.70+ with Cargo.
- Bash scanner: Bash 4+ on macOS/Linux with standard POSIX tools (`find`, `sed`, `awk`, `stat`, `nl`, `grep`, etc.).
- Python scanners: Python 3.6+ (no external dependencies).

## Quick start
```bash
git clone <this-repo>
cd Code-Scanner
# Add the project(s) you want to inspect inside input/
```

Run whichever scanner you prefer:
- Rust: `cargo run -- --input-dir ./input --output-dir ./output`
- Bash: `./bash/scan_project.sh`
- Python (unified): `python3 python/unified_scanner.py`
- Python (specialized):
  - Web projects: `python3 python/generate_project_summary.py`
  - Build/package projects: `python3 python/generate_project_summary_build.py`
  - Django/Python backends: `python3 python/generate_project_summary_django.py`

## Configuration
- `.scanner-config.json` in the target project adjusts code extensions, ignore lists, and max file size (see `.scanner-config.example.json`).
- Bash scanner environment examples:
  - `USE_GITIGNORE=false ./bash/scan_project.sh`
  - `TARGET_DIR=./my-project OUTPUT_DIR=./reports ./bash/scan_project.sh`
- Python scanners environment examples:
  - `INPUT_DIR=./my-projects OUTPUT_DIR=./reports python3 python/unified_scanner.py`
  - `TARGET_SUBDIR=backend python3 python/generate_project_summary_django.py`
- Rust CLI flags mirror the defaults used by the scripts:
  - `cargo run -- --input-dir ./input --output-dir ./output --no-gitignore --verbose`

## Output
Each project yields a text report in `output/`, typically named `<project>_project_code.txt` or `<project>_*_summary.txt` depending on the scanner. Large binaries, dependency folders, IDE files, and `.gitignore`d paths are skipped by default.

## Development (Rust)
- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`
- Tests: `cargo test`
