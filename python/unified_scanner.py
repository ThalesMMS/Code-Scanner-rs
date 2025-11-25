#!/usr/bin/env python3
"""CLI wrapper for the unified code scanner."""

import os
import sys
from pathlib import Path

from scanner.paths import DEFAULT_INPUT_DIR, DEFAULT_OUTPUT_DIR
from scanner.unified import run_unified_scanner


def main() -> int:
    input_dir = Path(os.environ.get("INPUT_DIR", DEFAULT_INPUT_DIR))
    output_dir = Path(os.environ.get("OUTPUT_DIR", DEFAULT_OUTPUT_DIR))
    return run_unified_scanner(input_dir, output_dir)


if __name__ == "__main__":
    sys.exit(main())

