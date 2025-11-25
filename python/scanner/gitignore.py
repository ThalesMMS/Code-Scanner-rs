"""Lightweight .gitignore parser used by scanners."""

import fnmatch
import os
import sys
from pathlib import Path
from typing import List, Optional, Tuple


class GitignoreParser:
    """Parses and applies .gitignore patterns."""

    def __init__(self, gitignore_path: Optional[Path] = None):
        self.patterns: List[Tuple[str, bool]] = []  # (pattern, is_negation)
        if gitignore_path and gitignore_path.exists():
            self.load(gitignore_path)

    def load(self, gitignore_path: Path) -> None:
        """Load patterns from .gitignore file."""
        try:
            with open(gitignore_path, "r", encoding="utf-8") as handle:
                for raw_line in handle:
                    line = raw_line.strip()
                    if not line or line.startswith("#"):
                        continue

                    is_negation = line.startswith("!")
                    if is_negation:
                        line = line[1:]

                    self.patterns.append((line, is_negation))
        except Exception as exc:
            print(f"Warning: Could not load .gitignore: {exc}", file=sys.stderr)

    def should_ignore(self, path: str) -> bool:
        """Check if a path should be ignored based on .gitignore patterns."""
        if not self.patterns:
            return False

        ignored = False
        for pattern, is_negation in self.patterns:
            if fnmatch.fnmatch(path, pattern) or fnmatch.fnmatch(os.path.basename(path), pattern):
                ignored = not is_negation

        return ignored

