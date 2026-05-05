#!/usr/bin/env python3
"""Detect trivial assert(true) in fixture files and test scripts.

Complex test cases (eval, legacy RegExp, generators, Date, TypedArray,
HTML-like comments, etc.) must not be replaced with assert(true) as a lazy
way to make tests pass.

Scans both fixtures/**/*.ts and scripts/**/*.py for assert(true).

Usage:
  mise run check assert-true        # full scan (both fixtures/ and scripts/)
  python scripts/check/assert-true-detect.py [--diff-only]

Options:
  --diff-only   Only check staged target files (pre-commit mode).
                Checks ALL content in staged files, not just new additions.

Dependencies: python3, rg (ripgrep)
"""

import sys
import subprocess
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()


def usage():
    print("Usage:")
    print("  mise run check assert-true")
    print("  python scripts/check/assert-true-detect.py [--diff-only]")
    print()
    print("Full scan: checks fixtures/**/*.ts and scripts/**/*.py for assert(true).")
    print("--diff-only: checks same patterns but only for staged files.")


def is_target_file(rel_path):
    return (
        rel_path.startswith("fixtures/") and rel_path.endswith(".ts")
    ) or (
        rel_path.startswith("scripts/") and rel_path.endswith(".py")
    )


def find_all_target_files():
    """Return all target files under repo."""
    paths = []
    for pattern in ["fixtures/**/*.ts", "scripts/**/*.py"]:
        paths.extend(REPO_ROOT.glob(pattern))
    return sorted(paths)


def get_staged_target_files():
    """Return list of staged target files."""
    result = subprocess.run(
        ["git", "diff", "--cached", "--name-only", "--diff-filter=ACM"],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        print("check_assert_true_detect: git diff failed", file=sys.stderr)
        sys.exit(1)
    paths = []
    for line in result.stdout.strip().splitlines():
        line = line.strip()
        if line and is_target_file(line):
            paths.append(REPO_ROOT / line)
    return paths


def scan_files(file_paths):
    """Scan files for assert(true). Return list of (file, line) tuples.

    Uses different patterns per file type:
      - .ts: assert(true) as a statement (^\s*assert\(true\))
      - .py: double-quoted "ASSERT_TRUE_STMT" as a string literal (matches code, not docstrings)
    """
    hits = []
    for f in file_paths:
        if not f.exists():
            continue
        if f.suffix == ".py":
            pattern = r'"assert\(true\);"'
        else:
            pattern = r"assert\(true\)"
        result = subprocess.run(
            ["rg", "-n", pattern, str(f)],
            capture_output=True, text=True, cwd=REPO_ROOT,
        )
        if result.returncode == 0:
            for line in result.stdout.strip().splitlines():
                hits.append((f, line))
    return hits


def main():
    args = sys.argv[1:]
    diff_only = "--diff-only" in args
    if "--help" in args or "-h" in args:
        usage()
        sys.exit(0)

    if diff_only:
        files = get_staged_target_files()
        if not files:
            print("check_assert_true_detect: no staged target files — skipping", file=sys.stderr)
            sys.exit(0)
    else:
        files = find_all_target_files()

    hits = scan_files(files)
    if hits:
        print("check_assert_true_detect: FAIL", file=sys.stderr)
        print("", file=sys.stderr)
        print("Found assert(true) — complex test cases must not be simplified", file=sys.stderr)
        print("to assert(true). Either implement real assertions or mark the test", file=sys.stderr)
        print("as intentionally trivial (e.g. assert(true, 'intentionally empty')).", file=sys.stderr)
        print("", file=sys.stderr)
        for f, line in hits:
            rel = f.relative_to(REPO_ROOT) if isinstance(f, Path) else f
            print(f"  {rel}:{line}", file=sys.stderr)
        print(file=sys.stderr)
        sys.exit(1)

    label = "staged target files" if diff_only else "target files"
    print(f"check_assert_true_detect: OK (no assert(true) in {label})", file=sys.stderr)


if __name__ == "__main__":
    main()
