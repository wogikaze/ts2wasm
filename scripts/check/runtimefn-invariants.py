#!/usr/bin/env python3
"""Run in-tree RuntimeLinkPlan / runtime helper unit tests (no integration / iwasm).
Reuses crates/cli src/backend runtime_link_plan.rs and runtime_fn.rs #[test] items.

Usage: mise run check runtimefn
Uses: cargo test (not nextest) for simple filter on internal module path.
"""

import sys
import subprocess
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

def main():
    if not shutil.which("cargo"):
        print("check_runtimefn_invariants: cargo is required", file=sys.stderr)
        sys.exit(1)
    
    print("check_runtimefn_invariants: runtime-catalog tests", file=sys.stderr)
    result = subprocess.run(
        ["cargo", "test", "-p", "ts2wasm-runtime-catalog", "--", "--quiet"],
        cwd=REPO_ROOT
    )
    if result.returncode != 0:
        sys.exit(result.returncode)
    
    print("check_runtimefn_invariants: backend-wasm runtime_link_plan", file=sys.stderr)
    result = subprocess.run(
        ["cargo", "test", "-p", "ts2wasm-backend-wasm", "--test", "runtime_link_plan", "--", "--quiet"],
        cwd=REPO_ROOT
    )
    if result.returncode != 0:
        sys.exit(result.returncode)
    
    print("check_runtimefn_invariants: backend-wasm host_import_capability", file=sys.stderr)
    result = subprocess.run(
        ["cargo", "test", "-p", "ts2wasm-backend-wasm", "--test", "host_import_capability", "--", "--quiet"],
        cwd=REPO_ROOT
    )
    if result.returncode != 0:
        sys.exit(result.returncode)
    
    print("check_runtimefn_invariants: OK", file=sys.stderr)

if __name__ == "__main__":
    main()
