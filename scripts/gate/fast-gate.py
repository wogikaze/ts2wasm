#!/usr/bin/env python3
"""Standard local gate: fmt + architecture/P0 entry checks + nextest (optional).

Usage:
  mise run gate [-- --skip-nextest]
  mise run gate-fast

Environment:
  TS2WASM_FAST_GATE_SKIP_NEXTEST=1  Same as --skip-nextest (for pre-push).
  TS2WASM_RUN_PERF_GATE=1            Run the optional benchmark smoke gate.

Dependencies: cargo, python3 (see nested scripts for cargo-nextest, jq, etc.)
"""

import os
import sys
import subprocess
import shutil
import json
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
PYTHON_BIN = os.environ.get("PYTHON_BIN", sys.executable)

def usage():
    print("Usage:")
    print("  mise run gate [-- --skip-nextest]")
    print("  mise run gate-fast")
    print()
    print("Runs:")
    print("  - rustfmt legacy-aware check")
    print("  - python scripts/check/tracking-consistency.py")
    print("  - python scripts/check/architecture-rules.py")
    print("  - P0 architecture entry checks")
    print("  - optional benchmark tracker when TS2WASM_RUN_PERF_GATE=1")
    print("  - cargo nextest run (unless --skip-nextest)")
    print()
    print("Options:")
    print("  --skip-nextest   Skip cargo nextest (faster; use in pre-push with targeted tests).")

def run(cmd, cwd=REPO_ROOT):
    """Run a command and exit if it fails."""
    print(f"gate: {' '.join(cmd)}", file=sys.stderr)
    result = subprocess.run(cmd, cwd=cwd)
    if result.returncode != 0:
        sys.exit(result.returncode)

def run_perf_gate():
    """Run the optional performance smoke gate and fail on reported regressions."""
    output_path = Path(tempfile.gettempdir()) / "ts2wasm-perf-gate-results.json"
    cmd = [
        PYTHON_BIN,
        str(REPO_ROOT / "scripts/perf/benchmark-tracker.py"),
        "--json",
        "--sample",
        "1",
        "--output",
        str(output_path),
    ]
    print(f"gate: {' '.join(cmd)}", file=sys.stderr)
    result = subprocess.run(cmd, cwd=REPO_ROOT, capture_output=True, text=True)
    if result.stderr:
        print(result.stderr, file=sys.stderr, end="")
    if result.returncode != 0:
        sys.exit(result.returncode)

    stdout_lines = [line for line in result.stdout.splitlines() if line.strip()]
    if not stdout_lines:
        print("gate: benchmark tracker did not emit JSON", file=sys.stderr)
        sys.exit(1)
    try:
        summary = json.loads(stdout_lines[-1])
    except json.JSONDecodeError as exc:
        print(f"gate: invalid benchmark JSON: {exc}", file=sys.stderr)
        sys.exit(1)

    compiler_throughput = summary.get("compiler_throughput_fixtures_per_sec", 0)
    perf_regression = bool(summary.get("perf_regression"))
    print(
        "gate: benchmark tracker "
        f"compiler_throughput={compiler_throughput} "
        f"perf_regression={perf_regression}",
        file=sys.stderr,
    )
    if perf_regression:
        print("gate: performance regression detected", file=sys.stderr)
        sys.exit(1)

def main():
    skip_nextest = os.environ.get("TS2WASM_FAST_GATE_SKIP_NEXTEST", "0") == "1"
    run_optional_perf_gate = os.environ.get("TS2WASM_RUN_PERF_GATE", "0") == "1"
    
    args = sys.argv[1:]
    while args:
        if args[0] == "--skip-nextest":
            skip_nextest = True
            args.pop(0)
        elif args[0] in ("-h", "--help"):
            usage()
            sys.exit(0)
        else:
            print(f"gate: unknown option: {args[0]}", file=sys.stderr)
            usage()
            sys.exit(1)
    
    # Check for required commands
    for cmd in ["python3", "python"]:
        if shutil.which(cmd):
            break
    else:
        print("gate: missing required command: python/python3", file=sys.stderr)
        sys.exit(1)

    if not shutil.which("cargo"):
        print("gate: missing required command: cargo", file=sys.stderr)
        sys.exit(1)
    
    # Run checks
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/rustfmt-legacy-aware.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/tracking-consistency.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/assert-true-detect.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/architecture-rules.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/crate-dag.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/legacy-freeze.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/specop-dispatch.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/trace-contract.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/architecture-exceptions.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/docs-routing.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/compiler-source-truth.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/check-runtimefn-deprecation.py")])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/check/coverage-classification.py"), "--self-test"])
    run([PYTHON_BIN, str(REPO_ROOT / "scripts/report/native-runtime-builder-coverage.py"), "--check"])
    if run_optional_perf_gate:
        run_perf_gate()
    else:
        print("gate: skipping benchmark tracker (TS2WASM_RUN_PERF_GATE=1 to run)", file=sys.stderr)
    
    if not skip_nextest:
        run(["cargo", "nextest", "run"])
    else:
        print("gate: skipping cargo nextest (--skip-nextest)", file=sys.stderr)
    
    print("gate: OK", file=sys.stderr)

if __name__ == "__main__":
    main()
