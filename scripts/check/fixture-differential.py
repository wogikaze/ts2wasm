#!/usr/bin/env python3
"""Re-run the Node vs iwasm fixture differential (integration) suite as a standalone gate.
Wraps: crates/cli/tests/m2_node_diff.rs

Usage: mise run check differential
Dependencies: cargo, nextest, node, ts2wasm binary (via nextest build)
"""

import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def usage():
    print("Usage: mise run check differential -- [--sample N]", file=sys.stderr)

def main():
    args = sys.argv[1:]
    
    # Check required commands
    for cmd in ["cargo", "node", "iwasm"]:
        if not shutil.which(cmd):
            print(f"check_fixture_differential: missing: {cmd}", file=sys.stderr)
            sys.exit(1)
    
    # Parse arguments
    i = 0
    while i < len(args):
        if args[i] == "-h" or args[i] == "--help":
            usage()
            sys.exit(0)
        elif args[i] == "--sample":
            if i + 1 < len(args) and not args[i + 1].startswith("-"):
                print(f"check_fixture_differential: note: --sample {args[i+1]} ignored; running full m2_node_diff", file=sys.stderr)
                i += 2
            else:
                print("check_fixture_differential: --sample requires a number", file=sys.stderr)
                sys.exit(1)
        else:
            print(f"check_fixture_differential: unknown arg: {args[i]}", file=sys.stderr)
            sys.exit(1)
    
    print("check_fixture_differential: cargo nextest -p ts2wasm-cli --test m2_node_diff", file=sys.stderr)
    result = subprocess.run(
        ["cargo", "nextest", "run", "-p", "ts2wasm-cli", "--test", "m2_node_diff"],
        cwd=REPO_ROOT
    )
    sys.exit(result.returncode)

if __name__ == "__main__":
    main()
