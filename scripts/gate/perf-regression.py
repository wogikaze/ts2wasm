#!/usr/bin/env python3
"""Performance regression gate: compiler_throughput and WASM binary size tracking.
perf_regression gate: monitors compiler throughput (fixtures/sec) and WASM binary size.

Runs the benchmark-tracker against a sample of fixtures and reports any
regressions in compilation time or WASM binary size compared to the previous
benchmark run.

Usage:
  python scripts/gate/perf-regression.py [--sample N] [--skip-build]

Options:
  --sample N      Number of fixtures per directory (default: 5)
  --skip-build    Skip cargo build (use existing binary)

Exit code:
  0 if no regressions detected (or no prior baseline)
  1 if regressions detected
"""

import sys
import subprocess
import json
import os
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
PYTHON_BIN = os.environ.get("PYTHON_BIN", sys.executable)


def main():
    args = []
    sample = "5"
    skip_build = False

    i = 1
    while i < len(sys.argv):
        if sys.argv[i] == "--sample" and i + 1 < len(sys.argv):
            sample = sys.argv[i + 1]
            i += 2
        elif sys.argv[i] == "--skip-build":
            skip_build = True
            i += 1
        elif sys.argv[i] in ("-h", "--help"):
            print(__doc__)
            sys.exit(0)
        else:
            print(f"perf-regression: unknown option: {sys.argv[i]}", file=sys.stderr)
            sys.exit(1)

    cmd = [
        PYTHON_BIN,
        str(REPO_ROOT / "scripts/perf/benchmark-tracker.py"),
        "--sample", sample,
        "--verbose",
    ]
    if skip_build:
        cmd.append("--skip-build")

    print(f"perf-regression: running benchmark-tracker...", file=sys.stderr)
    result = subprocess.run(cmd, cwd=REPO_ROOT, capture_output=True, text=True)

    # Print stderr from benchmark-tracker (the human-readable report)
    if result.stderr:
        print(result.stderr, file=sys.stderr)

    if result.returncode != 0:
        # benchmark-tracker exits 0 always, but check anyway
        print(f"perf-regression: benchmark-tracker failed (exit {result.returncode})", file=sys.stderr)
        sys.exit(1)

    # Parse stdout for the JSON summary
    try:
        summary = json.loads(result.stdout.strip())
    except json.JSONDecodeError:
        print(f"perf-regression: failed to parse benchmark output", file=sys.stderr)
        print(f"stdout: {result.stdout[:500]}", file=sys.stderr)
        sys.exit(1)

    regression_count = summary.get("regression_count", 0)
    if regression_count > 0:
        print(f"perf-regression: FAILED — {regression_count} regression(s) detected", file=sys.stderr)
        sys.exit(1)
    else:
        avg_time = summary.get("avg_compile_time_ms", 0)
        avg_size = summary.get("avg_wasm_size_bytes", 0)
        print(f"perf-regression: OK (avg {avg_time}ms, {avg_size} bytes, {summary.get('fixtures_ok', 0)} fixtures)", file=sys.stderr)
        sys.exit(0)


if __name__ == "__main__":
    main()
