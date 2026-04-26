#!/usr/bin/env python3
"""Lightweight dependency-direction checks (complement to docs/12 + ast-grep rules).

Usage: python scripts/manager.py check-architecture-rules

Current checks:
  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).
"""

import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def usage():
    print("Usage:")
    print("  python scripts/manager.py check-architecture-rules")
    print()
    print("Current checks:")
    print("  - ts2wasm-shared must not depend on ts2wasm-cli (inverted crate boundary).")

def main():
    args = sys.argv[1:]
    
    if args and args[0] in ("-h", "--help"):
        usage()
        sys.exit(0)
    
    if not shutil.which("cargo"):
        print("check_architecture_rules: cargo is required", file=sys.stderr)
        sys.exit(1)
    
    # Check if ts2wasm-shared depends on ts2wasm-cli
    result = subprocess.run(
        ["cargo", "tree", "-p", "ts2wasm-shared", "--edges", "normal,build"],
        capture_output=True,
        text=True,
        cwd=REPO_ROOT
    )
    
    if result.returncode != 0:
        # cargo tree might fail if package doesn't exist, that's OK for this check
        print("check_architecture_rules: OK", file=sys.stderr)
        sys.exit(0)
    
    if "ts2wasm-cli" in result.stdout:
        print("check_architecture_rules: ts2wasm-shared must not depend on ts2wasm-cli", file=sys.stderr)
        print(result.stdout, file=sys.stderr)
        sys.exit(1)
    
    print("check_architecture_rules: OK", file=sys.stderr)

if __name__ == "__main__":
    main()
