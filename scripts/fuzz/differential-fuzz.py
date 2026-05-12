#!/usr/bin/env python3
"""Differential fuzzing harness: generates programs, compiles with ts2wasm, runs under iwasm, and compares to Node.js output.

This harness extends the property-semantics generator into an open-ended fuzzing loop.
It detects:
  - Output mismatches (Node vs iwasm produce different stdout)
  - Compiler crashes (ts2wasm build fails with non-zero exit)
  - Runtime crashes (iwasm exits with non-zero or times out)
  - Timeouts (iwasm exceeds configurable timeout)

When mismatches are found, the failing program is saved to a counterexamples directory
for later analysis and minimization.

Usage:
  python3 scripts/fuzz/differential-fuzz.py                    # default 30s fuzz
  python3 scripts/fuzz/differential-fuzz.py --timeout 60       # 60 seconds
  python3 scripts/fuzz/differential-fuzz.py --timeout 30 --seed 42 --verbose
  python3 scripts/fuzz/differential-fuzz.py --help

Exit codes:
  0 = no mismatches found within fuzzing budget
  1 = one or more mismatches detected
  2 = infrastructure error (tools not found, etc.)

Non-goals:
  - No structured crash minimization (handled by scripts/fuzz/counterexample.py)
  - No automatic fix generation
"""

import argparse
import json
import os
import random
import subprocess
import sys
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path

# Re-use the program generator from property-semantics
REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
GEN_PATH = REPO_ROOT / "scripts" / "generate" / "property-semantics.py"
TS2WASM_BIN = REPO_ROOT / "target" / "debug" / "ts2wasm"
COUNTEREXAMPLES_DIR = REPO_ROOT / "reports" / "counterexamples"

# Defaults
DEFAULT_TIMEOUT = 30  # seconds
IWASM_TIMEOUT = 10    # per-program iwasm timeout
NODE_TIMEOUT = 10     # per-program Node timeout
BUILD_TIMEOUT = 30    # per-program build timeout


def ensure_counterexamples_dir():
    """Create the counterexamples directory if it doesn't exist."""
    COUNTEREXAMPLES_DIR.mkdir(parents=True, exist_ok=True)


# ---------------------------------------------------------------------------
# Generator (wraps property-semantics.ProgramGenerator)
# ---------------------------------------------------------------------------


def _load_generator():
    """Dynamically import ProgramGenerator from property-semantics."""
    import importlib.util

    spec = importlib.util.spec_from_file_location("property_semantics", GEN_PATH)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod.ProgramGenerator


ProgramGenerator = None


def get_generator():
    """Lazy-load the ProgramGenerator class."""
    global ProgramGenerator
    if ProgramGenerator is None:
        ProgramGenerator = _load_generator()
    return ProgramGenerator


# ---------------------------------------------------------------------------
# Runner helpers
# ---------------------------------------------------------------------------


def check_tools(ts2wasm_bin: str) -> list[str]:
    """Check that required tools are available."""
    missing = []
    for tool in ["node", "iwasm"]:
        if subprocess.run(["which", tool], capture_output=True).returncode != 0:
            missing.append(tool)
    if not os.path.exists(ts2wasm_bin):
        missing.append(f"ts2wasm binary ({ts2wasm_bin})")
    return missing


def run_program_via_node(source: str) -> tuple[int, str, str]:
    """Run a program via Node.js. Returns (returncode, stdout, stderr)."""
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".ts", prefix="dfuzz-node-", delete=False
    ) as f:
        f.write(source)
        ts_path = f.name
    try:
        result = subprocess.run(
            ["node", ts_path],
            capture_output=True,
            text=True,
            timeout=NODE_TIMEOUT,
        )
        return result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return -1, "", "timeout"
    finally:
        os.unlink(ts_path)


def run_program_via_ts2wasm(
    source: str, ts2wasm_bin: str
) -> tuple[str, str, str]:
    """Run a program via ts2wasm build + iwasm.

    Returns a tuple of (outcome, stdout, detail):
      outcome: "ok" | "build_fail" | "iwasm_fail" | "iwasm_crash" | "timeout"
      stdout: program stdout if OK, empty otherwise
      detail: error message or empty
    """
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".ts", prefix="dfuzz-ts-", delete=False
    ) as f:
        f.write(source)
        ts_path = f.name

    wasm_fd, wasm_path = tempfile.mkstemp(suffix=".wasm", prefix="dfuzz-wasm-")
    os.close(wasm_fd)

    try:
        # Build
        build_result = subprocess.run(
            [ts2wasm_bin, "build", ts_path, "-o", wasm_path],
            capture_output=True,
            text=True,
            timeout=BUILD_TIMEOUT,
        )
        if build_result.returncode != 0:
            return ("build_fail", "", build_result.stderr[:300])

        # Run with iwasm
        iwasm_result = subprocess.run(
            ["iwasm", wasm_path],
            capture_output=True,
            text=True,
            timeout=IWASM_TIMEOUT,
        )
        if iwasm_result.returncode != 0:
            return ("iwasm_crash", iwasm_result.stdout, iwasm_result.stderr[:300])

        return ("ok", iwasm_result.stdout, "")
    except subprocess.TimeoutExpired:
        return ("timeout", "", "")
    finally:
        os.unlink(ts_path)
        if os.path.exists(wasm_path):
            os.unlink(wasm_path)


# ---------------------------------------------------------------------------
# Counterexample reporting
# ---------------------------------------------------------------------------


def save_counterexample(source: str, seed: int, index: int, node_stdout: str,
                        iwasm_stdout: str, detail: str) -> str:
    """Save a counterexample program to the counterexamples directory.

    Returns the path to the saved file.
    """
    ensure_counterexamples_dir()
    timestamp = datetime.now(timezone.utc).strftime("%Y%m%d-%H%M%S")
    base = f"ce_seed{seed}_idx{index}_{timestamp}"
    ts_path = str(COUNTEREXAMPLES_DIR / f"{base}.ts")
    meta_path = str(COUNTEREXAMPLES_DIR / f"{base}.json")

    with open(ts_path, "w") as f:
        f.write(source)

    with open(meta_path, "w") as f:
        json.dump(
            {
                "seed": seed,
                "index": index,
                "timestamp": timestamp,
                "node_stdout": node_stdout,
                "iwasm_stdout": iwasm_stdout,
                "detail": detail,
            },
            f,
            indent=2,
        )

    return ts_path


# ---------------------------------------------------------------------------
# Fuzzing loop
# ---------------------------------------------------------------------------


def fuzz_loop(
    ts2wasm_bin: str,
    seed: int,
    timeout_seconds: int,
    verbose: bool = False,
    max_depth: int = 4,
) -> dict:
    """Run the fuzzing loop.

    Args:
        ts2wasm_bin: Path to ts2wasm binary
        seed: Random seed for generation
        timeout_seconds: Maximum fuzzing duration in seconds
        verbose: Print per-program progress
        max_depth: Maximum expression nesting depth

    Returns:
        dict with counts and discovered counterexamples
    """
    GenClass = get_generator()
    generator = GenClass(seed=seed, max_depth=max_depth)

    counts = {
        "total": 0,
        "ok": 0,          # Node and iwasm output match
        "mismatch": 0,     # Output differs
        "build_fail": 0,   # ts2wasm build fails
        "iwasm_crash": 0,  # iwasm exits with non-zero
        "timeout": 0,       # iwasm timeout
        "node_error": 0,    # Node fails to run program
    }
    counterexamples = []

    start_time = time.time()
    deadline = start_time + timeout_seconds
    gen_index = 0

    while time.time() < deadline:
        gen_index += 1
        program = generator.generate_program()

        # Run via Node
        node_rc, node_stdout, node_stderr = run_program_via_node(program)
        if node_rc != 0:
            counts["node_error"] += 1
            counts["total"] += 1
            continue

        # Run via ts2wasm
        outcome, wasm_stdout, detail = run_program_via_ts2wasm(program, ts2wasm_bin)

        if outcome == "build_fail":
            counts["build_fail"] += 1
        elif outcome == "iwasm_crash":
            counts["iwasm_crash"] += 1
        elif outcome == "timeout":
            counts["timeout"] += 1
        elif outcome == "ok":
            if node_stdout == wasm_stdout:
                counts["ok"] += 1
            else:
                counts["mismatch"] += 1
                save_path = save_counterexample(
                    program, seed, gen_index, node_stdout, wasm_stdout, detail
                )
                counterexamples.append(
                    {
                        "index": gen_index,
                        "program": program,
                        "node_stdout": node_stdout,
                        "iwasm_stdout": wasm_stdout,
                        "saved_at": save_path,
                    }
                )
        else:
            counts["iwasm_crash"] += 1  # unknown outcome, treat as crash

        counts["total"] += 1

        if verbose and (gen_index % 10 == 0 or gen_index == 1):
            elapsed = time.time() - start_time
            fps = gen_index / elapsed if elapsed > 0 else 0
            print(
                f"[{time.time() - start_time:.1f}s] "
                f"programs={gen_index} ok={counts['ok']} "
                f"mismatch={counts['mismatch']} build_fail={counts['build_fail']} "
                f"crash={counts['iwasm_crash']} tmo={counts['timeout']} "
                f"({fps:.1f} prog/s)",
                file=sys.stderr,
            )

    elapsed = time.time() - start_time
    return {
        "elapsed": elapsed,
        "gen_index": gen_index,
        "counts": counts,
        "counterexamples": counterexamples,
    }


def format_duration(seconds: float) -> str:
    """Format duration as human-readable string."""
    if seconds < 60:
        return f"{seconds:.1f}s"
    return f"{seconds / 60:.1f}m"


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Differential fuzzing harness: Node vs ts2wasm/iwasm comparison",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--timeout",
        type=int,
        default=DEFAULT_TIMEOUT,
        help=f"Fuzzing duration in seconds (default: {DEFAULT_TIMEOUT})",
    )
    parser.add_argument(
        "--seed",
        type=int,
        default=0,
        help="Random seed for deterministic generation (default: 0)",
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Print per-program progress",
    )
    parser.add_argument(
        "--ts2wasm",
        type=str,
        default=str(TS2WASM_BIN),
        help=f"Path to ts2wasm binary (default: {TS2WASM_BIN})",
    )
    parser.add_argument(
        "--max-depth",
        type=int,
        default=4,
        help="Maximum expression nesting depth (default: 4)",
    )
    parser.add_argument(
        "--save-all",
        action="store_true",
        help="Save all results (including OK) for analysis",
    )
    return parser.parse_args()


def main():
    args = parse_args()

    # Resolve ts2wasm binary path
    ts2wasm_bin = str(Path(args.ts2wasm).resolve()) if args.ts2wasm else ""

    # Check tools
    missing = check_tools(ts2wasm_bin if os.path.exists(ts2wasm_bin) else TS2WASM_BIN)
    if missing:
        print(
            f"differential-fuzz: missing required tools: {', '.join(missing)}",
            file=sys.stderr,
        )
        sys.exit(2)

    # Ensure ts2wasm binary if default is used
    ts2wasm_bin_resolved = str(TS2WASM_BIN)
    if args.ts2wasm:
        ts2wasm_bin_resolved = str(Path(args.ts2wasm).resolve())
    else:
        if not TS2WASM_BIN.exists():
            print(
                "differential-fuzz: ts2wasm binary not found, building...",
                file=sys.stderr,
            )
            result = subprocess.run(
                ["cargo", "build", "-p", "ts2wasm-cli"],
                cwd=REPO_ROOT,
                capture_output=True,
                text=True,
            )
            if result.returncode != 0:
                print(
                    f"differential-fuzz: cargo build failed: {result.stderr[:300]}",
                    file=sys.stderr,
                )
                sys.exit(2)

    print(
        f"differential-fuzz: seed={args.seed} timeout={args.timeout}s "
        f"ts2wasm={ts2wasm_bin_resolved}",
        file=sys.stderr,
    )

    # Run fuzzing loop
    result = fuzz_loop(
        ts2wasm_bin=ts2wasm_bin_resolved,
        seed=args.seed,
        timeout_seconds=args.timeout,
        verbose=args.verbose,
        max_depth=args.max_depth,
    )

    # Report summary
    c = result["counts"]
    print(
        f"differential-fuzz: summary: "
        f"ok={c['ok']} mismatch={c['mismatch']} "
        f"build_fail={c['build_fail']} crash={c['iwasm_crash']} "
        f"timeout={c['timeout']} node_error={c['node_error']} "
        f"total={c['total']} elapsed={format_duration(result['elapsed'])}",
        file=sys.stderr,
    )

    # Report counterexamples
    ces = result["counterexamples"]
    if ces:
        print(
            f"differential-fuzz: {len(ces)} counterexample(s) saved:",
            file=sys.stderr,
        )
        for ce in ces[:5]:
            print(
                f"  [{ce['index']}] {ce['saved_at']} "
                f"node={ce['node_stdout']!r} iwasm={ce['iwasm_stdout']!r}",
                file=sys.stderr,
            )
        print(
            "differential-fuzz: FAILED: mismatches detected",
            file=sys.stderr,
        )
        sys.exit(1)

    # Exit code logic
    if c["mismatch"] > 0:
        print(
            "differential-fuzz: FAILED: mismatches detected",
            file=sys.stderr,
        )
        sys.exit(1)

    print("differential-fuzz: PASSED (no mismatches)", file=sys.stderr)


if __name__ == "__main__":
    main()
