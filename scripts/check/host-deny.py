#!/usr/bin/env python3
"""Standalone fixtures must not import the wasm module name "host" (Node host shim only).
Pure WASI + in-wasm runtime must not need host.* imports in emitted wasm.

Builds with the ts2wasm CLI, prints wasm, and fails if a `(import "host" ...` appears.
Override list with TS2WASM_HOST_FREE_FIXTURES (space-separated paths) if needed.

Usage: python scripts/manager.py check-host-deny
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
        print("check_host_deny: wasm-tools is required", file=sys.stderr)
        sys.exit(1)
    
    # Default fixtures
    default_list = [
        "fixtures/basics-hello/hello.ts",
        "fixtures/primitives-control-flow/number.ts",
        "fixtures/primitives-control-flow/string.ts",
        "fixtures/primitives-control-flow/boolean-if.ts",
        "fixtures/core-semantics/strict-equal.ts",
        "fixtures/arrays-objects/object.ts",
    ]
    
    # Determine fixtures to check
    if args:
        default_fixtures = args
    elif os.environ.get("TS2WASM_HOST_FREE_FIXTURES"):
        default_fixtures = os.environ["TS2WASM_HOST_FREE_FIXTURES"].split()
    else:
        default_fixtures = default_list
    
    if not default_fixtures:
        print("check_host_deny: no fixtures configured", file=sys.stderr)
        sys.exit(1)
    
    # Build ts2wasm-cli
    result = subprocess.run(
        ["cargo", "build", "-q", "-p", "ts2wasm-cli"],
        cwd=REPO_ROOT
    )
    if result.returncode != 0:
        print("check_host_deny: failed to build ts2wasm-cli", file=sys.stderr)
        sys.exit(1)
    
    ts2wasm_bin = REPO_ROOT / "target/debug/ts2wasm"
    if not ts2wasm_bin.exists():
        print(f"check_host_deny: expected binary missing: {ts2wasm_bin}", file=sys.stderr)
        sys.exit(1)
    
    with tempfile.TemporaryDirectory() as tmpd:
        tmpd = Path(tmpd)
        
        for fixture in default_fixtures:
            if not fixture:
                continue
            
            fixture_path = REPO_ROOT / fixture
            if not fixture_path.exists():
                print(f"check_host_deny: missing fixture: {fixture}", file=sys.stderr)
                sys.exit(1)
            
            wasm_name = fixture.replace("/", "_")
            wasm_path = tmpd / f"{wasm_name}.wasm"
            
            print(f"check_host_deny: build {fixture}", file=sys.stderr)
            result = subprocess.run(
                [str(ts2wasm_bin), "build", str(fixture_path), "-o", str(wasm_path)],
                capture_output=True,
                text=True,
                cwd=REPO_ROOT
            )
            if result.returncode != 0:
                print(f"check_host_deny: build failed for {fixture}", file=sys.stderr)
                if result.stdout:
                    print(result.stdout, file=sys.stderr)
                if result.stderr:
                    print(result.stderr, file=sys.stderr)
                sys.exit(1)
            
            result = subprocess.run(
                ["wasm-tools", "print", str(wasm_path)],
                capture_output=True,
                text=True,
                cwd=REPO_ROOT
            )
            if result.returncode != 0:
                print(f"check_host_deny: wasm-tools print failed for {fixture}", file=sys.stderr)
                sys.exit(1)
            
            if '(import "host"' in result.stdout:
                print(f'check_host_deny: disallowed (import "host" ...) in wasm for: {fixture}', file=sys.stderr)
                sys.exit(1)
    
    print("check_host_deny: OK (no host module imports in listed fixtures)", file=sys.stderr)

if __name__ == "__main__":
    main()
