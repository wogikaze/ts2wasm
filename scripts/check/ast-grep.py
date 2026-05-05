#!/usr/bin/env python3
"""Run ast-grep rule tests and repository scan."""

from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


def ast_grep_bin() -> str:
    for candidate in ("ast-grep", "sg"):
        if shutil.which(candidate):
            return candidate
    print("missing: ast-grep (or sg)", file=sys.stderr)
    return ""


def run(cmd: list[str]) -> int:
    print(f"check-ast-grep: {' '.join(cmd)}", file=sys.stderr)
    return subprocess.run(cmd, cwd=REPO_ROOT).returncode


def main() -> int:
    binary = ast_grep_bin()
    if not binary:
        return 127

    test_status = run([binary, "test", "--skip-snapshot-tests"])
    if test_status != 0:
        return test_status

    return run([binary, "scan", "crates", "scripts"])


if __name__ == "__main__":
    sys.exit(main())
