"""Shared summary scanning utilities used by the Python entrypoints."""

import sys
from dataclasses import dataclass, field, replace
from pathlib import Path
from typing import List, Set, Tuple

from .file_utils import is_binary_file

INDENT_STRING = "  "


@dataclass(frozen=True)
class SummaryProfile:
    """Rules describing how to collect and render project summaries."""

    target_subdirs: Set[str]
    code_extensions: Set[str]
    include_root_files: Set[str]
    ignore_root_dirs: Set[str]
    ignore_content_files: Set[str] = field(default_factory=set)
    ignore_content_extensions: Set[str] = field(default_factory=set)
    ignore_any_dirs: Set[str] = field(default_factory=set)
    ignore_any_files: Set[str] = field(default_factory=set)
    binary_extensions: Set[str] = field(default_factory=set)
    allow_bare_without_extension: bool = False
    treat_filename_as_extension: bool = False
    max_file_size: int | None = None
    double_extension_parents: Set[str] = field(default_factory=lambda: {".js", ".mjs"})

    def with_target_subdirs(self, targets: Set[str]) -> "SummaryProfile":
        """Return a copy with updated target subdirectories."""
        return replace(self, target_subdirs=set(targets))

    @staticmethod
    def _double_extension(filename: str) -> str:
        """Return concatenated double extension like '.config.js' if present."""
        first_suffix = Path(filename).suffix
        if not first_suffix:
            return ""

        without_first = filename[: -len(first_suffix)]
        second_suffix = Path(without_first).suffix
        if not second_suffix:
            return ""

        return (second_suffix + first_suffix).lower()

    def should_include_content(self, file_path: Path, project_root: Path) -> bool:
        """Decide if file content should be included in the output."""
        relative = file_path.relative_to(project_root)
        filename = file_path.name
        suffix = file_path.suffix
        is_in_root = relative.parent == Path(".")

        lookup_ext = suffix.lower() if suffix else ""
        double_ext = self._double_extension(filename)
        effective_extension = double_ext or lookup_ext

        if self.treat_filename_as_extension and not suffix:
            effective_extension = filename

        if filename in self.ignore_content_files:
            return False

        if effective_extension and effective_extension in self.ignore_content_extensions:
            return False

        if is_in_root and filename in self.include_root_files:
            return True

        if effective_extension in self.code_extensions:
            return True

        if not suffix and self.allow_bare_without_extension:
            return not is_binary_file(file_path, self.binary_extensions)

        return False


class ProjectSummaryGenerator:
    """Builds project structure and content sections based on a profile."""

    def __init__(self, profile: SummaryProfile):
        self.profile = profile

    def collect_structure_and_files(self, project_dir: Path) -> Tuple[List[str], List[Path]]:
        """Return rendered tree lines and files chosen for content."""
        structure_lines: List[str] = [f"- {project_dir.name}/ (root)"]
        code_files: List[Path] = []
        self._walk(project_dir, project_dir, structure_lines, code_files, indent_level=1, process_children=False)
        return structure_lines, code_files

    def _walk(
        self,
        start_path: Path,
        root_dir: Path,
        structure_lines: List[str],
        code_files: List[Path],
        indent_level: int,
        process_children: bool,
    ) -> None:
        try:
            items = sorted(start_path.iterdir(), key=lambda path: (not path.is_dir(), path.name))
        except OSError as exc:
            print(f"Warning: Could not read directory {start_path}: {exc}", file=sys.stderr)
            return

        is_root_level = start_path == root_dir

        for item in items:
            is_dir = item.is_dir()
            name = item.name
            indent_prefix = INDENT_STRING * indent_level

            if is_dir and name in self.profile.ignore_any_dirs:
                continue
            if not is_dir and name in self.profile.ignore_any_files:
                continue
            if is_root_level and is_dir and name in self.profile.ignore_root_dirs:
                continue

            if is_dir:
                is_target_dir = is_root_level and name in self.profile.target_subdirs
                line = f"{indent_prefix}- {name}/"
                if is_root_level and not is_target_dir:
                    line += " [...ignored]"
                structure_lines.append(line)

                if process_children or is_target_dir:
                    self._walk(item, root_dir, structure_lines, code_files, indent_level + 1, True)
            else:
                structure_lines.append(f"{indent_prefix}- {name}")
                if process_children or is_root_level:
                    if self.profile.should_include_content(item, root_dir):
                        code_files.append(item)

    def write_structure(self, structure_lines: List[str], output_file) -> None:
        """Write pre-rendered structure lines to the output handle."""
        for line in structure_lines:
            output_file.write(f"{line}\n")

    def write_file_contents(self, code_files: List[Path], project_dir: Path, output_file) -> None:
        """Write file contents for the collected files."""
        for file_path in sorted(code_files):
            relative_path = file_path.relative_to(project_dir).as_posix()

            if self.profile.max_file_size is not None:
                try:
                    if file_path.stat().st_size > self.profile.max_file_size:
                        output_file.write(f"--- File: {relative_path} --- (CONTENT IGNORED - TOO LARGE)\n\n")
                        output_file.write("=" * 15 + f" End of {relative_path} " + "=" * 15 + "\n\n")
                        continue
                except OSError as exc:
                    output_file.write(f"--- File: {relative_path} ---\n\n")
                    output_file.write(f"*** Error checking file size: {exc} ***\n")
                    output_file.write("\n\n" + "=" * 15 + f" End of {relative_path} (with error) " + "=" * 15 + "\n\n")
                    continue

            if self.profile.binary_extensions and is_binary_file(file_path, self.profile.binary_extensions):
                output_file.write(f"--- File: {relative_path} ---\n\n")
                output_file.write("*** BINARY FILE - CONTENT NOT DISPLAYED ***\n")
                output_file.write("\n\n" + "=" * 15 + f" End of {relative_path} " + "=" * 15 + "\n\n")
                continue

            try:
                with open(file_path, "r", encoding="utf-8", errors="ignore") as handle:
                    content = handle.read()
                    if not content.strip() and file_path.stat().st_size == 0:
                        continue

                output_file.write(f"--- File: {relative_path} ---\n\n")
                output_file.write(content)
                output_file.write("\n\n" + "=" * 15 + f" End of {relative_path} " + "=" * 15 + "\n\n")

            except Exception as exc:
                output_file.write(f"--- File: {relative_path} ---\n\n")
                output_file.write(f"*** Error reading file: {exc} ***\n")
                output_file.write("\n\n" + "=" * 15 + f" End of {relative_path} (with error) " + "=" * 15 + "\n\n")
