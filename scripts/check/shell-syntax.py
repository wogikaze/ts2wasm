#!/usr/bin/env python3
"""Validate shell scripts before running coverage/test workflows.

Usage: python scripts/manager.py check-scripts

Note: `bash -n` is syntax-only. It does not prove runtime behavior.
After editing a script, also run a representative command.
"""

import sys
import subprocess
import platform
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def log(msg):
    print(f"check_shell_syntax: {msg}", file=sys.stderr)

def main():
    # On Windows, skip bash syntax check unless bash is available
    if platform.system() == "Windows":
        if not shutil.which("bash"):
            log("Skipping bash syntax check on Windows (bash not available)")
            sys.exit(0)
    
    log("Running bash -n on scripts/**/*.sh and scripts/manager")
    
    # Find all .sh files
    script_dirs = [
        "scripts",
        "scripts/check",
        "scripts/gate",
        "scripts/gen",
        "scripts/run",
        "scripts/report",
        "scripts/perf",
        "scripts/dev",
        "scripts/lib",
    ]
    
    for dir_name in script_dirs:
        dir_path = REPO_ROOT / dir_name
        if not dir_path.exists():
            continue
        
        for script_file in dir_path.glob("*.sh"):
            if not script_file.is_file():
                continue
            
            result = subprocess.run(
                ["bash", "-n", str(script_file)],
                capture_output=True,
                cwd=REPO_ROOT
            )
            if result.returncode != 0:
                print(f"Syntax error in {script_file}:", file=sys.stderr)
                print(result.stderr.decode('utf-8'), file=sys.stderr)
                sys.exit(1)
            
            log(f"OK: {script_file}")
    
    # Check scripts/manager (bash version if it exists)
    manager_bash = REPO_ROOT / "scripts/manager"
    if manager_bash.exists():
        result = subprocess.run(
            ["bash", "-n", str(manager_bash)],
            capture_output=True,
            cwd=REPO_ROOT
        )
        if result.returncode != 0:
            print(f"Syntax error in scripts/manager:", file=sys.stderr)
            print(result.stderr.decode('utf-8'), file=sys.stderr)
            sys.exit(1)
        log("OK: scripts/manager")
    
    log("All shell syntax checks passed")

if __name__ == "__main__":
    import shutil
    main()
