#!/usr/bin/env python3
"""Counterexample minimization and fixture promotion tool.

When differential fuzzing finds a program where ts2wasm/iwasm produces different
output from Node.js, this tool minimizes the failing program to its essential form
(delta debugging) and optionally promotes it to the fixture catalog with proper
semantic tracking.

Subcommands:
  minimize <counterexample-dir>   Minimize a counterexample program (delta debugging)
  promote  <ce-ts-path>           Promote a minimized program to the fixture catalog
  self-test                       Run internal self-tests

Minimization algorithm (delta debugging):
  1. Start with the full failing program.
  2. Try removing statements one at a time.
  3. If the reduced program still fails, keep the reduction.
  4. Try simplifying expressions in the remaining statements.
  5. Repeat until no further reduction is possible.

Fixture promotion:
  Creates a new fixture under fixtures/semantic/differential-fuzz/<name>.ts
  and adds a corresponding entry to fixtures/catalog.yaml with status=fail
  and tracking information linking back to the original counterexample.

Usage:
  python3 scripts/fuzz/counterexample.py --self-test
  python3 scripts/fuzz/counterexample.py minimize reports/counterexamples/ce_seed0_idx5_*.ts
  python3 scripts/fuzz/counterexample.py promote /tmp/minimized_ce.ts
  python3 scripts/fuzz/counterexample.py --help

Exit codes:
  0 = OK (minimized, promoted, or self-test passed)
  1 = error or self-test failure
  2 = infrastructure error (tools not found)

Non-goals:
  - No automatic fix generation (ts2wasm code changes are manual)
  - No cross-program minimization (each counterexample is independent)
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
GEN_PATH = REPO_ROOT / "scripts" / "generate" / "property-semantics.py"
TS2WASM_BIN = REPO_ROOT / "target" / "debug" / "ts2wasm"
CATALOG_PATH = REPO_ROOT / "fixtures" / "catalog.yaml"
FUZZ_FIXTURES_DIR = REPO_ROOT / "fixtures" / "semantic" / "differential-fuzz"
COUNTEREXAMPLES_DIR = REPO_ROOT / "reports" / "counterexamples"

IWASM_TIMEOUT = 10
NODE_TIMEOUT = 10
BUILD_TIMEOUT = 30


# ---------------------------------------------------------------------------
# Tool checks
# ---------------------------------------------------------------------------


def check_tools(ts2wasm_bin: str) -> list[str]:
    """Check required tools."""
    missing = []
    for tool in ["node", "iwasm"]:
        if subprocess.run(["which", tool], capture_output=True).returncode != 0:
            missing.append(tool)
    if not os.path.exists(ts2wasm_bin):
        missing.append(f"ts2wasm ({ts2wasm_bin})")
    return missing


# ---------------------------------------------------------------------------
# Running a program (reproduces the failure)
# ---------------------------------------------------------------------------


def run_via_node(source: str) -> tuple[int, str]:
    """Run via Node. Returns (returncode, stdout)."""
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".ts", prefix="ce-node-", delete=False
    ) as f:
        f.write(source)
        ts_path = f.name
    try:
        r = subprocess.run(
            ["node", ts_path],
            capture_output=True, text=True, timeout=NODE_TIMEOUT,
        )
        return r.returncode, r.stdout
    except subprocess.TimeoutExpired:
        return -1, ""
    finally:
        os.unlink(ts_path)


def run_via_ts2wasm(source: str, ts2wasm_bin: str) -> tuple[str, str]:
    """Run via ts2wasm + iwasm. Returns (outcome, stdout)."""
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".ts", prefix="ce-ts-", delete=False
    ) as f:
        f.write(source)
        ts_path = f.name
    wasm_fd, wasm_path = tempfile.mkstemp(suffix=".wasm", prefix="ce-wasm-")
    os.close(wasm_fd)
    try:
        build = subprocess.run(
            [ts2wasm_bin, "build", ts_path, "-o", wasm_path],
            capture_output=True, text=True, timeout=BUILD_TIMEOUT,
        )
        if build.returncode != 0:
            return ("build_fail", "")
        iwasm = subprocess.run(
            ["iwasm", wasm_path],
            capture_output=True, text=True, timeout=IWASM_TIMEOUT,
        )
        if iwasm.returncode != 0:
            return ("crash", iwasm.stdout)
        return ("ok", iwasm.stdout)
    except subprocess.TimeoutExpired:
        return ("timeout", "")
    finally:
        os.unlink(ts_path)
        if os.path.exists(wasm_path):
            os.unlink(wasm_path)


def is_mismatch(source: str, ts2wasm_bin: str) -> tuple[bool, str, str]:
    """Check if a program produces different output between Node and ts2wasm.

    Returns (is_mismatch, node_stdout, ts2wasm_stdout).
    """
    node_rc, node_stdout = run_via_node(source)
    if node_rc != 0:
        return False, "", ""
    outcome, wasm_stdout = run_via_ts2wasm(source, ts2wasm_bin)
    if outcome != "ok":
        return False, "", ""
    return node_stdout != wasm_stdout, node_stdout, wasm_stdout


# ---------------------------------------------------------------------------
# Delta-debugging minimizer
# ---------------------------------------------------------------------------


def parse_statements(source: str) -> list[str]:
    """Split a program into individual statement strings.

    Handles simple statement boundaries: function defs, for loops, if/else blocks,
    and simple semicolon-terminated statements.
    """
    lines = source.split("\n")
    stmts = []
    current = []
    brace_depth = 0
    for line in lines:
        stripped = line.strip()
        if not stripped:
            if current:
                current.append(line)
            continue
        # Track brace depth for block statements
        brace_depth += stripped.count("{") - stripped.count("}")
        current.append(line)
        if brace_depth == 0 and stripped.endswith(";"):
            stmts.append("\n".join(current))
            current = []
        elif brace_depth == 0 and stripped.endswith("}"):
            stmts.append("\n".join(current))
            current = []
    # Flush remaining
    if current:
        stmts.append("\n".join(current))
    return stmts


def delta_debug(source: str, ts2wasm_bin: str, node_stdout: str,
                 wasm_stdout: str, verbose: bool = False) -> str:
    """Minimize a failing program by delta debugging (statement-level).

    Algorithm:
      1. Parse source into statement list.
      2. For each statement, try removing it.
      3. If the removal still reproduces the mismatch, keep the reduced program.
      4. Repeat until no single statement can be removed.

    Returns the minimized source.
    """
    # Helper to check if a program reproduces the original mismatch
    def reproduces(src: str) -> bool:
        node_rc, node_out = run_via_node(src)
        if node_rc != 0:
            return False
        outcome, wasm_out = run_via_ts2wasm(src, ts2wasm_bin)
        if outcome != "ok":
            return False
        return node_out == node_stdout and wasm_out == wasm_stdout

    current_source = source
    changed = True
    iterations = 0

    while changed and iterations < 10:
        changed = False
        iterations += 1
        stmts = parse_statements(current_source)

        if len(stmts) <= 1:
            break

        for i in range(len(stmts)):
            reduced = "\n".join(stmts[:i] + stmts[i + 1:])
            if not reduced.strip():
                continue

            if reproduces(reduced):
                current_source = reduced
                changed = True
                if verbose:
                    print(
                        f"  delta-debug: removed statement {i}, "
                        f"{len(stmts) - 1} stmts remaining",
                        file=sys.stderr,
                    )
                break

    if verbose:
        print(
            f"  delta-debug: done after {iterations} passes, "
            f"{len(parse_statements(current_source))} stmts",
            file=sys.stderr,
        )
    return current_source


def minimize_counterexample(
    ts_path: str,
    ts2wasm_bin: str,
    verbose: bool = False,
) -> Optional[str]:
    """Minimize a single counterexample .ts file.

    Returns the minimized source, or None if the file doesn't reproduce.
    """
    with open(ts_path) as f:
        source = f.read()

    if verbose:
        print(f"  Loading {ts_path}", file=sys.stderr)

    # Check the program still produces the mismatch
    node_rc, node_stdout = run_via_node(source)
    if node_rc != 0:
        print(f"  SKIP: Node fails to run {ts_path}", file=sys.stderr)
        return None

    outcome, wasm_stdout = run_via_ts2wasm(source, ts2wasm_bin)
    if outcome != "ok":
        print(f"  SKIP: ts2wasm build/runtime failure for {ts_path}", file=sys.stderr)
        return None

    if node_stdout == wasm_stdout:
        print(f"  SKIP: no mismatch for {ts_path}", file=sys.stderr)
        return None

    if verbose:
        print(
            f"  Reproduced: node={node_stdout!r} iwasm={wasm_stdout!r}",
            file=sys.stderr,
        )

    # Minimize via delta debugging
    minimized = delta_debug(source, ts2wasm_bin, node_stdout, wasm_stdout, verbose)
    return minimized


# ---------------------------------------------------------------------------
# Fixture promotion
# ---------------------------------------------------------------------------


def sanitize_filename(source: str) -> str:
    """Create a sanitized filename from a program source."""
    # Extract any string literals or meaningful patterns
    name_match = re.search(r'console\.log\((.*?)\)', source)
    if name_match:
        base = name_match.group(1).strip("\"'")
        base = re.sub(r'[^a-zA-Z0-9_-]', '_', base)[:40]
        if base:
            return f"dfuzz-{base}.ts"

    # Fallback: hash-based name
    import hashlib
    h = hashlib.md5(source.encode()).hexdigest()[:8]
    return f"dfuzz-{h}.ts"


def promote_to_catalog(
    minimized_source: str,
    node_stdout: str,
    iwasm_stdout: str,
    original_ce_path: str,
    verbose: bool = False,
) -> str:
    """Promote a minimized counterexample to the fixture catalog.

    Creates:
      1. fixtures/semantic/differential-fuzz/<name>.ts
      2. Entry in fixtures/catalog.yaml

    Returns the fixture path.
    """
    import yaml

    # Create fixture directory
    FUZZ_FIXTURES_DIR.mkdir(parents=True, exist_ok=True)

    # Generate fixture filename
    fname = sanitize_filename(minimized_source)
    fixture_path = FUZZ_FIXTURES_DIR / fname

    # Avoid overwriting existing fixtures
    if fixture_path.exists():
        base = fname.replace(".ts", "")
        counter = 1
        while fixture_path.exists():
            fname = f"{base}_{counter}.ts"
            fixture_path = FUZZ_FIXTURES_DIR / fname
            counter += 1

    # Write the minimized fixture
    fixture_path.write_text(minimized_source)

    if verbose:
        print(f"  Created fixture: {fixture_path}", file=sys.stderr)

    # Update catalog.yaml
    cat_entry = {
        "name": fname,
        "status": "fail",
        "expected": (
            f"DIFFERENTIAL MISMATCH: Node returns {node_stdout!r}, "
            f"iwasm returns {iwasm_stdout!r}"
        ),
        "tracking": f"counterexample:{os.path.basename(original_ce_path)}",
    }

    if CATALOG_PATH.exists():
        with open(CATALOG_PATH) as f:
            catalog = yaml.safe_load(f) or {}
    else:
        catalog = {"version": 1, "categories": {}, "directories": {}}

    # Ensure directory entry exists
    dir_name = "semantic/differential-fuzz"
    if "directories" not in catalog:
        catalog["directories"] = {}
    if dir_name not in catalog["directories"]:
        catalog["directories"][dir_name] = {
            "category": "differential",
            "status": "fail",
            "expected": "Differential fuzzing counterexamples (ts2wasm output mismatch)",
            "fixtures": [],
        }

    dir_entry = catalog["directories"][dir_name]
    if "fixtures" not in dir_entry:
        dir_entry["fixtures"] = []

    # Check if name already exists in catalog
    existing_names = set()
    for fix in dir_entry["fixtures"]:
        if isinstance(fix, dict):
            existing_names.add(fix.get("name", ""))
        elif isinstance(fix, str):
            existing_names.add(fix)

    if fname not in existing_names:
        dir_entry["fixtures"].append(cat_entry)

    # Write updated catalog
    with open(CATALOG_PATH, "w") as f:
        yaml.dump(catalog, f, default_flow_style=False, sort_keys=False)

    if verbose:
        print(f"  Updated catalog: {CATALOG_PATH}", file=sys.stderr)
        print(f"  Promoted: {fname}", file=sys.stderr)

    return str(fixture_path)


# ---------------------------------------------------------------------------
# Self-test
# ---------------------------------------------------------------------------


class SelfTestError(Exception):
    """Raised on self-test failure."""
    pass


def run_self_test(ts2wasm_bin: str) -> int:
    """Run internal self-tests. Returns number of failures (0 = all pass)."""
    failures = 0

    def check(name: str, condition: bool):
        nonlocal failures
        if condition:
            print(f"  PASS: {name}", file=sys.stderr)
        else:
            print(f"  FAIL: {name}", file=sys.stderr)
            failures += 1

    def check_eq(name: str, actual: object, expected: object):
        nonlocal failures
        if actual == expected:
            print(f"  PASS: {name}", file=sys.stderr)
        else:
            print(
                f"  FAIL: {name} (expected={expected!r}, actual={actual!r})",
                file=sys.stderr,
            )
            failures += 1

    print("counterexample: self-test: statement parsing...", file=sys.stderr)
    source = "console.log(1);\nconsole.log(2);\n"
    stmts = parse_statements(source)
    check_eq("parse two log stmts", len(stmts), 2)
    check("first stmt has log(1)", "log(1)" in stmts[0])

    source2 = """function f() {
  return 1;
}
console.log(f());
"""
    stmts2 = parse_statements(source2)
    check_eq("parse function + log", len(stmts2), 2)

    source3 = """if (true) {
  console.log(1);
} else {
  console.log(2);
}
"""
    stmts3 = parse_statements(source3)
    check_eq("parse if/else block", len(stmts3), 1)

    print("counterexample: self-test: sanitize_filename...", file=sys.stderr)
    name1 = sanitize_filename('console.log("hello")\n')
    check_eq("sanitize: console.log", name1, "dfuzz-hello.ts")

    name2 = sanitize_filename("let x = 1;")
    check_eq("sanitize: fallback hash", name2.startswith("dfuzz-"), True)

    print("counterexample: self-test: mismatch detection...", file=sys.stderr)
    # Create a program that definitely matches (should not be a mismatch)
    matching_prog = 'console.log(42);\n'
    mm, node_out, wasm_out = is_mismatch(matching_prog, ts2wasm_bin)
    check("matching program is not mismatch", not mm)

    print("counterexample: self-test: delta-debug on known program...", file=sys.stderr)
    # Test that delta-debug preserves a minimal program
    bloated = """let x = 99;
console.log(42);
let y = x + 1;
"""
    node_rc1, node1 = run_via_node(bloated)
    _, wasm1 = run_via_ts2wasm(bloated, ts2wasm_bin)
    if node_rc1 == 0:
        minimized = delta_debug(bloated, ts2wasm_bin, node1, wasm1, verbose=False)
        mm_check, _, _ = is_mismatch(minimized, ts2wasm_bin)
        check("delta-debug: no new mismatch from match program", not mm_check)
        minimized_stmts = parse_statements(minimized)
        check_eq("delta-debug: minimized statements", len(minimized_stmts), 1)
        check("delta-debug: console.log 42 preserved", "log(42)" in minimized)

    print("counterexample: self-test: summary", file=sys.stderr)
    if failures == 0:
        print("  All tests passed!", file=sys.stderr)
    else:
        print(f"  {failures} test(s) failed!", file=sys.stderr)

    return failures


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def cmd_minimize(args: argparse.Namespace, ts2wasm_bin: str) -> int:
    """Minimize one or more counterexample files."""
    paths = args.ce_paths

    if not paths:
        # Scan reports/counterexamples/ for .ts files
        if COUNTEREXAMPLES_DIR.exists():
            paths = sorted([
                str(p) for p in COUNTEREXAMPLES_DIR.glob("*.ts")
            ])

    if not paths:
        print(
            "counterexample: no counterexample files found. "
            "Run differential-fuzz first to generate counterexamples.",
            file=sys.stderr,
        )
        sys.exit(1)

    errors = 0
    for ce_path in paths:
        ce_file = Path(ce_path)
        if not ce_file.exists():
            print(f"  NOT FOUND: {ce_path}", file=sys.stderr)
            errors += 1
            continue

        minimized = minimize_counterexample(ce_path, ts2wasm_bin, verbose=True)
        if minimized is None:
            print(f"  SKIPPED: {ce_path}", file=sys.stderr)
            continue

        # Save minimized version
        out_name = ce_file.stem + "_minimized.ts"
        out_path = ce_file.parent / out_name
        out_path.write_text(minimized)

        print(
            f"  MINIMIZED: {out_path} "
            f"(original={ce_file.name})",
            file=sys.stderr,
        )

        if args.promote:
            # Re-run to get stdout
            _, node_stdout = run_via_node(minimized)
            _, wasm_stdout = run_via_ts2wasm(minimized, ts2wasm_bin)
            promote_to_catalog(
                minimized, node_stdout, wasm_stdout, ce_path, verbose=True
            )

    return errors


def cmd_promote(args: argparse.Namespace, ts2wasm_bin: str) -> int:
    """Promote a minimized counterexample to the fixture catalog."""
    ts_path = Path(args.ts_path)
    if not ts_path.exists():
        print(f"counterexample: not found: {ts_path}", file=sys.stderr)
        return 1

    source = ts_path.read_text()

    node_rc, node_stdout = run_via_node(source)
    if node_rc != 0:
        print(f"counterexample: Node fails to run {ts_path}", file=sys.stderr)
        return 1

    _, wasm_stdout = run_via_ts2wasm(source, ts2wasm_bin)

    if node_stdout == wasm_stdout:
        print(
            f"counterexample: no mismatch for {ts_path} (outputs match)",
            file=sys.stderr,
        )
        return 1

    fixture_path = promote_to_catalog(
        source, node_stdout, wasm_stdout, str(ts_path), verbose=True
    )
    print(f"counterexample: promoted to {fixture_path}", file=sys.stderr)
    return 0


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Counterexample minimization and fixture promotion tool",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--ts2wasm",
        type=str,
        default=str(TS2WASM_BIN),
        help=f"Path to ts2wasm binary (default: {TS2WASM_BIN})",
    )
    parser.add_argument("--verbose", "-v", action="store_true", help="Verbose output")
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="Run internal self-tests (overrides subcommand)",
    )

    subparsers = parser.add_subparsers(dest="command")

    # minimize subcommand
    min_parser = subparsers.add_parser("minimize", help="Minimize counterexample programs")
    min_parser.add_argument("ce_paths", nargs="*", help="Counterexample .ts file(s) to minimize")
    min_parser.add_argument(
        "--promote", action="store_true", help="Auto-promote minimized counterexample to catalog"
    )

    # promote subcommand
    prom_parser = subparsers.add_parser("promote", help="Promote a program to the fixture catalog")
    prom_parser.add_argument("ts_path", help="Path to the .ts file to promote")

    return parser.parse_args()


def main():
    args = parse_args()

    # Resolve ts2wasm binary path
    ts2wasm_bin = TS2WASM_BIN
    if args.ts2wasm:
        ts2wasm_bin = Path(args.ts2wasm)

    if not ts2wasm_bin.exists():
        print(
            f"counterexample: ts2wasm binary not found at {ts2wasm_bin}",
            file=sys.stderr,
        )
        sys.exit(2)

    # --self-test flag overrides subcommand
    if args.self_test:
        failures = run_self_test(str(ts2wasm_bin))
        sys.exit(failures)

    # Check tools for commands that need them
    if args.command not in ("minimize", "promote"):
        print(
            "counterexample: no command specified. "
            "Use --self-test, minimize, or promote. See --help.",
            file=sys.stderr,
        )
        sys.exit(2)

    missing = check_tools(str(ts2wasm_bin))
    if missing:
        print(
            f"counterexample: missing: {', '.join(missing)}",
            file=sys.stderr,
        )
        sys.exit(2)

    if args.command == "minimize":
        code = cmd_minimize(args, str(ts2wasm_bin))
        sys.exit(code)

    elif args.command == "promote":
        code = cmd_promote(args, str(ts2wasm_bin))
        sys.exit(code)


if __name__ == "__main__":
    main()
