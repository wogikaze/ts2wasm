#!/usr/bin/env python3
"""Inventory and baseline: toolchain, P0 harness scripts, existing gates, optional P1+ stubs.

Default: planned (P1+) scripts may be missing — warn only. P0 must exist and pass.
  REQUIRE_ALL_HARNESSES=1  — treat P1+ scripts as required (exist + executable) too.
Nextest: default is plain `cargo nextest run` (warnings allowed). Strict:
  TS2WASM_NEXTEST_DENY_WARNINGS=1  —  RUSTFLAGS='-D warnings' (project may fail until #011 is done).

Usage: python scripts/manager.py check-harness-installation
"""

import sys
import subprocess
import shutil
import os
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

fail = 0

def ok(msg):
    print(f"harness: OK: {msg}")

def bad(msg):
    global fail
    print(f"harness: FAIL: {msg}", file=sys.stderr)
    fail = 1

def warn(msg):
    print(f"harness: WARN: {msg}", file=sys.stderr)

def need_cmd(cmd):
    if shutil.which(cmd):
        ok(f"command: {cmd}")
    else:
        bad(f"missing command: {cmd}")

def need_exec_required(path):
    full_path = REPO_ROOT / path
    if full_path.exists() and os.access(full_path, os.X_OK):
        ok(f"executable: {path}")
    else:
        bad(f"missing executable: {path}")

def need_exec_optional(path):
    full_path = REPO_ROOT / path
    if full_path.exists() and os.access(full_path, os.X_OK):
        ok(f"executable: {path}")
    else:
        warn(f"not installed (optional in default mode): {path}")

def run_check(name, cmd, cwd=REPO_ROOT):
    print("", file=sys.stderr)
    print(f"== {name} ==", file=sys.stderr)
    result = subprocess.run(cmd, cwd=cwd)
    if result.returncode == 0:
        ok(name)
    else:
        bad(name)

def main():
    global fail
    
    print("== toolchain (quick) ==", file=sys.stderr)
    need_cmd("cargo")
    need_cmd("bash")
    need_cmd("node")
    need_cmd("iwasm")
    need_cmd("jq")
    need_cmd("git")
    need_cmd("wasm-tools")
    
    if shutil.which("ast-grep") or shutil.which("sg"):
        ok("ast-grep/sg")
    else:
        bad("ast-grep/sg")
    
    if shutil.which("ig") or shutil.which("rg"):
        ok("ig/rg")
    else:
        bad("ig or rg")
    
    result = subprocess.run(["cargo", "nextest", "--version"], capture_output=True)
    if result.returncode == 0:
        ok("cargo nextest")
    else:
        bad("cargo nextest")
    
    print("", file=sys.stderr)
    print("== P0 harness (must exist) ==", file=sys.stderr)
    # Use Python versions where available
    p0_harnesses = [
        "scripts/check/toolchain.py",
        "scripts/check/fixture-differential.py",
        "scripts/check/host-deny.py",
        "scripts/check/runtimefn-invariants.py",
        "scripts/check/wasm-validation.py",
    ]
    for f in p0_harnesses:
        need_exec_required(f)
    
    print("", file=sys.stderr)
    print("== P1+ planned harnesses (default: optional) ==", file=sys.stderr)
    require_all = os.environ.get("REQUIRE_ALL_HARNESSES", "0") == "1"
    check_func = need_exec_required if require_all else need_exec_optional
    
    p1_harnesses = [
        "scripts/check/docs-health.sh",
        "scripts/check/agent-policy.sh",
        "scripts/check/benchmark-regression.sh",
        "scripts/check/scripts-behavior.sh",
        "scripts/check/determinism.sh",
    ]
    for f in p1_harnesses:
        check_func(f)
    
    print("", file=sys.stderr)
    print("== required repo gates (script files) ==", file=sys.stderr)
    required_gates = [
        "scripts/check/shell-syntax.sh",
        "scripts/check/issue-health.py",
        "scripts/gen/coverage-matrix.py",
        "scripts/gate/fast-gate.py",
        "scripts/check/manifest-imports.py",
        "scripts/check/test-records-schema.py",
        "scripts/check/fixture-catalog.py",
        "scripts/check/architecture-rules.py",
        "scripts/check/compiler-diagnostics.py",
    ]
    for f in required_gates:
        need_exec_required(f)
    
    print("", file=sys.stderr)
    print("== run P0 harnesses ==", file=sys.stderr)
    run_check("P0: check_toolchain", [sys.executable, str(REPO_ROOT / "scripts/check/toolchain.py")])
    run_check("P0: check_fixture_differential", [sys.executable, str(REPO_ROOT / "scripts/check/fixture-differential.py")])
    run_check("P0: check_host_deny", [sys.executable, str(REPO_ROOT / "scripts/check/host-deny.py")])
    run_check("P0: check_runtimefn_invariants", [sys.executable, str(REPO_ROOT / "scripts/check/runtimefn-invariants.py")])
    run_check("P0: check_wasm_validation", [sys.executable, str(REPO_ROOT / "scripts/check/wasm-validation.py")])
    
    print("", file=sys.stderr)
    print("== run aggregate gates (fast gate without nextest first) ==", file=sys.stderr)
    run_check("scripts/manager check-fast-gate --skip-nextest", [sys.executable, str(REPO_ROOT / "scripts/gate/fast-gate.py"), "--skip-nextest"])
    
    if os.environ.get("TS2WASM_NEXTEST_DENY_WARNINGS", "0") == "1":
        print("harness: TS2WASM_NEXTEST_DENY_WARNINGS=1 (RUSTFLAGS=-D warnings)", file=sys.stderr)
        env = os.environ.copy()
        env["RUSTFLAGS"] = "-D warnings"
        run_check("cargo nextest (RUSTFLAGS=-D warnings)", ["cargo", "nextest", "run"], env=env)
    else:
        print("harness: (hint) set TS2WASM_NEXTEST_DENY_WARNINGS=1 to fail on Rust warnings (see issues/open/011-*.md)", file=sys.stderr)
        run_check("cargo nextest", ["cargo", "nextest", "run"])
    
    print("", file=sys.stderr)
    print("== additional custom harnesses ==", file=sys.stderr)
    run_check("scripts/manager check-manifest-imports", [sys.executable, str(REPO_ROOT / "scripts/check/manifest-imports.py")])
    run_check("scripts/manager check-test-records-schema (empty)", [sys.executable, str(REPO_ROOT / "scripts/check/test-records-schema.py")], input=b"")
    run_check("scripts/manager check-fixture-catalog", [sys.executable, str(REPO_ROOT / "scripts/check/fixture-catalog.py")])
    run_check("scripts/manager check-architecture-rules", [sys.executable, str(REPO_ROOT / "scripts/check/architecture-rules.py")])
    run_check("scripts/manager check-compiler-diagnostics", [sys.executable, str(REPO_ROOT / "scripts/check/compiler-diagnostics.py")])
    
    print("", file=sys.stderr)
    if fail == 0:
        print("HARNESS BASELINE PASSED", file=sys.stderr)
        sys.exit(0)
    print("HARNESS BASELINE FAILED", file=sys.stderr)
    sys.exit(1)

if __name__ == "__main__":
    main()
