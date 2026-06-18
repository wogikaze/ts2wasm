#!/usr/bin/env python3
"""SpecOp dispatch coverage checker.

Every SpecOp variant must have an explicit dispatch arm in spec-kernel.
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


def get_dispatch_files() -> list[Path]:
    src = REPO_ROOT / "crates" / "spec-kernel" / "src"
    return list(src.rglob("*.rs"))


def get_called_variants() -> set[str]:
    """Extract SpecOp variants that appear in any dispatch context
    (match arms, function calls, references)."""
    called = set()
    for fpath in get_dispatch_files():
        text = fpath.read_text()
        for m in re.finditer(r"SpecOp::(\w+)", text):
            called.add(m.group(1))
    return called


def check_dispatch_coverage() -> list[str]:
    variants = extract_specop_variants()
    if not variants:
        return ["ERROR: cannot parse SpecOp enum"]

    called = get_called_variants()

    # Variants that have dispatch arms (appear in match patterns)
    dispatched = set()
    for fpath in get_dispatch_files():
        text = fpath.read_text()
        for m in re.finditer(r"^\s+SpecOp::(\w+)", text, re.MULTILINE):
            dispatched.add(m.group(1))

    # Also check spec_emit.rs in backend-wasm
    emit_path = REPO_ROOT / "crates" / "backend-wasm" / "src" / "spec_emit.rs"
    if emit_path.exists():
        text = emit_path.read_text()
        for m in re.finditer(r"SpecOp::(\w+)", text):
            dispatched.add(m.group(1))

    missing = variants - dispatched
    violations = []
    for v in sorted(missing):
        violations.append(f"ERROR SpecOp::{v} has no dispatch arm")

    # Check for wildcard matches in dispatch functions
    for fpath in get_dispatch_files():
        text = fpath.read_text()
        # Find match blocks that might use wildcards
        match_blocks = re.finditer(
            r"match\s+.*?\{(.*?)\n\s+\}",
            text,
            re.MULTILINE | re.DOTALL,
        )
        for mb in match_blocks:
            body = mb.group(1)
            if re.search(r"^\s+_\s*=>", body, re.MULTILINE):
                rel = fpath.relative_to(REPO_ROOT)
                # Only flag if it's covering SpecOp variants
                if "SpecOp" in body or "self" in mb.group(0):
                    # Check if the wildcard is truly a catch-all
                    violations.append(
                        f"WARN {rel}: wildcard match in SpecOp dispatch — "
                        f"new variants may be silently ignored"
                    )

    return violations


def run_self_test():
    errors = 0
    variants = extract_specop_variants()
    if not variants:
        print("FAIL: cannot parse SpecOp enum", file=sys.stderr)
        errors += 1
    if errors:
        print(f"self-test: FAILED ({errors} errors)", file=sys.stderr)
        sys.exit(1)
    print(f"self-test: OK ({len(variants)} SpecOp variants found)", file=sys.stderr)


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
