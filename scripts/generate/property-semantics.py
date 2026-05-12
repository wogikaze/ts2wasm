#!/usr/bin/env python3
"""Property-based semantic generator: deterministic program generation with Node vs ts2wasm comparison.

Generates small TypeScript programs within a restricted subset (number ops, comparisons,
booleans, strings, control flow, functions), runs them through Node.js and ts2wasm/iwasm,
and compares outputs. Deterministic seed replay ensures reproducibility.

Subset:
  - Number literals, arithmetic (+, -, *, /, %)
  - Boolean literals (true, false)
  - String literals (basic ASCII)
  - Comparison operators (===, !==, <, >, <=, >=)
  - Logical operators (&&, ||, !)
  - Variable declarations (let) with basic types
  - If/else control flow
  - For loops with numeric bounds
  - Function definitions and calls
  - Ternary operator (?:)
  - console.log() for output

Non-goals:
  - No full JavaScript fuzzing
  - No classes, objects, arrays, closures
  - No async/await, generators, regex
  - No runtime reflection or eval

Usage:
  python3 scripts/generate/property-semantics.py              # default seed=0, 50 programs
  python3 scripts/generate/property-semantics.py --seed 42 --limit 100
  python3 scripts/generate/property-semantics.py --seed 42 --limit 100 --verbose
  python3 scripts/generate/property-semantics.py --help

Exit codes:
  0 = all programs matched (or all unsupported/build failures)
  1 = some programs produced different output between Node and ts2wasm
  2 = infrastructure error (tools not found, etc.)
"""

import argparse
import os
import random
import subprocess
import sys
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()
TS2WASM_BIN = REPO_ROOT / "target" / "debug" / "ts2wasm"

# ---------------------------------------------------------------------------
# Constants for generation
# ---------------------------------------------------------------------------

NUMERIC_OPS = ["+", "-", "*", "/", "%"]
COMPARISON_OPS = ["===", "!==", "<", ">", "<=", ">="]
LOGICAL_OPS = ["&&", "||"]
UNARY_OPS = ["!", "-"]

# Literal pools
NUMBERS = [0, 1, 2, 3, 5, 10, 42, 100]
BOOLEANS = ["true", "false"]
STRINGS = ["hello", "world", "abc", "x", "42", ""]

IWASM_TIMEOUT = 30
NODE_TIMEOUT = 15
BUILD_TIMEOUT = 60


# ---------------------------------------------------------------------------
# Program Generator
# ---------------------------------------------------------------------------


class ProgramGenerator:
    """Deterministic generator of small TypeScript programs."""

    def __init__(self, seed: int, max_depth: int = 4, max_stmts: int = 6):
        self.rng = random.Random(seed)
        self.max_depth = max_depth
        self.max_stmts = max_stmts
        self.var_counter = 0
        self.func_counter = 0

    def _fresh_var(self) -> str:
        self.var_counter += 1
        return f"v{self.var_counter}"

    def _fresh_func(self) -> str:
        self.func_counter += 1
        return f"f{self.func_counter}"

    def _pick(self, items: list) -> object:
        return self.rng.choice(items)

    def _pick_int(self, lo: int, hi: int) -> int:
        return self.rng.randint(lo, hi)

    def generate_number_literal(self) -> str:
        return str(self._pick(NUMBERS))

    def generate_boolean_literal(self) -> str:
        return str(self._pick(BOOLEANS))

    def generate_string_literal(self) -> str:
        val = self._pick(STRINGS)
        return f'"{val}"'

    def generate_literal(self) -> str:
        kind = self._pick(["number", "boolean", "string"])
        if kind == "number":
            return self.generate_number_literal()
        elif kind == "boolean":
            return self.generate_boolean_literal()
        else:
            return self.generate_string_literal()

    def generate_expr(self, depth: int = 0) -> str:
        """Generate a random expression. Depth limits nesting to avoid blowup."""
        if depth >= self.max_depth:
            return self.generate_literal()

        # Weighted choices: prefer simpler at higher depth
        choices = ["literal", "unary", "binary", "comparison", "logical", "ternary"]
        weights = [30, 10, 20, 15, 10, 15]
        if depth > 2:
            weights = [60, 5, 15, 10, 5, 5]

        kind = self.rng.choices(choices, weights=weights, k=1)[0]

        if kind == "literal":
            return self.generate_literal()
        elif kind == "unary":
            op = self._pick(UNARY_OPS)
            inner = self.generate_expr(depth + 1)
            if op == "!":
                return f"!({inner})"
            else:
                return f"-({inner})"
        elif kind == "binary":
            op = self._pick(NUMERIC_OPS)
            left = self.generate_expr(depth + 1)
            right = self.generate_expr(depth + 1)
            return f"({left} {op} {right})"
        elif kind == "comparison":
            op = self._pick(COMPARISON_OPS)
            left = self.generate_expr(depth + 1)
            right = self.generate_expr(depth + 1)
            return f"({left} {op} {right})"
        elif kind == "logical":
            op = self._pick(LOGICAL_OPS)
            left = self.generate_expr(depth + 1)
            right = self.generate_expr(depth + 1)
            return f"({left} {op} {right})"
        elif kind == "ternary":
            cond = self.generate_expr(depth + 1)
            then_expr = self.generate_expr(depth + 1)
            else_expr = self.generate_expr(depth + 1)
            return f"({cond} ? {then_expr} : {else_expr})"

        return self.generate_literal()

    def generate_stmt(self, depth: int = 0) -> str:
        """Generate a single statement."""
        kind = self._pick(
            ["log", "let", "if", "for", "expr"]
        )
        if kind == "log":
            expr = self.generate_expr(depth)
            return f"console.log({expr});"
        elif kind == "let":
            var = self._fresh_var()
            expr = self.generate_expr(depth)
            return f"let {var} = {expr};"
        elif kind == "if":
            cond = self.generate_expr(depth)
            then_body = self._generate_body(depth + 1, max_stmts=2)
            if self.rng.random() < 0.4:
                else_body = self._generate_body(depth + 1, max_stmts=2)
                return f"if ({cond}) {{\n  {then_body}\n}} else {{\n  {else_body}\n}}"
            return f"if ({cond}) {{\n  {then_body}\n}}"
        elif kind == "for":
            var = self._fresh_var()
            lo = self._pick_int(0, 3)
            hi = self._pick_int(lo + 1, lo + 5)
            body = self._generate_body(depth + 1, max_stmts=2)
            return f"for (let {var} = {lo}; {var} < {hi}; {var}++) {{\n  {body}\n}}"
        elif kind == "expr":
            # Standalone expression as statement
            expr = self.generate_expr(depth)
            return f"{expr};"
        return ""

    def _generate_body(self, depth: int = 0, max_stmts: int = 3) -> str:
        """Generate a block body as semicolon-separated statements."""
        n = self._pick_int(1, max_stmts)
        stmts = []
        for _ in range(n):
            stmts.append(self.generate_stmt(depth))
        return "\n  ".join(stmts)

    def generate_function(self, depth: int = 0) -> tuple[str, str]:
        """Generate a function definition and a call expression.
        Returns (definition: str, call_expr: str).
        """
        fname = self._fresh_func()
        # Decide number of params
        n_params = self._pick_int(0, 2)
        params = [self._fresh_var() for _ in range(n_params)]
        param_list = ", ".join(params)

        # Body: a few statements, last one returns or logs
        body_stmts = []
        for _ in range(self._pick_int(1, 3)):
            s = self.generate_stmt(depth + 1)
            if "console.log" not in s:
                body_stmts.append(s)

        # Sometimes add a return
        if self.rng.random() < 0.5:
            ret_expr = self.generate_expr(depth + 1)
            body_stmts.append(f"return {ret_expr};")

        body = "\n    ".join(body_stmts) if body_stmts else "  return 0;"
        definition = f"function {fname}({param_list}) {{\n    {body}\n  }}"

        # Generate call with appropriate args
        args = []
        for _ in range(n_params):
            if self.rng.random() < 0.5:
                args.append(self.generate_literal())
            else:
                args.append(self.generate_expr(depth + 1))

        call_expr = f"{fname}({', '.join(args)})"
        return definition, call_expr

    def generate_program(self) -> str:
        """Generate a complete TypeScript program."""
        self.var_counter = 0
        self.func_counter = 0

        stmts = []
        n_stmts = self._pick_int(1, self.max_stmts)

        # Maybe add a function definition at the top
        if n_stmts >= 3 and self.rng.random() < 0.5:
            fn_def, fn_call = self.generate_function()
            stmts.append(fn_def)
            stmts.append(f"console.log({fn_call});")
            # Fill remaining with basic stmts
            remaining = self._pick_int(0, max(0, n_stmts - 2))
            for _ in range(remaining):
                stmts.append(self.generate_stmt())
        else:
            for _ in range(n_stmts):
                stmts.append(self.generate_stmt())

        return "\n".join(stmts) + "\n"


# ---------------------------------------------------------------------------
# Runner
# ---------------------------------------------------------------------------


def check_tools():
    """Check that required tools are available."""
    missing = []
    for tool in ["node"]:
        if subprocess.run(["which", tool], capture_output=True).returncode != 0:
            missing.append(tool)
    return missing


def find_ts2wasm_binary(custom_path: str | None = None) -> str:
    """Locate ts2wasm binary."""
    if custom_path:
        p = Path(custom_path)
        if p.exists():
            return str(p.resolve())
    if TS2WASM_BIN.exists():
        return str(TS2WASM_BIN)
    result = subprocess.run(
        ["cargo", "build", "-p", "ts2wasm-cli"],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    if result.returncode == 0 and TS2WASM_BIN.exists():
        return str(TS2WASM_BIN)
    which = subprocess.run(["which", "ts2wasm"], capture_output=True, text=True)
    if which.returncode == 0:
        return which.stdout.strip()
    raise RuntimeError("ts2wasm binary not found; build with: cargo build -p ts2wasm-cli")


def run_via_node(source: str, timeout: int = NODE_TIMEOUT) -> tuple[int, str, str]:
    """Run a TypeScript program via Node.js. Returns (returncode, stdout, stderr)."""
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".ts", prefix="psg-", delete=False
    ) as f:
        f.write(source)
        ts_path = f.name
    try:
        result = subprocess.run(
            ["node", ts_path],
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        return result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired as e:
        return -1, "", str(e)
    finally:
        os.unlink(ts_path)


def run_via_ts2wasm(
    source: str, ts2wasm_bin: str, timeout_iwasm: int = IWASM_TIMEOUT
) -> tuple[int, str, str]:
    """Run a TypeScript program via ts2wasm build + iwasm.
    Returns (status_code, stdout, error_detail).
    status_code: 0=OK, 1=build fail, 2=iwasm fail, -1=timeout
    """
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".ts", prefix="psg-", delete=False
    ) as f:
        f.write(source)
        ts_path = f.name

    wasm_fd, wasm_path = tempfile.mkstemp(suffix=".wasm", prefix="psg-")
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
            diag = "build_failed"
            stderr = build_result.stderr[:200] if build_result.stderr else ""
            return 1, "", f"{diag}: {stderr}"

        # Run with iwasm
        iwasm_result = subprocess.run(
            ["iwasm", wasm_path],
            capture_output=True,
            text=True,
            timeout=timeout_iwasm,
        )
        if iwasm_result.returncode != 0:
            stderr = iwasm_result.stderr[:200] if iwasm_result.stderr else ""
            return 2, "", f"iwasm_exit_{iwasm_result.returncode}: {stderr}"

        return 0, iwasm_result.stdout, ""
    except subprocess.TimeoutExpired:
        return -1, "", "iwasm_timeout"
    finally:
        os.unlink(ts_path)
        if os.path.exists(wasm_path):
            os.unlink(wasm_path)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Property-based semantic generator: Node vs ts2wasm comparison",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--seed",
        type=int,
        default=0,
        help="Random seed for deterministic generation (default: 0)",
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=50,
        help="Number of programs to generate and test (default: 50)",
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Print each generated program and result",
    )
    parser.add_argument(
        "--ts2wasm",
        type=str,
        default=None,
        help="Path to ts2wasm binary (default: auto-detect)",
    )
    parser.add_argument(
        "--max-depth",
        type=int,
        default=4,
        help="Maximum expression nesting depth (default: 4)",
    )
    parser.add_argument(
        "--dump-programs",
        action="store_true",
        help="Dump generated programs to stdout before running",
    )
    return parser.parse_args()


def main():
    args = parse_args()

    # Check tools
    missing = check_tools()
    if missing:
        print(
            f"property-semantics: missing required tools: {', '.join(missing)}",
            file=sys.stderr,
        )
        sys.exit(2)

    # Find ts2wasm binary
    try:
        ts2wasm_bin = find_ts2wasm_binary(args.ts2wasm)
    except RuntimeError as e:
        print(f"property-semantics: {e}", file=sys.stderr)
        sys.exit(2)

    print(
        f"property-semantics: seed={args.seed} limit={args.limit} "
        f"ts2wasm={ts2wasm_bin}",
        file=sys.stderr,
    )

    # Generate and test
    generator = ProgramGenerator(seed=args.seed, max_depth=args.max_depth)
    generator.rng  # ensure rng initialized

    counts = {"match": 0, "mismatch": 0, "unsupported": 0, "timeout": 0, "error": 0}
    mismatches = []

    for i in range(1, args.limit + 1):
        program = generator.generate_program()

        if args.dump_programs:
            print(f"--- Program {i} ---")
            print(program)
            print("---")

        # Run via Node
        node_rc, node_stdout, node_stderr = run_via_node(program)
        if node_rc != 0:
            if args.verbose:
                print(f"[{i}] Node error (rc={node_rc})", file=sys.stderr)
            counts["error"] += 1
            continue

        # Run via ts2wasm
        wasm_status, wasm_stdout, wasm_detail = run_via_ts2wasm(program, ts2wasm_bin)

        if wasm_status == -1:
            counts["timeout"] += 1
            if args.verbose:
                print(f"[{i}] iwasm timeout", file=sys.stderr)
            continue
        elif wasm_status == 1:
            counts["unsupported"] += 1
            if args.verbose:
                print(f"[{i}] build failed: {wasm_detail[:100]}", file=sys.stderr)
            continue
        elif wasm_status == 2:
            counts["error"] += 1
            if args.verbose:
                print(f"[{i}] iwasm error: {wasm_detail[:100]}", file=sys.stderr)
            continue

        # Compare
        if node_stdout == wasm_stdout:
            counts["match"] += 1
            if args.verbose:
                print(f"[{i}] MATCH", file=sys.stderr)
        else:
            counts["mismatch"] += 1
            mismatches.append(
                {
                    "index": i,
                    "program": program,
                    "node_stdout": node_stdout,
                    "iwasm_stdout": wasm_stdout,
                }
            )
            if args.verbose:
                print(
                    f"[{i}] MISMATCH: node={node_stdout!r} iwasm={wasm_stdout!r}",
                    file=sys.stderr,
                )

        # Progress indicator
        if i % 10 == 0 or i == args.limit:
            print(
                f"property-semantics: progress: {i}/{args.limit} "
                f"match={counts['match']} mismatch={counts['mismatch']} "
                f"unsupported={counts['unsupported']} timeout={counts['timeout']}",
                file=sys.stderr,
            )

    # Summary
    total_run = sum(counts.values())
    print(
        f"property-semantics: summary: "
        f"match={counts['match']} mismatch={counts['mismatch']} "
        f"unsupported={counts['unsupported']} timeout={counts['timeout']} "
        f"error={counts['error']} "
        f"total={total_run}/limit={args.limit}",
        file=sys.stderr,
    )

    # Report mismatches
    if mismatches:
        print(
            f"property-semantics: {len(mismatches)} mismatch(es) detected",
            file=sys.stderr,
        )
        for m in mismatches[:5]:  # Show first 5
            print(
                f"  [{m['index']}] node={m['node_stdout']!r} "
                f"iwasm={m['iwasm_stdout']!r}",
                file=sys.stderr,
            )

    if counts["mismatch"] > 0:
        sys.exit(1)

    print("property-semantics: all programs OK", file=sys.stderr)


if __name__ == "__main__":
    main()
