# Code Scanner Toolkit

Code Scanner Toolkit provides ready-to-run scripts (Bash and Python) that walk through software projects, extract source files and relevant configuration, and produce concise text bundles you can share or audit. The scripts intentionally ignore hefty build artifacts and secrets, keeping the output focused on the code that matters.

## Repository Layout

### Bash Scanner (Unified)
- `scripts/scan_project.sh` – single Bash scanner that merges the legacy/enhanced flows; includes .gitignore support, project type detection, and a Windows-friendly fallback
- `scripts/unified_scanner.py` – **Unified Python scanner** with auto-detection of project types, .gitignore support, and custom configuration

### Python Scanners (Specialized)
- `scripts/generate_project_summary.py` – optimized for web projects (JavaScript/TypeScript/React)
- `scripts/generate_project_summary_build.py` – for build/package projects with binary detection
- `scripts/generate_project_summary_django.py` – for Django/Python backend projects

### Directories
- `input/` – drop the projects or folders you want to scan; kept in Git via `.gitkeep`
- `output/` – generated reports (ignored by Git); each project becomes `<name>_project_code.txt` or `<name>_*_summary.txt`

## Requirements
### For Bash Scanner
- macOS or Linux with Bash 4+ (default on most systems)
- POSIX utilities already used by the script (`find`, `sed`, `awk`, `stat`, `file`, `tr`, `nl`, `grep`)

### For Python Scanners
- Python 3.6+ (no external dependencies required)

## Quick Start

### Unified Scanners (RECOMMENDED)

#### Unified Python Scanner (Best for Most Projects)
```bash
git clone <this-repo>
cd Code-Scanner
# Add the project(s) you want to review into the input/ directory
python3 scripts/unified_scanner.py
```

Features:
- **Auto-detects project type** (Node.js, Python, Java, Rust, Go, Flutter, etc.)
- **Respects .gitignore** files automatically
- **Custom configuration** via `.scanner-config.json` in project root
- **Smart filtering** of dependencies and build artifacts
- **Comprehensive language support** (20+ programming languages)

#### Unified Bash Scanner
```bash
./scripts/scan_project.sh
```

Features:
- **Project type detection** (Node.js, Python, Django, Java, Rust, Go, etc.)
- **.gitignore support** (enabled by default)
- **Verbose mode** for debugging
- **Windows-friendly** fallback for Git Bash/WSL (no separate script needed)

### Python Scanners (Specialized)
```bash
# For web projects (JavaScript/TypeScript/React)
python3 scripts/generate_project_summary.py

# For build/package projects
python3 scripts/generate_project_summary_build.py

# For Django/Python projects
python3 scripts/generate_project_summary_django.py
```

On first run, the scripts ensure `input/` and `output/` exist. If `input/` is empty, the scripts will notify you to add projects before running again.

## Customising the Scan

### Unified Scanners

#### Unified Python Scanner Configuration

The unified scanner supports custom configuration via `.scanner-config.json` in your project root:

```json
{
  "code_extensions": [".py", ".js", ".ts", ".jsx", ".tsx"],
  "ignore_dirs": ["node_modules", "dist", "build"],
  "ignore_files": ["*.log", "*.lock"],
  "ignore_extensions": [".pyc", ".png", ".jpg"],
  "target_subdirs": ["src", "lib"],
  "max_file_size": 1048576,
  "include_hidden": false
}
```

See `.scanner-config.example.json` for a full example.

Environment variables:
```bash
# Custom input/output directories
INPUT_DIR=./my-projects OUTPUT_DIR=./my-reports python3 scripts/unified_scanner.py
```

#### Unified Bash Scanner Configuration

Environment variables:
```bash
# Disable .gitignore support
USE_GITIGNORE=false ./scripts/scan_project.sh

# Enable verbose mode
VERBOSE=true ./scripts/scan_project.sh

# Custom directories and settings
TARGET_DIR=./my-project OUTPUT_DIR=./reports MAX_SIZE_BYTES=5242880 ./scripts/scan_project.sh
```

The Bash scanner is entirely driven by environment variables, so you can tailor it without editing the source:

| Variable | Purpose | Example |
| --- | --- | --- |
| `TARGET_DIR` | Directory to scan | `TARGET_DIR=./my-samples ./scripts/scan_project.sh` |
| `OUTPUT_DIR` | Where to write reports | `OUTPUT_DIR=./reports ./scripts/scan_project.sh` |
| `OUTPUT_FILE_SUFFIX` | Change filename suffix | `OUTPUT_FILE_SUFFIX=_audit.txt ./scripts/scan_project.sh` |
| `MAX_SIZE_BYTES` | Limit per file (default 2MB) | `MAX_SIZE_BYTES=$((1024*1024)) ./scripts/scan_project.sh` |
| `USE_GITIGNORE` | Respect project .gitignore | `USE_GITIGNORE=false ./scripts/scan_project.sh` |
| `VERBOSE` | Emit debug logs while filtering | `VERBOSE=true ./scripts/scan_project.sh` |
| `IGNORE_FILES_EXTRA` | Extra file patterns to skip | `IGNORE_FILES_EXTRA='*.snap|*.bin' ./scripts/scan_project.sh` |
| `IGNORE_DIRS_EXTRA` | Extra directories to skip | `IGNORE_DIRS_EXTRA='docs|examples' ./scripts/scan_project.sh` |
| `IGNORE_PATHS` | Relative paths inside a project | `IGNORE_PATHS='vendor/cache|data/generated' ./scripts/scan_project.sh` |
| `IGNORE_ABSOLUTE_PATHS` | Absolute directories to skip | `IGNORE_ABSOLUTE_PATHS="$PWD/input/big-lib" ./scripts/scan_project.sh` |

### Python Scanners
Python scanners support these environment variables:

| Variable | Purpose | Example |
| --- | --- | --- |
| `INPUT_DIR` | Directory to scan | `INPUT_DIR=./my-samples python3 scripts/generate_project_summary.py` |
| `OUTPUT_DIR` | Where to write reports | `OUTPUT_DIR=./reports python3 scripts/generate_project_summary.py` |
| `TARGET_SUBDIRS` | Subdirs to scan deeply (web only) | `TARGET_SUBDIRS=src,docs,lib python3 scripts/generate_project_summary.py` |
| `TARGET_SUBDIR` | Subdir to scan (build/django) | `TARGET_SUBDIR=backend python3 scripts/generate_project_summary_django.py` |

### Scanner Types and Use Cases

**`scan_project.sh` (Bash)** - Most comprehensive scanner
- Supports all major programming languages and frameworks
- Includes extensive configuration options
- Binary file detection
- Best for: General-purpose project analysis

**`generate_project_summary.py` (Python)** - Web Projects
- Optimized for JavaScript/TypeScript/React projects
- Scans multiple target directories (default: `src`, `docs`)
- Includes web-specific file types (JSX, TSX, SCSS, etc.)
- Best for: Frontend and full-stack web applications

**`generate_project_summary_build.py` (Python)** - Build Projects
- Binary file detection and handling
- Single target directory focus (default: `package`)
- Configuration and script files
- Best for: Build outputs, deployment packages, QPKG packages

**`generate_project_summary_django.py` (Python)** - Django/Python
- Optimized for Django/Python backend projects
- Handles Python-specific files (Pipfile, requirements.txt)
- Filters `__pycache__` and `.pyc` files
- Single target directory (default: `back`)
- Best for: Django applications and Python backends

### Output Format
Output files for each project contain:
1. A project overview
2. A tree view of directories and files that were included
3. Summaries for each included file with size metadata
4. Full contents of text-based files (with line numbers for Bash scanner)

## Unified Scanner Features

### Auto-Detection of Project Types

The unified scanners automatically detect and adapt to the following project types:

| Project Type | Detection Criteria |
|--------------|-------------------|
| **Node.js** | `package.json`, `node_modules` |
| **Python** | `requirements.txt`, `setup.py`, `pyproject.toml`, `Pipfile` |
| **Django** | `manage.py`, `settings.py`, `wsgi.py` |
| **React** | `package.json` + React components in `src/` |
| **Next.js** | `next.config.js`, `pages/`, `app/` |
| **Vue** | `vue.config.js`, `src/App.vue` |
| **Angular** | `angular.json`, `src/app` |
| **Java (Maven)** | `pom.xml`, `mvnw` |
| **Java (Gradle)** | `build.gradle`, `gradlew` |
| **Spring Boot** | `pom.xml` + `application.properties/yml` |
| **Rust** | `Cargo.toml`, `Cargo.lock` |
| **Go** | `go.mod`, `go.sum` |
| **.NET** | `.csproj`, `.sln` files |
| **PHP/Laravel** | `composer.json`, `artisan` |
| **Ruby/Rails** | `Gemfile`, `Rakefile` |
| **Flutter** | `pubspec.yaml`, `lib/main.dart` |
| **Docker** | `Dockerfile`, `docker-compose.yml` |

### Smart Filtering

Unified scanners automatically ignore:
- **Dependencies**: `node_modules`, `vendor`, `Pods`, etc.
- **Build artifacts**: `dist`, `build`, `target`, `out`, etc.
- **Cache directories**: `.cache`, `__pycache__`, `.gradle`, etc.
- **IDE files**: `.vscode`, `.idea`, `.DS_Store`, etc.
- **Sensitive files**: `.env*`, `*.key`, `*.pem`, etc.
- **Large binaries**: Files exceeding configurable size limit
- **.gitignore patterns**: Respects project's .gitignore (optional)

### Supported Languages (20+)

Python, JavaScript, TypeScript, Java, Kotlin, Rust, Go, C, C++, C#, F#, Swift, Objective-C, Dart, Ruby, PHP, HTML, CSS, SCSS, Markdown, JSON, YAML, XML, TOML, Shell scripts, and more.

## Tips
- **Use the unified scanners first**: They provide the best balance of features and ease of use
- **Customize with .scanner-config.json**: Place in project root for project-specific settings
- Use `IGNORE_FILES_EXTRA` and `IGNORE_DIRS_EXTRA` (Bash) to keep noisy artefacts out of reports
- Large binary files are automatically skipped by all scanners
- If you only need a single project, place it directly in `input/` or point `TARGET_DIR`/`INPUT_DIR` to it
- Bash scanner outputs default to `<project>_project_code.txt` (override via `OUTPUT_FILE_SUFFIX`)
- Original Python scanners generate: `*_web_summary.txt`, `*_build_summary.txt`, `*_django_summary.txt`
- All scanners automatically create `input/` and `output/` directories if they don't exist
- Use `VERBOSE=true` with the Bash scanner to debug filtering issues

## Licensing
This project is released under the MIT License; see `LICENSE` for full details.
