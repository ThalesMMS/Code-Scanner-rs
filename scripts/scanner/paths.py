"""Common path helpers and defaults used by scanner scripts."""

from pathlib import Path

PACKAGE_ROOT = Path(__file__).resolve().parent
# PACKAGE_ROOT = scripts/scanner, so hop two levels to reach repo root.
REPO_ROOT = PACKAGE_ROOT.parent.parent
DEFAULT_INPUT_DIR = REPO_ROOT / "input"
DEFAULT_OUTPUT_DIR = REPO_ROOT / "output"
