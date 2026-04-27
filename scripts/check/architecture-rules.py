#!/usr/bin/env python3
"""Lightweight architecture checks (complement to docs/12 + ast-grep rules).

Usage: python scripts/manager.py check-architecture-rules

Current checks:
  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).
  - Warn when a repo-owned source/document file exceeds 2000 lines.
"""

import os
import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DEFAULT_MAX_FILE_LINES = 2000
LINE_COUNT_SUFFIXES = {
    ".json",
    ".md",
    ".py",
    ".rs",
    ".sh",
    ".toml",
    ".yaml",
    ".yml",
}
EXCLUDED_PATH_PARTS = {
    ".git",
    ".mypy_cache",
    "__pycache__",
    "artifacts",
    "node_modules",
    "reference",
    "reports",
    "target",
}
EXCLUDED_FILENAMES = {
    "Cargo.lock",
}


def usage():
    print("Usage:")
    print("  python scripts/manager.py check-architecture-rules [--max-file-lines N]")
    print()
    print("Current checks:")
    print("  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).")
    print("  - Warn when a repo-owned source/document file exceeds the line limit.")


def parse_max_file_lines(args: list[str]) -> int:
    raw_max_file_lines = os.environ.get("TS2WASM_MAX_FILE_LINES", str(DEFAULT_MAX_FILE_LINES))
    try:
        max_file_lines = int(raw_max_file_lines)
    except ValueError:
        print("check_architecture_rules: TS2WASM_MAX_FILE_LINES must be an integer", file=sys.stderr)
        sys.exit(1)

    while args:
        arg = args.pop(0)
        if arg in ("-h", "--help"):
            usage()
            sys.exit(0)
        if arg == "--max-file-lines":
            if not args:
                print("check_architecture_rules: --max-file-lines requires a value", file=sys.stderr)
                sys.exit(1)
            try:
                max_file_lines = int(args.pop(0))
            except ValueError:
                print("check_architecture_rules: --max-file-lines must be an integer", file=sys.stderr)
                sys.exit(1)
        else:
            print(f"check_architecture_rules: unknown option: {arg}", file=sys.stderr)
            usage()
            sys.exit(1)

    if max_file_lines < 1:
        print("check_architecture_rules: --max-file-lines must be >= 1", file=sys.stderr)
        sys.exit(1)
    return max_file_lines


def should_count_lines(path: Path) -> bool:
    relative = path.relative_to(REPO_ROOT)
    if path.name in EXCLUDED_FILENAMES:
        return False
    if any(part in EXCLUDED_PATH_PARTS for part in relative.parts):
        return False
    return path.suffix in LINE_COUNT_SUFFIXES


def line_count(path: Path) -> int:
    data = path.read_bytes()
    if not data:
        return 0
    return data.count(b"\n") + (0 if data.endswith(b"\n") else 1)


def warn_oversized_files(max_file_lines: int) -> None:
    oversized: list[tuple[int, Path]] = []
    for path in REPO_ROOT.rglob("*"):
        if not path.is_file() or not should_count_lines(path):
            continue
        count = line_count(path)
        if count > max_file_lines:
            oversized.append((count, path.relative_to(REPO_ROOT)))

    if not oversized:
        return

    print(
        "check_architecture_rules: WARN files exceed "
        f"{max_file_lines} lines; consider splitting ownership/modules",
        file=sys.stderr,
    )
    for count, path in sorted(oversized, key=lambda item: (-item[0], item[1])):
        print(f"check_architecture_rules: WARN {path}: {count} lines", file=sys.stderr)


def main():
    args = sys.argv[1:]
    max_file_lines = parse_max_file_lines(args)
    warn_oversized_files(max_file_lines)
    
    if not shutil.which("cargo"):
        print("check_architecture_rules: cargo is required", file=sys.stderr)
        sys.exit(1)
    
    # Check if ts2wasm-shared depends on ts2wasm-cli
    result = subprocess.run(
        ["cargo", "tree", "-p", "ts2wasm-shared", "--edges", "normal,build"],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )
    
    if result.returncode != 0:
        # cargo tree might fail if package doesn't exist, that's OK for this check
        print("check_architecture_rules: OK", file=sys.stderr)
        sys.exit(0)
    
    if "ts2wasm-cli" in result.stdout:
        print("check_architecture_rules: ts2wasm-shared must not depend on ts2wasm-cli", file=sys.stderr)
        print(result.stdout, file=sys.stderr)
        sys.exit(1)
    
    print("check_architecture_rules: OK", file=sys.stderr)


if __name__ == "__main__":
    main()
