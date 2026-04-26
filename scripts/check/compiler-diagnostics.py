#!/usr/bin/env python3
"""Fail if obvious non-test panic sites appear in production compiler directories.

Policy (incremental): backend/, runtime/, and main.rs must not contain `panic!(`.
Tests and lib.rs monolith may still use panics inside #[cfg(test)]; tighten over time.

Usage: python scripts/manager.py check-compiler-diagnostics

Fails if `panic!(` appears under crates/cli/src/backend, crates/cli/src/runtime, or main.rs.
"""

import sys
import subprocess
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def usage():
    print("Usage:")
    print("  python scripts/manager.py check-compiler-diagnostics")
    print()
    print("Fails if `panic!(` appears under crates/cli/src/backend, crates/cli/src/runtime, or main.rs.")

def main():
    args = sys.argv[1:]
    
    if args and args[0] in ("-h", "--help"):
        usage()
        sys.exit(0)
    
    # Search for panic! in specified directories
    search_paths = [
        "crates/cli/src/backend",
        "crates/cli/src/runtime",
        "crates/cli/src/main.rs"
    ]
    
    hits = 0
    for search_path in search_paths:
        full_path = REPO_ROOT / search_path
        if not full_path.exists():
            continue
        
        if full_path.is_file():
            # Single file
            result = subprocess.run(
                ["git", "grep", "-n", "panic!(", "--", str(full_path)],
                capture_output=True,
                text=True,
                cwd=REPO_ROOT
            )
        else:
            # Directory
            result = subprocess.run(
                ["git", "grep", "-n", "panic!(", "--", str(full_path)],
                capture_output=True,
                text=True,
                cwd=REPO_ROOT
            )
        
        if result.stdout:
            for line in result.stdout.splitlines():
                print(f"check_compiler_diagnostics: {line}", file=sys.stderr)
                hits = 1
    
    if hits != 0:
        sys.exit(1)
    
    print("check_compiler_diagnostics: OK", file=sys.stderr)

if __name__ == "__main__":
    main()
