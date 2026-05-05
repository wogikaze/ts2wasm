#!/usr/bin/env python3
"""Detect trivial true assertions in fixture files and test scripts.

Complex test cases (eval, legacy RegExp, generators, Date, TypedArray,
HTML-like comments, etc.) must not be replaced with a trivial true assertion
as a lazy way to make tests pass.

Scans both fixtures/**/*.ts and scripts/**/*.py for trivial true assertions.

Usage:
  mise run check assert-true        # full scan (both fixtures/ and scripts/)
  python scripts/check/assert-true-detect.py [--diff-only]

Options:
  --diff-only   Only check staged target files (pre-commit mode).
                Checks ALL content in staged files, not just new additions.

Dependencies: python3, rg (ripgrep)
"""

import ast
import io
import re
import subprocess
import sys
import tokenize
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
ASSERT_TRUE_RE = re.compile(r"\bassert\s*\(\s*true\s*\)")


def usage():
    print("Usage:")
    print("  mise run check assert-true")
    print("  python scripts/check/assert-true-detect.py [--diff-only]")
    print()
    print("Full scan: checks fixtures/**/*.ts and scripts/**/*.py for trivial true assertions.")
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


def python_string_values(source):
    """Yield decoded Python string tokens as (line, value)."""
    try:
        tokens = tokenize.generate_tokens(io.StringIO(source).readline)
    except tokenize.TokenError as exc:
        yield exc.args[1][0] if len(exc.args) > 1 and exc.args[1] else 1, source
        return

    for token in tokens:
        if token.type != tokenize.STRING:
            continue
        try:
            value = ast.literal_eval(token.string)
        except (SyntaxError, ValueError):
            continue
        if isinstance(value, str):
            yield token.start[0], value


def scan_python_file(path):
    try:
        source = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        source = path.read_text(encoding="utf-8", errors="replace")

    hits = []
    for line_no, value in python_string_values(source):
        if ASSERT_TRUE_RE.search(value):
            preview = value.strip().splitlines()[0] if value.strip() else "<empty string>"
            hits.append((path, f"{line_no}: string literal contains trivial true assertion: {preview}"))
    return hits


def scan_text_file(path):
    result = subprocess.run(
        ["rg", "-n", ASSERT_TRUE_RE.pattern, str(path)],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        return []
    return [(path, line) for line in result.stdout.strip().splitlines()]


def scan_files(file_paths):
    """Scan files for trivial true assertions. Return list of (file, line) tuples.

    Uses different patterns per file type:
      - .ts: source text scan for trivial true assertion calls with optional whitespace.
      - .py: decoded string-token scan, so replacement strings like JS comments are caught.
    """
    hits = []
    for f in file_paths:
        if not f.exists():
            continue
        if f.suffix == ".py":
            hits.extend(scan_python_file(f))
        else:
            hits.extend(scan_text_file(f))
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
        print("Found trivial true assertion — complex test cases must not be simplified", file=sys.stderr)
        print("to unconditional success. Either implement real assertions or mark the test", file=sys.stderr)
        print("as intentionally trivial with an explicit message.", file=sys.stderr)
        print("", file=sys.stderr)
        for f, line in hits:
            rel = f.relative_to(REPO_ROOT) if isinstance(f, Path) else f
            print(f"  {rel}:{line}", file=sys.stderr)
        print(file=sys.stderr)
        sys.exit(1)

    label = "staged target files" if diff_only else "target files"
    print(f"check_assert_true_detect: OK (no trivial true assertions in {label})", file=sys.stderr)


if __name__ == "__main__":
    main()
