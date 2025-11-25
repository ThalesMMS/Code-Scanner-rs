"""File utilities shared across scanner implementations."""

from pathlib import Path
from typing import Iterable, Optional, Set


def is_binary_file(file_path: Path, binary_extensions: Optional[Iterable[str]] = None) -> bool:
    """Heuristic check for binary files."""
    ext = file_path.suffix.lower()
    if binary_extensions and ext in set(binary_extensions):
        return True

    try:
        with open(file_path, "rb") as handle:
            return b"\x00" in handle.read(1024)
    except Exception:
        # Fail closed to avoid dumping binary data by mistake.
        return True


def format_size(size_bytes: int) -> str:
    """Return a human-readable size string."""
    units = ["B", "KB", "MB", "GB", "TB"]
    value = float(size_bytes)
    for unit in units:
        if value < 1024.0 or unit == units[-1]:
            return f"{value:.2f} {unit}"
        value /= 1024.0

