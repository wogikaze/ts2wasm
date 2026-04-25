#!/usr/bin/env python3
"""Parallel script runner for independent checks.

Runs multiple shell commands in parallel and collects their output.
Exits with non-zero status if any command fails.
"""

import argparse
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import List, Tuple


def run_command(cmd: str) -> Tuple[int, str, str]:
    """Run a single command and return (exit_code, stdout, stderr)."""
    try:
        result = subprocess.run(
            cmd,
            shell=True,
            capture_output=True,
            text=True,
            cwd="/home/wogikaze/ts2wasm",
        )
        return result.returncode, result.stdout, result.stderr
    except Exception as e:
        return 1, "", str(e)


def main() -> int:
    parser = argparse.ArgumentParser(description="Run multiple commands in parallel")
    parser.add_argument("commands", nargs="+", help="Commands to run in parallel")
    args = parser.parse_args()

    if not args.commands:
        print("No commands to run", file=sys.stderr)
        return 0

    print(f"Running {len(args.commands)} commands in parallel...", file=sys.stderr)

    failed = False
    with ThreadPoolExecutor(max_workers=len(args.commands)) as executor:
        future_to_cmd = {executor.submit(run_command, cmd): cmd for cmd in args.commands}
        for future in as_completed(future_to_cmd):
            cmd = future_to_cmd[future]
            try:
                exit_code, stdout, stderr = future.result()
                if exit_code != 0:
                    failed = True
                    print(f"FAILED: {cmd}", file=sys.stderr)
                    if stderr:
                        print(stderr, file=sys.stderr)
                    if stdout:
                        print(stdout, file=sys.stderr)
                else:
                    print(f"OK: {cmd}", file=sys.stderr)
                    if stderr:
                        print(stderr, file=sys.stderr)
            except Exception as e:
                failed = True
                print(f"ERROR running {cmd}: {e}", file=sys.stderr)

    if failed:
        print("One or more commands failed", file=sys.stderr)
        return 1

    print("All commands passed", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
