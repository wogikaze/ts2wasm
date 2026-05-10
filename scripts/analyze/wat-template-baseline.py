#!/usr/bin/env python3
"""Analyze raw WAT template functions in the backend-wasm runtime.

Scans runtime_*.rs files for WAT template functions and produces a baseline
report: count of template functions, total lines, and per-file breakdown.

Usage:
  python3 scripts/analyze/wat-template-baseline.py

Output (stderr):
  WAT template baseline: N template functions, M lines across F files

Exit code:
  0 on success, 1 on error
"""
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
BACKEND_DIR = REPO_ROOT / "crates" / "backend-wasm" / "src"


def main():
    runtime_files = sorted(BACKEND_DIR.glob("runtime_*.rs"))
    if not runtime_files:
        print("No runtime_*.rs files found", file=sys.stderr)
        return 1

    total_template_fns = 0
    total_lines = 0
    file_breakdown = []

    for fpath in runtime_files:
        with open(fpath) as f:
            content = f.read()
        lines = content.splitlines()
        n_lines = len(lines)

        # Count WAT functions defined inline in r#"..."# blocks
        wat_func_count = len(re.findall(
            r'r#"\s*\n\s*\(func\s+\$(\w+)',
            content
        ))

        # Count emit_ method definitions (Rust fns that produce WAT)
        emit_methods = len(re.findall(
            r'fn\s+(emit_\w+)\s*\(', content
        ))

        total = wat_func_count + emit_methods
        total_template_fns += total
        total_lines += n_lines
        file_breakdown.append((fpath.name, total, n_lines, wat_func_count, emit_methods))

    # Print breakdown to stdout
    print(f"{'File':45s} {'Templates':>10s} {'Lines':>8s} {'WAT funcs':>10s} {'emit_ fns':>10s}")
    print("-" * 85)
    for name, total, n_lines, wat, emit in file_breakdown:
        print(f"{name:45s} {total:>10d} {n_lines:>8d} {wat:>10d} {emit:>10d}")

    # Overall summary to stderr
    summary = (
        f"WAT template baseline: {total_template_fns} template functions, "
        f"{total_lines} lines across {len(runtime_files)} files"
    )
    print(summary, file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
