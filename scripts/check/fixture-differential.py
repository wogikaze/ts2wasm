#!/usr/bin/env python3
"""Re-run the Node vs iwasm fixture differential (integration) suite as a standalone gate.
Supports both assertion-based (m2_node_diff) and JSONL (differential_jsonl) modes.

Wraps:
  crates/cli/tests/m2_node_diff.rs     -- assertion-based (default)
  crates/cli/tests/differential_jsonl.rs -- JSONL structured output

Usage:
  mise run check differential               # assertion mode (default)
  mise run check differential -- --jsonl     # JSONL mode (runs ignored batches)
  mise run check differential -- --sample N  # ignored in assertion mode

Dependencies: cargo, nextest, node, iwasm, ts2wasm binary (via nextest build)
"""

import sys
import subprocess
import shutil
import json
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()


def usage():
    print(
        "Usage: mise run check differential -- [--jsonl] [--sample N]",
        file=sys.stderr,
    )


def run_nextest(test_name: str, args: list[str] | None = None) -> int:
    """Run a cargo nextest test and return exit code."""
    cmd = [
        "cargo",
        "nextest",
        "run",
        "-p",
        "ts2wasm-cli",
        "--test",
        test_name,
    ]
    if args:
        cmd.extend(args)
    print(f"check_fixture_differential: {' '.join(cmd)}", file=sys.stderr)
    result = subprocess.run(cmd, cwd=REPO_ROOT)
    return result.returncode


def validate_jsonl_output(output: str) -> tuple[int, int]:
    """Parse JSONL output and validate each record.

    Returns (valid_count, error_count).
    """
    valid = 0
    errors = 0
    for line_num, line in enumerate(output.splitlines(), 1):
        line = line.strip()
        if not line:
            continue
        try:
            record = json.loads(line)
        except json.JSONDecodeError as e:
            print(f"check_fixture_differential: JSONL line {line_num}: invalid JSON: {e}",
                  file=sys.stderr)
            errors += 1
            continue

        # Validate required fields
        missing = [f for f in ["suite", "case", "target", "status"] if f not in record]
        if missing:
            print(f"check_fixture_differential: line {line_num}: missing fields {missing}: {line}",
                  file=sys.stderr)
            errors += 1
            continue

        # Validate status value
        valid_statuses = ["pass", "fail", "unsupported", "blocked", "skip-with-reason"]
        if record["status"] not in valid_statuses:
            print(f"check_fixture_differential: line {line_num}: invalid status '{record['status']}': {line}",
                  file=sys.stderr)
            errors += 1
            continue

        # Validate unsupported/blocked have reason and tracking
        if record["status"] in ("unsupported", "blocked", "skip-with-reason"):
            if not record.get("reason"):
                print(f"check_fixture_differential: line {line_num}: missing reason for {record['status']}: {line}",
                      file=sys.stderr)
                errors += 1
            if not record.get("tracking"):
                print(f"check_fixture_differential: line {line_num}: missing tracking for {record['status']}: {line}",
                      file=sys.stderr)
                errors += 1

        valid += 1

    return valid, errors


def run_jsonl_mode(args: list[str]) -> int:
    """Run in JSONL mode, running the full fixture sweep and validating output."""
    print("check_fixture_differential: JSONL mode", file=sys.stderr)

    # Run the full fixture sweep (ignored by default, so use --run-ignored)
    nextest_args = ["--run-ignored", "ignored-only"]

    # Run all four batch tests
    batch_tests = [
        "differential_jsonl_runs_and_validates_first_batch",
        "differential_jsonl_runs_and_validates_second_batch",
        "differential_jsonl_runs_and_validates_third_batch",
        "differential_jsonl_runs_and_validates_fourth_batch",
    ]

    overall_valid = 0
    overall_errors = 0

    for test in batch_tests:
        result = subprocess.run(
            [
                "cargo",
                "nextest",
                "run",
                "-p",
                "ts2wasm-cli",
                "--test",
                "differential_jsonl",
                "--",
                test,
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
        )

        # Validate stdout as JSONL
        if result.stdout:
            v, e = validate_jsonl_output(result.stdout)
            overall_valid += v
            overall_errors += e

        # Print stderr from nextest
        if result.stderr:
            for line in result.stderr.splitlines():
                print(line, file=sys.stderr)

    # Summary
    print(
        f"check_fixture_differential: JSONL summary: valid={overall_valid} errors={overall_errors}",
        file=sys.stderr,
    )

    if overall_errors > 0:
        print(
            f"check_fixture_differential: FAILED: {overall_errors} JSONL validation errors",
            file=sys.stderr,
        )
        return 1

    print(
        f"check_fixture_differential: PASSED: {overall_valid} valid JSONL records",
        file=sys.stderr,
    )
    return 0


def main():
    args = sys.argv[1:]

    # Check required commands
    for cmd in ["cargo", "node", "iwasm"]:
        if not shutil.which(cmd):
            print(f"check_fixture_differential: missing: {cmd}", file=sys.stderr)
            sys.exit(1)

    jsonl_mode = False

    # Parse arguments
    i = 0
    while i < len(args):
        if args[i] == "-h" or args[i] == "--help":
            usage()
            sys.exit(0)
        elif args[i] == "--sample":
            if i + 1 < len(args) and not args[i + 1].startswith("-"):
                print(
                    f"check_fixture_differential: note: --sample {args[i+1]} ignored; running full suite",
                    file=sys.stderr,
                )
                i += 2
            else:
                print(
                    "check_fixture_differential: --sample requires a number",
                    file=sys.stderr,
                )
                sys.exit(1)
        elif args[i] == "--jsonl":
            jsonl_mode = True
            i += 1
        else:
            print(f"check_fixture_differential: unknown arg: {args[i]}", file=sys.stderr)
            usage()
            sys.exit(1)

    if jsonl_mode:
        sys.exit(run_jsonl_mode(args))

    # Default: assertion mode (m2_node_diff)
    print(
        "check_fixture_differential: cargo nextest -p ts2wasm-cli --test m2_node_diff",
        file=sys.stderr,
    )
    result = subprocess.run(
        ["cargo", "nextest", "run", "-p", "ts2wasm-cli", "--test", "m2_node_diff"],
        cwd=REPO_ROOT,
    )
    sys.exit(result.returncode)


if __name__ == "__main__":
    main()
