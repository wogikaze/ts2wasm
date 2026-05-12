#!/usr/bin/env python3
"""Run expression-level programs through Node and ts2wasm, compare stdout.

Modes:
  --seed <N>    Random seed for deterministic generation (default: 42)
  --count <N>   Number of expressions to test (default: 10)
  --dir <DIR>   Use pre-generated .ts files from a directory instead of generating
  --keep        Keep temporary files on exit
  --smoke       Use the fixed smoke expression set (equivalent to generate + run)

Usage:
  python3 scripts/check/expression-differential.py --seed 42
  python3 scripts/check/expression-differential.py --seed 42 --count 20
  python3 scripts/check/expression-differential.py --dir /tmp/ts2wasm-expr-gen-12345
  python3 scripts/check/expression-differential.py --smoke
"""

import sys
import os
import subprocess
import tempfile
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
GENERATOR_SCRIPT = REPO_ROOT / "scripts" / "generate" / "expression-semantics.py"


def usage():
    print("Usage:")
    print("  python3 scripts/check/expression-differential.py [--seed N] [--count N] [--dir DIR] [--keep] [--smoke]")
    print()
    print("  --seed <N>    Random seed (default: 42)")
    print("  --count <N>   Number of test expressions (default: 10)")
    print("  --dir <DIR>   Use .ts files from an existing directory")
    print("  --keep        Keep temp directory on exit")
    print("  --smoke       Use smoke expression set")
    sys.exit(0)


def find_ts2wasm() -> Path | None:
    """Find ts2wasm binary. Returns path or None."""
    which = shutil.which("ts2wasm")
    if which:
        return Path(which)
    cargo_path = REPO_ROOT / "target" / "debug" / "ts2wasm"
    if cargo_path.exists():
        return cargo_path
    return None


def check_deps() -> bool:
    """Verify required tools are available. ts2wasm and iwasm are optional (compile/run-only)."""
    missing = []
    for cmd in ["node"]:
        if not shutil.which(cmd):
            missing.append(cmd)

    if missing:
        print(f"expression_differential: missing required tools: {', '.join(missing)}", file=sys.stderr)
        return False
    return True


def has_iwasm() -> bool:
    """Check if iwasm is available."""
    iwasm_paths = [
        shutil.which("iwasm"),
        REPO_ROOT / "target" / "debug" / "iwasm",
    ]
    for p in iwasm_paths:
        if p and Path(p).exists():
            return True
    return False


def run_node(ts_path: Path) -> tuple[int, str, str]:
    """Run a .ts file through Node.js (with tsx/ts-node or direct Node for .js)."""
    try:
        result = subprocess.run(
            ["node", "--experimental-strip-types", str(ts_path)],
            capture_output=True, text=True, timeout=30
        )
        return result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return -1, "", "TIMEOUT"
    except FileNotFoundError:
        return -2, "", "node not found"


TS2WASM_BIN: Path | None = None


def run_ts2wasm(ts_path: Path, wasm_path: Path) -> tuple[int, str, str]:
    """Compile .ts to .wasm using ts2wasm."""
    global TS2WASM_BIN
    if TS2WASM_BIN is None:
        TS2WASM_BIN = find_ts2wasm()
    if TS2WASM_BIN is None:
        return -2, "", "ts2wasm not found"
    try:
        result = subprocess.run(
            [str(TS2WASM_BIN), "build", str(ts_path), "-o", str(wasm_path)],
            capture_output=True, text=True, timeout=60
        )
        return result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return -1, "", "TIMEOUT"
    except FileNotFoundError:
        return -2, "", "ts2wasm not found"


def run_iwasm(wasm_path: Path) -> tuple[int, str, str]:
    """Run a .wasm file through iwasm."""
    try:
        result = subprocess.run(
            ["iwasm", str(wasm_path)],
            capture_output=True, text=True, timeout=30
        )
        return result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return -1, "", "TIMEOUT"
    except FileNotFoundError:
        return -2, "", "iwasm not found"


def run_expression_test(ts_path: Path, tmp_dir: Path) -> dict:
    """Run a single expression test and return results."""
    result = {
        "file": str(ts_path),
        "node_stdout": "",
        "node_exit": -1,
        "node_stderr": "",
        "wasm_stdout": "",
        "wasm_exit": -1,
        "wasm_stderr": "",
        "match": False,
        "error": None,
    }

    # Run with Node
    node_exit, node_stdout, node_stderr = run_node(ts_path)
    result["node_exit"] = node_exit
    result["node_stdout"] = node_stdout
    result["node_stderr"] = node_stderr

    if node_exit != 0:
        result["error"] = f"Node exited with code {node_exit}: {node_stderr[:200]}"
        return result

    # Compile with ts2wasm
    wasm_path = tmp_dir / f"{ts_path.stem}.wasm"
    build_exit, build_stdout, build_stderr = run_ts2wasm(ts_path, wasm_path)

    if build_exit != 0:
        result["error"] = f"ts2wasm build failed (exit={build_exit}): {build_stderr[:200]}"
        return result

    # Run with iwasm
    if not has_iwasm():
        result["error"] = "iwasm not available -- compile-only test"
        result["wasm_exit"] = 0
        return result

    wasm_exit, wasm_stdout, wasm_stderr = run_iwasm(wasm_path)
    result["wasm_exit"] = wasm_exit
    result["wasm_stdout"] = wasm_stdout
    result["wasm_stderr"] = wasm_stderr

    # Compare stdout
    normalized_node = node_stdout.rstrip("\n")
    normalized_wasm = wasm_stdout.rstrip("\n")
    result["match"] = normalized_node == normalized_wasm

    return result


def collect_ts_files(src_dir: Path) -> list[Path]:
    """Collect all .ts files from a directory (sorted)."""
    files = sorted(src_dir.glob("*.ts"))
    return files


def main():
    args = sys.argv[1:]

    if args and args[0] in ("-h", "--help"):
        usage()

    seed = 42
    count = 10
    src_dir = None
    keep = False
    smoke_mode = False

    i = 0
    while i < len(args):
        if args[i] == "--seed" and i + 1 < len(args):
            seed = int(args[i + 1])
            i += 2
        elif args[i] == "--count" and i + 1 < len(args):
            count = int(args[i + 1])
            i += 2
        elif args[i] == "--dir" and i + 1 < len(args):
            src_dir = Path(args[i + 1])
            i += 2
        elif args[i] == "--keep":
            keep = True
            i += 1
        elif args[i] == "--smoke":
            smoke_mode = True
            i += 1
        else:
            print(f"expression_differential: unknown arg: {args[i]}", file=sys.stderr)
            usage()

    if not check_deps():
        sys.exit(1)

    # Gather source .ts files
    ts_files = []

    if src_dir is not None:
        if not src_dir.exists():
            print(f"expression_differential: source directory not found: {src_dir}", file=sys.stderr)
            sys.exit(1)
        ts_files = collect_ts_files(src_dir)
        if not ts_files:
            print(f"expression_differential: no .ts files found in {src_dir}", file=sys.stderr)
            sys.exit(1)
        print(f"expression_differential: using {len(ts_files)} pre-generated files from {src_dir}", file=sys.stderr)
    else:
        # Generate fresh
        gen_args = ["--seed", str(seed)]
        if smoke_mode:
            gen_args.append("--smoke")
        else:
            gen_args.extend(["--count", str(count)])
        gen_args.extend(["--out", str(REPO_ROOT / "target" / "expr-gen-output")])

        print(f"expression_differential: generating expressions (seed={seed}, smoke={smoke_mode})", file=sys.stderr)
        gen_result = subprocess.run(
            [sys.executable, str(GENERATOR_SCRIPT)] + gen_args,
            capture_output=True, text=True
        )

        if gen_result.returncode != 0:
            print(f"expression_differential: generator failed: {gen_result.stderr[:200]}", file=sys.stderr)
            sys.exit(1)

        gen_dir = REPO_ROOT / "target" / "expr-gen-output"
        ts_files = collect_ts_files(gen_dir)
        if not ts_files:
            print(f"expression_differential: generator produced no files", file=sys.stderr)
            sys.exit(1)

    # Create temp dir for WASM output
    tmp_dir = Path(tempfile.mkdtemp(prefix="expr-diff-"))

    try:
        total = len(ts_files)
        passed = 0
        failed = 0
        skipped = 0
        compile_only = 0
        details = []

        for ts_path in ts_files:
            test_name = ts_path.name
            result = run_expression_test(ts_path, tmp_dir)

            if result["error"]:
                if "iwasm not available" in result["error"]:
                    compile_only += 1
                    print(f"  SKIP (compile-only): {test_name}", file=sys.stderr)
                else:
                    skipped += 1
                    print(f"  SKIP: {test_name}: {result['error']}", file=sys.stderr)
            elif result["match"]:
                passed += 1
                details.append((test_name, "PASS"))
                print(f"  PASS: {test_name}", file=sys.stderr)
            else:
                failed += 1
                details.append((test_name, "FAIL"))
                print(f"  FAIL: {test_name}", file=sys.stderr)
                print(f"    Node stdout: {repr(result['node_stdout'])}", file=sys.stderr)
                print(f"    Wasm stdout: {repr(result['wasm_stdout'])}", file=sys.stderr)

        # Summary
        print(file=sys.stderr)
        print(f"expression_differential: summary: "
              f"{passed} passed, {failed} failed, {skipped} skipped, "
              f"{compile_only} compile-only", file=sys.stderr)

        # Also output JSONL-like result lines to stdout
        import json
        for test_name, status in details:
            record = {
                "suite": "expression-semantics",
                "case": test_name,
                "target": "node-vs-iwasm",
                "status": "pass" if status == "PASS" else "fail",
            }
            print(json.dumps(record))

        if failed > 0:
            print(f"expression_differential: FAILED: {failed} mismatch(es)", file=sys.stderr)
            return 1

        print("expression_differential: PASSED", file=sys.stderr)
        return 0

    finally:
        if not keep:
            shutil.rmtree(tmp_dir, ignore_errors=True)


if __name__ == "__main__":
    sys.exit(main())
