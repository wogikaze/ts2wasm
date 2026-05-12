#!/usr/bin/env python3
"""Flaky test detection tool: run a command repeatedly and compare outputs.

Repeatedly executes a test command, collects the stdout from each run, and
reports any differences across runs to help identify non-deterministic
(flaky) test results.

Works best with commands that produce JSONL output (one JSON object per
line), where structured comparison per test-case is performed. Falls back to
line-by-line text comparison for non-JSONL output.

Usage:
  python3 scripts/check/flaky-detect.py
  python3 scripts/check/flaky-detect.py --runs 10
  python3 scripts/check/flaky-detect.py --command "python3 scripts/check/fixture-differential.py --smoke"
  python3 scripts/check/flaky-detect.py --help
"""

import argparse
import json
import subprocess
import sys
import time
from collections import defaultdict
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
DEFAULT_COMMAND = (
    f"python3 {REPO_ROOT / 'scripts' / 'check' / 'fixture-differential.py'} --smoke"
)


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Detect flaky tests by running a command multiple times and comparing outputs.",
    )
    parser.add_argument(
        "--runs",
        type=int,
        default=5,
        help="Number of times to repeat the command (default: 5).",
    )
    parser.add_argument(
        "--command",
        type=str,
        default=DEFAULT_COMMAND,
        help=(
            "Command to run repeatedly. "
            "Default: python3 scripts/check/fixture-differential.py --smoke"
        ),
    )
    parser.add_argument(
        "--verbose",
        action="store_true",
        help="Print full stdout/stderr per run for debugging.",
    )
    parser.add_argument(
        "--delay",
        type=float,
        default=0.0,
        help="Delay in seconds between runs (default: 0).",
    )
    return parser.parse_args(argv)


def run_command(cmd: str, verbose: bool = False) -> tuple[str, str, int]:
    """Execute a shell command and return (stdout, stderr, returncode)."""
    proc = subprocess.run(
        cmd,
        shell=True,
        capture_output=True,
        text=True,
        timeout=300,
        cwd=REPO_ROOT,
    )
    if verbose:
        print(f"\n--- stdout ---\n{proc.stdout}", file=sys.stderr)
        if proc.stderr:
            print(f"--- stderr ---\n{proc.stderr}", file=sys.stderr)
        print(f"--- returncode: {proc.returncode} ---\n", file=sys.stderr)
    return proc.stdout, proc.stderr, proc.returncode


def try_parse_jsonl(text: str) -> list[dict] | None:
    """Try to parse text as JSONL. Returns list of dicts, or None on failure."""
    lines = [line.strip() for line in text.splitlines() if line.strip()]
    if not lines:
        return None
    records: list[dict] = []
    for line in lines:
        try:
            val = json.loads(line)
        except json.JSONDecodeError:
            return None
        if not isinstance(val, dict):
            return None
        records.append(val)
    return records


def key_from_jsonl(record: dict) -> str:
    """Extract a stable key from a JSONL record for cross-run comparison."""
    parts = []
    for field in ("suite", "case", "id", "name", "test", "path"):
        if field in record:
            parts.append(str(record[field]))
            break
    for field in ("status", "result", "outcome", "pass"):
        if field in record:
            parts.append(str(record[field]))
            break
    return "|".join(parts) if parts else json.dumps(record, sort_keys=True)


def compare_jsonl_outputs(runs: list[list[dict]]) -> list[dict]:
    """Compare JSONL records across runs and report differences.

    Returns a list of difference records, each containing:
      - test_key: the identifying key for the test
      - run_results: dict mapping run index to status string
      - is_flaky: True if results differ between runs
    """
    # Build a map: test_key -> {run_idx -> status}
    all_tests: dict[str, dict[int, str]] = defaultdict(dict)
    for run_idx, records in enumerate(runs):
        for rec in records:
            key = key_from_jsonl(rec)
            all_tests[key][run_idx] = rec.get("status", rec.get("result", "?"))

    # Find tests where results vary between runs
    flaky: list[dict] = []
    for test_key, run_results in sorted(all_tests.items()):
        unique_results = set(run_results.values())
        if len(unique_results) > 1:
            flaky.append({
                "test_key": test_key,
                "run_results": {str(k): v for k, v in sorted(run_results.items())},
                "is_flaky": True,
            })

    return flaky


def compare_text_outputs(runs: list[str]) -> list[dict]:
    """Compare raw text outputs across runs and report differences."""
    differences: list[dict] = []
    baselines = {0: runs[0]} if runs else {}
    for run_idx in range(1, len(runs)):
        current = runs[run_idx]
        baseline = baselines.get(0, "")
        if current != baseline:
            differences.append({
                "run_pair": f"run-0 vs run-{run_idx}",
                "is_flaky": True,
                "note": "Full output differs between run 0 and this run.",
            })
    return differences


def main() -> None:
    args = parse_args()

    if args.runs < 2:
        print("error: --runs must be at least 2 for meaningful comparison.", file=sys.stderr)
        sys.exit(0)

    cmd = args.command
    n = args.runs
    print(f"Running command {n} times: {cmd}", file=sys.stderr)
    print(f"  (each run has a 300-second timeout)", file=sys.stderr)
    print(file=sys.stderr)

    # Collect stdout from each run
    all_stdout: list[str] = []
    all_returncodes: list[int] = []

    for i in range(n):
        print(f"  Run {i + 1}/{n}...", file=sys.stderr, end=" ")
        sys.stderr.flush()
        try:
            stdout, stderr, rc = run_command(cmd, verbose=args.verbose)
            all_stdout.append(stdout)
            all_returncodes.append(rc)
            print(f"done (exit code {rc})", file=sys.stderr)
        except subprocess.TimeoutExpired:
            print("TIMEOUT", file=sys.stderr)
            all_stdout.append("")
            all_returncodes.append(-1)
        except Exception as e:
            print(f"ERROR: {e}", file=sys.stderr)
            all_stdout.append("")
            all_returncodes.append(-1)

        if args.delay > 0 and i < n - 1:
            time.sleep(args.delay)

    print(file=sys.stderr)

    # Summarize return codes
    unique_rcs = set(all_returncodes)
    if len(unique_rcs) > 1:
        print(f"Exit codes varied across runs: {dict(enumerate(all_returncodes))}", file=sys.stderr)
        print(file=sys.stderr)

    # Compare outputs
    # Try JSONL parsing first; fall back to text comparison
    parsed = [try_parse_jsonl(stdout) for stdout in all_stdout]

    if all(p is not None for p in parsed):
        flaky = compare_jsonl_outputs(parsed)  # type: ignore[arg-type]
        if flaky:
            print("=== FLAKY TESTS DETECTED ===")
            print()
            for item in flaky:
                print(f"  Test: {item['test_key']}")
                for run_idx, status in item["run_results"].items():
                    print(f"    Run {run_idx}: {status}")
                print()
            print(
                f"Summary: {len(flaky)} test(s) produced different results across {n} run(s).",
                file=sys.stderr,
            )
        else:
            print(f"All {n} runs produced identical JSONL results. No flaky tests detected.", file=sys.stderr)
    else:
        # Fall back to text comparison
        if len(set(all_stdout)) == 1:
            print(f"All {n} runs produced identical text output. No flaky tests detected.", file=sys.stderr)
        else:
            diffs = compare_text_outputs(all_stdout)
            print("=== OUTPUT DIFFERS BETWEEN RUNS ===")
            for d in diffs:
                print(f"  {d['run_pair']}: {d.get('note', '')}")
            print()
            print(
                f"Summary: output differed across {n} run(s). "
                "Review the command output manually to identify specific flaky tests.",
                file=sys.stderr,
            )

    # Always exit 0 (info-only reporting)
    sys.exit(0)


if __name__ == "__main__":
    main()
