#!/usr/bin/env python3
"""Detect trivial assert(true) replacements in fixture files and test scripts.

Complex test cases (eval, legacy RegExp, generators, Date, TypedArray,
HTML-like comments, etc.) must not be replaced with assert(true) as a lazy
way to make tests pass.

Scans:
  - fixtures/**/*.ts should never contain assert(true) (full scan / --diff-only)
  - scripts/**/*.py is checked ONLY via --diff-only (detects new additions)

Usage:
  mise run check assert-true
  python scripts/check/assert-true-detect.py [--diff-only]

Options:
  --diff-only   Only check staged additions (pre-commit mode).
                Uses git diff --cached to find newly added assert(true) lines.
                Works for both .ts fixtures and .py scripts.

Dependencies: python3, rg (ripgrep)
"""

import sys
import subprocess
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
FIXTURES_DIR = REPO_ROOT / "fixtures"

# File patterns to check in --diff-only mode
DIFF_TARGET_PATTERNS = (
    "fixtures/*.ts",
    "fixtures/**/*.ts",
    "scripts/**/*.py",
)


def usage():
    print("Usage:")
    print("  mise run check assert-true")
    print("  python scripts/check/assert-true-detect.py [--diff-only]")
    print()
    print("Full scan: checks fixtures/**/*.ts (files must never contain assert(true)).")
    print("--diff-only: checks staged additions in fixtures/ and scripts/ for new assert(true).")


def is_target_file(rel_path):
    """Check if a file path should be scanned."""
    return (
        rel_path.startswith("fixtures/") and rel_path.endswith(".ts")
    ) or (
        rel_path.startswith("scripts/") and rel_path.endswith(".py")
    )


def get_staged_new_assert_true():
    """Return list of (file, line) for newly added assert(true) in staged files.

    Uses git diff --cached to extract only lines being ADDED (prefixed with +).
    This naturally skips existing assert(true) instances in scripts.
    """
    result = subprocess.run(
        ["git", "diff", "--cached", "--name-only", "--diff-filter=ACM"],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        print("check_assert_true_detect: git diff failed", file=sys.stderr)
        sys.exit(1)

    staged_files = [line.strip() for line in result.stdout.strip().splitlines() if line.strip()]
    target_files = [f for f in staged_files if is_target_file(f)]

    if not target_files:
        return []

    hits = []
    for rel_path in target_files:
        abs_path = REPO_ROOT / rel_path
        if not abs_path.exists():
            continue

        # Get only the added lines from the staged diff
        diff_result = subprocess.run(
            ["git", "diff", "--cached", "-U0", "--", rel_path],
            capture_output=True, text=True, cwd=REPO_ROOT,
        )
        if diff_result.returncode != 0:
            continue

        for line in diff_result.stdout.splitlines():
            # Added lines start with '+' (but not '+++' which is the file header)
            if line.startswith("+") and not line.startswith("+++"):
                added_content = line[1:]  # strip the leading '+'
                if "assert(true)" in added_content:
                    hits.append((rel_path, added_content.strip()))

    return hits


def find_all_fixture_files():
    """Return list of all .ts files under fixtures/."""
    return sorted(FIXTURES_DIR.rglob("*.ts"))


def scan_fixtures_for_assert_true(files):
    """Scan fixture files for assert(true). Return list of (path, line) tuples."""
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


def report_hits(hits, label):
    """Print hits and return True if any found."""
    if not hits:
        return False
    for path, line in hits:
        print(f"  {path}:{line}", file=sys.stderr)
    return True


def main():
    args = sys.argv[1:]
    diff_only = "--diff-only" in args
    if "--help" in args or "-h" in args:
        usage()
        sys.exit(0)

    if diff_only:
        # Pre-commit mode: detect NEW assert(true) additions in staged files
        hits = get_staged_new_assert_true()
        if hits:
            print("check_assert_true_detect: FAIL", file=sys.stderr)
            print("", file=sys.stderr)
            print("New assert(true) detected in staged changes.", file=sys.stderr)
            print("Complex test cases must not be simplified to assert(true).", file=sys.stderr)
            print("Either implement real assertions or mark the test as intentionally", file=sys.stderr)
            print("trivial (e.g. assert(true, 'intentionally empty')).", file=sys.stderr)
            print("", file=sys.stderr)
            for path, line in hits:
                print(f"  {path}: {line}", file=sys.stderr)
            print(file=sys.stderr)
            sys.exit(1)
        print("check_assert_true_detect: OK (no new assert(true) in staged files)", file=sys.stderr)
        sys.exit(0)

    # Full scan mode: fixtures/ must never contain assert(true)
    if not FIXTURES_DIR.exists():
        print("check_assert_true_detect: fixtures/ directory not found", file=sys.stderr)
        sys.exit(1)

    files = find_all_fixture_files()
    hits = scan_fixtures_for_assert_true(files)
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
