#!/usr/bin/env python3
"""Standard local gate: fmt + issue queue + architecture + coverage matrix + nextest (optional).

Usage:
  mise run gate [-- --skip-nextest]
  mise run gate-fast

Environment:
  TS2WASM_FAST_GATE_SKIP_NEXTEST=1  Same as --skip-nextest (for pre-push).

Dependencies: cargo, python3 (see nested scripts for cargo-nextest, jq, etc.)
"""

import os
import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
PYTHON_BIN = os.environ.get("PYTHON_BIN", sys.executable)

def usage():
    print("Usage:")
    print("  mise run gate [-- --skip-nextest]")
    print("  mise run gate-fast")
    print()
    print("Runs:")
    print("  - cargo fmt --all --check")
    print("  - python scripts/check/tracking-consistency.py")
    print("  - python scripts/check/architecture-rules.py")
    print("  - cargo nextest run (unless --skip-nextest)")
    print()
    print("Options:")
    print("  --skip-nextest   Skip cargo nextest (faster; use in pre-push with targeted tests).")

def run(cmd, cwd=REPO_ROOT):
    """Run a command and exit if it fails."""
    print(f"gate: {' '.join(cmd)}", file=sys.stderr)
    result = subprocess.run(cmd, cwd=cwd)
    if result.returncode != 0:
        sys.exit(result.returncode)

def main():
    skip_nextest = os.environ.get("TS2WASM_FAST_GATE_SKIP_NEXTEST", "0") == "1"
    
    args = sys.argv[1:]
    while args:
        if args[0] == "--skip-nextest":
            skip_nextest = True
            args.pop(0)
        elif args[0] in ("-h", "--help"):
            usage()
            sys.exit(0)
        else:
            print(f"gate: unknown option: {args[0]}", file=sys.stderr)
            usage()
            sys.exit(1)
    
    # Check for required commands
    for cmd in ["python3", "python"]:
        if shutil.which(cmd):
            break
    else:
        print("gate: missing required command: python/python3", file=sys.stderr)
        sys.exit(1)

    if not shutil.which("cargo"):
        print("gate: missing required command: cargo", file=sys.stderr)
        sys.exit(1)
    
    # Run checks
    run(["cargo", "fmt", "--all", "--check"])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/tracking-consistency.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/assert-true-detect.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/architecture-rules.py")])
    
    if not skip_nextest:
        run(["cargo", "nextest", "run"])
    else:
        print("gate: skipping cargo nextest (--skip-nextest)", file=sys.stderr)
    
    print("gate: OK", file=sys.stderr)

if __name__ == "__main__":
    main()
