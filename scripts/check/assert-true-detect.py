#!/usr/bin/env python3
"""Detect fixture files that have been gutted and replaced with trivial assert(true).

Complex test cases (eval, legacy RegExp, generators, Date, TypedArray,
HTML-like comments, etc.) must not be replaced with assert(true) or assert(true, ...)
as a lazy way to make tests pass.

This also catches accidentally-committed assert(true) in fixture files.

Usage:
  mise run check assert-true
  python scripts/check/assert-true-detect.py [--diff-only]

Options:
  --diff-only   Only check files listed in `git diff --cached --name-only`
                (useful in pre-commit hook)

Dependencies: python3, rg (ripgrep)
"""

import sys
import subprocess
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
FIXTURES_DIR = REPO_ROOT / "fixtures"


def usage():
    print("Usage:")
    print("  mise run check assert-true")
    print("  python scripts/check/assert-true-detect.py [--diff-only]")
    print()
    print("Scans fixture files for trivial assert(true) replacements.")


def get_staged_fixture_files():
    """Return list of staged .ts files under fixtures/."""
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
        if line.startswith("fixtures/") and line.endswith(".ts"):
            paths.append(REPO_ROOT / line)
    return paths


def find_all_fixture_files():
    """Return list of all .ts files under fixtures/."""
    return sorted(FIXTURES_DIR.rglob("*.ts"))


def scan_files(files):
    """Scan files for assert(true) patterns. Return list of (file, line) tuples."""
    hits = []
    for f in files:
        if not f.exists():
            continue
        result = subprocess.run(
            ["rg", "-n", r"assert\(true\)", str(f)],
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

    if not FIXTURES_DIR.exists():
        print("check_assert_true_detect: fixtures/ directory not found", file=sys.stderr)
        sys.exit(1)

    if diff_only:
        files = get_staged_fixture_files()
        if not files:
            print("check_assert_true_detect: no staged fixture files — skipping", file=sys.stderr)
            sys.exit(0)
    else:
        files = find_all_fixture_files()

    hits = scan_files(files)
    if hits:
        print("check_assert_true_detect: FAIL", file=sys.stderr)
        print("", file=sys.stderr)
        print("Found assert(true) in fixture files — complex test cases must not be", file=sys.stderr)
        print("simplified to assert(true). Either implement real assertions or mark", file=sys.stderr)
        print("the test as intentionally trivial (e.g. assert(true, 'intentionally empty')).", file=sys.stderr)
        print("", file=sys.stderr)
        for f, line in hits:
            rel = f.relative_to(REPO_ROOT)
            print(f"  {rel}:{line}", file=sys.stderr)
        print(file=sys.stderr)
        sys.exit(1)

    print("check_assert_true_detect: OK (no assert(true) in fixtures)", file=sys.stderr)


if __name__ == "__main__":
    main()
