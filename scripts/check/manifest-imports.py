#!/usr/bin/env python3
"""Compare capability manifest "imports" with wasm module imports (wasm-tools print).

Usage:
  mise run check manifest -- [--fixture PATH.ts]
  mise run check manifest -- PATH.ts

Default fixture: fixtures/basics-hello/hello.ts

Fails if manifest import (module,name) pairs differ from wasm import section.
"""

import sys
import subprocess
import json
import tempfile
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def usage():
    print("Usage:")
    print("  mise run check manifest -- [--fixture PATH.ts]")
    print("  mise run check manifest -- PATH.ts")
    print()
    print("A single path ending in .ts may be given without --fixture.")
    print("Default fixture: fixtures/basics-hello/hello.ts")
    print()
    print("Fails if manifest import (module,name) pairs differ from wasm import section.")

def main():
    args = sys.argv[1:]
    
    fixture = "fixtures/basics-hello/hello.ts"
    
    # Parse arguments
    i = 0
    while i < len(args):
        if args[i] == "-h" or args[i] == "--help":
            usage()
            sys.exit(0)
        elif args[i].endswith(".ts") and not args[i].startswith("--"):
            fixture = args[i]
            i += 1
        elif args[i] == "--fixture":
            if i + 1 >= len(args):
                print("error: --fixture requires a path", file=sys.stderr)
                sys.exit(1)
            fixture = args[i + 1]
            i += 2
        else:
            print(f"unknown option: {args[i]}", file=sys.stderr)
            usage()
            sys.exit(1)
    
    # Check required commands
    for cmd in ["cargo", "wasm-tools"]:
        if not shutil.which(cmd):
            print(f"check_manifest_imports: missing required command: {cmd}", file=sys.stderr)
            sys.exit(1)
    
    fixture_path = REPO_ROOT / fixture
    if not fixture_path.exists():
        print(f"check_manifest_imports: fixture not found: {fixture}", file=sys.stderr)
        sys.exit(1)
    
    # Create temp directory
    with tempfile.TemporaryDirectory() as tmpd:
        tmpd = Path(tmpd)
        wasm_path = tmpd / "check.wasm"
        manifest_path = tmpd / "manifest.json"
        
        print(f"check_manifest_imports: build {fixture}", file=sys.stderr)
        result = subprocess.run(
            ["cargo", "run", "-q", "-p", "ts2wasm-cli", "--", "build", str(fixture_path), 
             "-o", str(wasm_path), "--emit-manifest", str(manifest_path)],
            cwd=REPO_ROOT
        )
        if result.returncode != 0:
            sys.exit(result.returncode)
        
        # Extract manifest imports
        with open(manifest_path) as f:
            manifest = json.load(f)
        
        manifest_imports = set()
        
        # Extract WASI imports
        if manifest.get("wasi", {}).get("stdout"):
            manifest_imports.add(("wasi_snapshot_preview1", "fd_write"))
        if manifest.get("wasi", {}).get("stdin"):
            manifest_imports.add(("wasi_snapshot_preview1", "fd_read"))
        if manifest.get("wasi", {}).get("stderr"):
            manifest_imports.add(("wasi_snapshot_preview1", "fd_write"))
        
        # Extract Node host imports
        for imp in manifest.get("node_host", {}).get("imports", []):
            parts = imp.split(".")
            if len(parts) >= 2:
                manifest_imports.add((parts[0], parts[1]))
        
        # Extract wasm imports
        result = subprocess.run(
            ["wasm-tools", "print", str(wasm_path)],
            capture_output=True,
            text=True,
            cwd=REPO_ROOT
        )
        if result.returncode != 0:
            print(f"check_manifest_imports: wasm-tools print failed", file=sys.stderr)
            sys.exit(1)
        
        wasm_imports = set()
        for line in result.stdout.splitlines():
            import re
            match = re.search(r'\(import "([^"]*)" "([^"]*)"', line)
            if match:
                wasm_imports.add((match.group(1), match.group(2)))
        
        # Compare
        if manifest_imports != wasm_imports:
            print("check_manifest_imports: manifest imports != wasm imports", file=sys.stderr)
            print("--- manifest (module<tab>name) ---", file=sys.stderr)
            for module, name in sorted(manifest_imports):
                print(f"{module}\t{name}", file=sys.stderr)
            print("--- wasm ---", file=sys.stderr)
            for module, name in sorted(wasm_imports):
                print(f"{module}\t{name}", file=sys.stderr)
            sys.exit(1)
        
        print(f"check_manifest_imports: OK ({fixture})", file=sys.stderr)

if __name__ == "__main__":
    main()
