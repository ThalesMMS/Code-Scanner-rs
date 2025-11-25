#!/usr/bin/env python3
"""Project Summary Generator for Web Projects."""

import os
import sys
from pathlib import Path
from typing import Set

from scanner.paths import DEFAULT_INPUT_DIR, DEFAULT_OUTPUT_DIR
from scanner.summary import ProjectSummaryGenerator, SummaryProfile

# Default target subdirectories to scan deeply
DEFAULT_TARGET_SUBDIRS = {"src", "docs"}

# File extensions to include content from
CODE_EXTENSIONS = {
    ".js",
    ".jsx",
    ".ts",
    ".tsx",
    ".html",
    ".css",
    ".scss",
    ".json",
    ".md",
    ".config.js",
    ".yaml",
    ".yml",
    ".sh",
    ".bash",
    ".mjs",
    ".puml",
    ".mermaid",
}

# Specific root files to include content from (if not in ignore list)
INCLUDE_ROOT_FILES_BY_NAME = {
    "eslint.config.js",
    "vite.config.js",
    "index.html",
    "package.json",
    "README.md",
    "citation.cff",
    "nest-cli.json",
    "tsconfig.json",
    "tsconfig.build.json",
}

# Files whose CONTENT will be ignored, even if extension/location matches
IGNORE_CONTENT_FILES = {"package-lock.json", ".gitignore"}

# Extensions whose CONTENT will be ignored (useful for assets like SVG, images, etc.)
IGNORE_CONTENT_EXTENSIONS = {
    ".svg",
    ".png",
    ".jpg",
    ".jpeg",
    ".gif",
    ".webp",
    ".ico",
    ".woff",
    ".woff2",
    ".ttf",
    ".otf",
}

# Directories to ignore at root level
IGNORE_DIRS_ROOT = {
    "node_modules",
    "dist",
    "build",
    ".git",
    ".vscode",
    "__pycache__",
    ".idea",
    "input",
    "output",
}

PROFILE = SummaryProfile(
    target_subdirs=DEFAULT_TARGET_SUBDIRS,
    code_extensions=CODE_EXTENSIONS,
    include_root_files=INCLUDE_ROOT_FILES_BY_NAME,
    ignore_root_dirs=IGNORE_DIRS_ROOT,
    ignore_content_files=IGNORE_CONTENT_FILES,
    ignore_content_extensions=IGNORE_CONTENT_EXTENSIONS,
)


def process_project(project_dir: Path, output_filename: Path, target_subdirs: Set[str]) -> bool:
    """Process a single project directory."""
    profile = PROFILE.with_target_subdirs(target_subdirs)
    generator = ProjectSummaryGenerator(profile)

    print(f"Processing: {project_dir.name}")
    print(f"  Output file: {output_filename}")
    print(f"  Target subdirectories for deep analysis: {', '.join(sorted(target_subdirs))}")
    print(f"  Extensions with content included: {', '.join(sorted(CODE_EXTENSIONS))}")
    print(f"  Root files with content included (by name): {', '.join(sorted(INCLUDE_ROOT_FILES_BY_NAME))}")
    print(f"  Files with content ignored (by name): {', '.join(sorted(IGNORE_CONTENT_FILES))}")
    print(f"  Extensions with content ignored: {', '.join(sorted(IGNORE_CONTENT_EXTENSIONS))}")
    print(f"  Directories ignored at root: {', '.join(sorted(IGNORE_DIRS_ROOT))}")

    try:
        structure_lines, code_files_to_read = generator.collect_structure_and_files(project_dir)
        with open(output_filename, "w", encoding="utf-8") as outfile:
            outfile.write("=" * 30 + "\n")
            outfile.write(" Project Structure\n")
            outfile.write("=" * 30 + "\n")
            outfile.write(f"(Content from subfolders except {', '.join(sorted(target_subdirs))} was ignored)\n")
            outfile.write(
                f"(Content from files like {', '.join(sorted(IGNORE_CONTENT_FILES))} "
                f"and extensions {', '.join(sorted(IGNORE_CONTENT_EXTENSIONS))} ignored)\n\n"
            )

            generator.write_structure(structure_lines, outfile)
            outfile.write("\n\n")
            outfile.write("=" * 30 + "\n")
            outfile.write(" Relevant File Contents\n")
            outfile.write("=" * 30 + "\n\n")

            generator.write_file_contents(code_files_to_read, project_dir, outfile)

        print(f"  ✓ Successfully generated '{output_filename}'!")
        return True

    except IOError as exc:
        print(f"\n  ✗ I/O error writing to file {output_filename}: {exc}", file=sys.stderr)
        return False
    except Exception as exc:
        print(f"\n  ✗ Unexpected error occurred: {exc}", file=sys.stderr)
        return False


def main() -> int:
    """Main function to process all projects in input directory."""
    input_dir = Path(os.environ.get("INPUT_DIR", DEFAULT_INPUT_DIR))
    output_dir = Path(os.environ.get("OUTPUT_DIR", DEFAULT_OUTPUT_DIR))

    target_subdirs_env = os.environ.get("TARGET_SUBDIRS", "")
    target_subdirs = set(target_subdirs_env.split(",")) if target_subdirs_env else DEFAULT_TARGET_SUBDIRS

    print("=" * 60)
    print("PROJECT SUMMARY GENERATOR - WEB PROJECTS")
    print("=" * 60)
    print(f"Input directory: {input_dir}")
    print(f"Output directory: {output_dir}")
    print(f"Target subdirectories: {', '.join(sorted(target_subdirs))}")
    print("=" * 60)
    print()

    if not input_dir.exists():
        print(f"Error: Input directory not found: {input_dir}", file=sys.stderr)
        print("Please create it and add projects to scan.", file=sys.stderr)
        return 1

    output_dir.mkdir(parents=True, exist_ok=True)
    projects = [path for path in input_dir.iterdir() if path.is_dir()]

    if not projects:
        print(f"Warning: No project directories found in {input_dir}", file=sys.stderr)
        print("Please add project directories to scan.", file=sys.stderr)
        return 1

    success_count = 0
    for project_path in sorted(projects):
        output_filename = output_dir / f"{project_path.name}_web_summary.txt"
        print(f"\n[Project: {project_path.name}]")
        if process_project(project_path, output_filename, target_subdirs):
            success_count += 1
        print()

    print("=" * 60)
    print(f"COMPLETED! Processed {success_count}/{len(projects)} projects")
    print(f"Output files in: {output_dir}")
    print("=" * 60)

    return 0 if success_count > 0 else 1


if __name__ == "__main__":
    sys.exit(main())

