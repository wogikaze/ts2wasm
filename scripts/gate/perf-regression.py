#!/usr/bin/env python3
"""perf_regression: Performance regression gate for compiler throughput and WASM binary size.

Usage:
  python scripts/gate/perf-regression.py [--json]

Collects compilation time and WASM binary size metrics using the
benchmark-tracker, then checks for regressions against historical data.

Exits 0 on success, 1 if regressions detected.
"""

import sys
import json
import subprocess
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.resolve()

def main():
    json_mode = "--json" in sys.argv[1:]

    # Run benchmark-tracker with --json to get clean JSON output
    result = subprocess.run(
        [sys.executable, str(REPO_ROOT / "scripts/perf/benchmark-tracker.py"), "--json"],
        capture_output=True, text=True, cwd=REPO_ROOT,
    )

    if result.returncode != 0:
        print(result.stderr, file=sys.stderr)
        sys.exit(1)

    # Parse the JSON summary from stdout
    try:
        summary = json.loads(result.stdout.strip())
    except json.JSONDecodeError as e:
        print(f"error: failed to parse benchmark output: {e}", file=sys.stderr)
        sys.exit(1)

    if json_mode:
        print(json.dumps(summary, indent=2))
        return

    # Human-readable output
    print(f"  Fixtures:       {summary['fixtures_ok']} ok, {summary['fixtures_skipped']} skipped")
    print(f"  Avg compile:    {summary['avg_compile_time_ms']} ms")
    print(f"  Avg wasm size:  {summary['avg_wasm_size_bytes']} bytes")
    if summary.get("regression_count", 0) > 0:
        print(f"  Regressions:    {summary['regression_count']} detected")
        for r in summary.get("regressions", []):
            print(f"    {r['fixture']} ({r['metric']}): {r['current']} vs {r['previous']} (+{r['pct_increase']}%)")
        sys.exit(1)
    else:
        print(f"  Regressions:    none")
        print("perf-regression: OK")

if __name__ == "__main__":
    main()
