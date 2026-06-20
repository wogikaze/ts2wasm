#!/usr/bin/env python3
"""SpecOp dispatch coverage checker.

Every SpecOp variant must have explicit lowering metadata:
  - spec_op.rs param_count/result_count arms
  - backend-wasm spec_emit symbol mapping
  - backend-wasm spec_emit builder mapping
Wildcard matches (_ =>) are not allowed as a substitute.

Usage:
  python scripts/check/specop-dispatch.py
  python scripts/check/specop-dispatch.py --self-test
"""

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()


def extract_specop_variants() -> set[str]:
    path = REPO_ROOT / "crates" / "spec-kernel" / "src" / "spec_op.rs"
    text = path.read_text()
    enum_match = re.search(r"pub enum SpecOp \{(.*?)^\}", text, re.MULTILINE | re.DOTALL)
    if not enum_match:
        return set()
    body = enum_match.group(1)
    variants = set()
    for m in re.finditer(r"^\s+([A-Z]\w+)\s*\{?", body, re.MULTILINE):
        variants.add(m.group(1))
    return variants


def check_dispatch_coverage() -> list[str]:
    variants = extract_specop_variants()
    if not variants:
        return ["ERROR: cannot parse SpecOp enum"]

    violations = []

    spec_op_path = REPO_ROOT / "crates" / "spec-kernel" / "src" / "spec_op.rs"
    spec_text = spec_op_path.read_text()
    for fn_name in ("param_count", "result_count"):
        fn_match = re.search(
            rf"pub fn {fn_name}\(&self\) -> usize \{{(.*?)^\s+\}}",
            spec_text,
            re.MULTILINE | re.DOTALL,
        )
        if not fn_match:
            violations.append(f"ERROR SpecOp::{fn_name} cannot be parsed")
            continue
        arms = set(re.findall(r"Self::(\w+)\s*\{", fn_match.group(1)))
        missing = variants - arms
        for v in sorted(missing):
            violations.append(f"ERROR SpecOp::{v} missing from {fn_name} metadata")

    emit_path = REPO_ROOT / "crates" / "backend-correctness" / "src" / "spec_emit.rs"
    if not emit_path.exists():
        violations.append("ERROR crates/backend-correctness/src/spec_emit.rs not found")
    else:
        emit_text = emit_path.read_text()
        symbol_match = re.search(
            r"fn spec_op_symbol\(op: &SpecOp\) -> String \{(.*?)^\}",
            emit_text,
            re.MULTILINE | re.DOTALL,
        )
        builder_match = re.search(
            r"fn build_spec_op_function\(name: &str\) -> Option<WasmFunction> \{(.*?)^\}",
            emit_text,
            re.MULTILINE | re.DOTALL,
        )
        if not symbol_match:
            violations.append("ERROR spec_emit.rs missing spec_op_symbol")
        if not builder_match:
            violations.append("ERROR spec_emit.rs missing build_spec_op_function")
        symbols = {}
        if symbol_match:
            for variant, symbol in re.findall(
                r"SpecOp::(\w+)\s*\{[^}]*\}\s*=>\s*\"([^\"]+)\"\.into\(\)",
                symbol_match.group(1),
                re.DOTALL,
            ):
                symbols[variant] = symbol
            for v in sorted(variants - set(symbols)):
                violations.append(f"ERROR SpecOp::{v} missing spec_emit symbol mapping")
        # Also check build_algo_op_function_with_program (SpecAlgoIR path)
        algo_match = re.search(
            r"fn build_algo_op_function_with_program.*?\{(.*?)^\}",
            emit_text,
            re.MULTILINE | re.DOTALL,
        )
        algo_builder_symbols: set[str] = set()
        if algo_match:
            algo_builder_symbols = set(re.findall(r'\"(\$spec_[a-z_]+)\"', algo_match.group(1)))
            # Remove symbols that are in the return None group
            return_none_match = re.search(
                r"\$spec_get_binding_value.*?\|.*?\$spec_push_string_constant\s*=>\s*\{\s*return None;",
                algo_match.group(1), re.DOTALL
            )
            if return_none_match:
                return_none_symbols = set(re.findall(r'\$spec_[a-z_]+', return_none_match.group(0)))
                algo_builder_symbols -= return_none_symbols

        if builder_match and symbols:
            # Merge symbols from both builder paths (SpecAlgoIR + hand-written fallback)
            fallback_symbols = set(re.findall(r"\"([^\"]+)\"\s*=>\s*\{?\s*Some\(", builder_match.group(1)))
            all_builder_symbols = fallback_symbols | algo_builder_symbols
            for variant, symbol in sorted(symbols.items()):
                if symbol not in all_builder_symbols:
                    violations.append(
                        f"ERROR SpecOp::{variant} symbol {symbol} missing spec_emit builder mapping"
                    )
            if re.search(r"_\s*=>\s*None", builder_match.group(1)):
                violations.append(
                    "ERROR spec_emit build_spec_op_function has wildcard None arm — "
                    "new SpecOp variants must fail explicitly"
                )

    # Check for wildcard matches in SpecOp-specific dispatch functions only
    # (spec_op.rs param_count/result_count, not ObjectKind/EnvironmentRecord dispatches)
    lines = spec_text.splitlines()
    rel = spec_op_path.relative_to(REPO_ROOT)
    in_fn = False
    fn_name = ""
    for i, line in enumerate(lines):
        m = re.match(r'^\s*(pub\s+)?fn\s+(\w+)', line)
        if m:
            in_fn = True
            fn_name = m.group(2)
        if in_fn and re.match(r'^\s+_\s*=>', line):
            violations.append(
                f"ERROR {rel}:{i+1}: wildcard in fn `{fn_name}` — "
                f"new SpecOp variants must be listed explicitly"
            )
            in_fn = False

    return violations


def run_self_test():
    errors = 0
    variants = extract_specop_variants()
    if not variants:
        print("FAIL: cannot parse SpecOp enum", file=sys.stderr)
        errors += 1

    dispatch_violations = check_dispatch_coverage()
    if dispatch_violations:
        print("FAIL: current SpecOp dispatch coverage is incomplete", file=sys.stderr)
        for v in dispatch_violations:
            print(f"  {v}", file=sys.stderr)
        errors += 1

    if errors:
        print(f"self-test: FAILED ({errors} errors)", file=sys.stderr)
        sys.exit(1)
    print(f"self-test: OK ({len(variants)} SpecOp variants)", file=sys.stderr)


def main():
    args = sys.argv[1:]
    if "-h" in args or "--help" in args:
        print(__doc__.strip())
        sys.exit(0)
    if "--self-test" in args:
        run_self_test()
        return

    violations = check_dispatch_coverage()
    for v in violations:
        print(f"specop_dispatch: {v}", file=sys.stderr)

    errors = [v for v in violations if v.startswith("ERROR")]
    if errors:
        print(f"specop_dispatch: FAILED ({len(errors)} errors)", file=sys.stderr)
        sys.exit(1)

    print(f"specop_dispatch: OK ({len(violations)} warnings)", file=sys.stderr)


if __name__ == "__main__":
    main()
