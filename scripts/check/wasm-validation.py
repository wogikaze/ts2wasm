#!/usr/bin/env python3
"""Build representative fixtures to wasm, then `wasm-tools validate` each binary.

Usage: python scripts/manager.py check-wasm-validation
Optional: TS2WASM_VALIDATE_FIXTURES="f1 f2" (space-separated, repo-root paths)
Dependencies: cargo, wasm-tools
"""

import sys
import subprocess
import shutil
import tempfile
import os
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def main():
    args = sys.argv[1:]
    
    if not shutil.which("wasm-tools"):
        print("check_wasm_validation: wasm-tools is required", file=sys.stderr)
        sys.exit(1)
    
    # Default fixtures
    default_list = [
        "fixtures/basics-hello/hello.ts",
        "fixtures/primitives-control-flow/number.ts",
        "fixtures/arrays-objects/object.ts",
    ]
    
    # Determine fixtures to validate
    if args:
        vfix = args
    elif os.environ.get("TS2WASM_VALIDATE_FIXTURES"):
        vfix = os.environ["TS2WASM_VALIDATE_FIXTURES"].split()
    else:
        vfix = default_list
    
    # Build ts2wasm-cli
    result = subprocess.run(
        ["cargo", "build", "-q", "-p", "ts2wasm-cli"],
        cwd=REPO_ROOT
    )
    if result.returncode != 0:
        print("check_wasm_validation: failed to build ts2wasm-cli", file=sys.stderr)
        sys.exit(1)
    
    ts2wasm_bin = REPO_ROOT / "target/debug/ts2wasm"
    if not ts2wasm_bin.exists():
        print(f"check_wasm_validation: expected binary missing: {ts2wasm_bin}", file=sys.stderr)
        sys.exit(1)
    
    with tempfile.TemporaryDirectory() as tmpd:
        tmpd = Path(tmpd)
        
        for fixture in vfix:
            if not fixture:
                continue
            
            fixture_path = REPO_ROOT / fixture
            if not fixture_path.exists():
                print(f"check_wasm_validation: missing: {fixture}", file=sys.stderr)
                sys.exit(1)
            
            wasm_path = tmpd / "validate.wasm"
            print(f"check_wasm_validation: build {fixture}", file=sys.stderr)
            
            result = subprocess.run(
                [str(ts2wasm_bin), "build", str(fixture_path), "-o", str(wasm_path)],
                capture_output=True,
                text=True,
                cwd=REPO_ROOT
            )
            if result.returncode != 0:
                print(f"check_wasm_validation: build failed: {fixture}", file=sys.stderr)
                if result.stdout:
                    print(result.stdout, file=sys.stderr)
                if result.stderr:
                    print(result.stderr, file=sys.stderr)
                sys.exit(1)
            
            result = subprocess.run(
                ["wasm-tools", "validate", str(wasm_path)],
                capture_output=True,
                text=True,
                cwd=REPO_ROOT
            )
            if result.returncode != 0:
                print(f"check_wasm_validation: validate failed: {fixture}", file=sys.stderr)
                if result.stdout:
                    print(result.stdout, file=sys.stderr)
                if result.stderr:
                    print(result.stderr, file=sys.stderr)
                sys.exit(1)
    
    print("check_wasm_validation: OK", file=sys.stderr)

if __name__ == "__main__":
    main()
