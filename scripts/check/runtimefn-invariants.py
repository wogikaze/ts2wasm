#!/usr/bin/env python3
"""Run in-tree RuntimeLinkPlan / runtime helper unit tests (no integration / iwasm).
Reuses crates/cli src/backend runtime_link_plan.rs and runtime_fn.rs #[test] items.

Usage: python scripts/manager.py check-runtimefn-invariants
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
    
    print("check_runtimefn_invariants: runtime_link_plan::tests", file=sys.stderr)
    result = subprocess.run(
        ["cargo", "test", "-p", "ts2wasm-cli", "--lib", "runtime_link_plan::tests", "--", "--quiet"],
        cwd=REPO_ROOT
    )
    if result.returncode != 0:
        sys.exit(result.returncode)
    
    print("check_runtimefn_invariants: runtime_fn::tests", file=sys.stderr)
    result = subprocess.run(
        ["cargo", "test", "-p", "ts2wasm-cli", "--lib", "runtime_fn::tests", "--", "--quiet"],
        cwd=REPO_ROOT
    )
    if result.returncode != 0:
        sys.exit(result.returncode)
    
    print("check_runtimefn_invariants: OK", file=sys.stderr)

if __name__ == "__main__":
    main()
