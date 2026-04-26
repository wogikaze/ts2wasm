#!/usr/bin/env python3
"""Install git hooks for ts2wasm (Windows version)

Usage: python scripts/manager.py install-hooks

Sets core.hooksPath to .githooks to enable pre-commit and pre-push hooks.
"""

import sys
import subprocess
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def main():
    # Set core.hooksPath
    result = subprocess.run(
        ["git", "config", "core.hooksPath", ".githooks"],
        cwd=REPO_ROOT
    )
    if result.returncode != 0:
        print(f"Failed to set core.hooksPath: {result.stderr}", file=sys.stderr)
        sys.exit(1)
    
    # On Unix, we would chmod +x the hooks, but on Windows this is not needed
    # Git on Windows will execute the hooks using the shebang line
    
    print("Installed git hooks path: .githooks")
    print("Active hooks: .githooks/pre-commit, .githooks/pre-push")

if __name__ == "__main__":
    main()
