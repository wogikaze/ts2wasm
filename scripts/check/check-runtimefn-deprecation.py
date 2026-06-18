#!/usr/bin/env python3
"""Check that no new RuntimeFn variants overlap with SpecOp equivalents.

This script:
  1. Reads the RuntimeFn enum from crates/runtime-catalog/src/runtime_fn.rs
  2. Checks a hardcoded list of "deprecatable" variants (those with SpecOp equivalents)
  3. Fails if a variant on the deprecation list is added to emission_order
     (meaning the new pipeline can't avoid it)

Usage:
  python scripts/check/check-runtimefn-deprecation.py

Exit code:
  0 = no new deprecated variants in active use
  1 = deprecated variant found in emission_order
"""

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent.resolve()

# RuntimeFn variants that have direct SpecOp equivalents.
# These should NOT be required by new pipeline output.
# If any of these appear in emission_order(), something is still using old path.
DEPRECATED_VARIANTS = {
    "PropertyGet",
    "PropertySet",
    "PropertyDelete",
    "PropertyHas",
    "ObjectKeys",
    "ObjectGetOwnPropertyNames",
    "ObjectGetOwnPropertyDescriptor",
    "ObjectGetPrototypeOf",
    "ObjectSetPrototypeOf",
    "ObjectIsExtensible",
    "ObjectPreventExtensions",
    "ObjectDefineProperty",
    "ReflectDefineProperty",
    "ReflectDeleteProperty",
    "ReflectGet",
    "ReflectHas",
    "ReflectSet",
    "ReflectApply",
    "ReflectConstruct",
    "GetIterator",
    "IteratorNext",
    "TruthyBool",
    "NumberCoerce",
    "IsString",
}


def check_emission_order() -> list[str]:
    path = REPO_ROOT / "crates" / "runtime-catalog" / "src" / "runtime_fn.rs"
    text = path.read_text()

    fn_match = re.search(
        r"pub const fn emission_order\(\)\s*->\s*&'static \[RuntimeFn\]\s*\{(.*?)^\}",
        text,
        re.MULTILINE | re.DOTALL,
    )
    if not fn_match:
        return ["cannot find emission_order() function"]

    body = fn_match.group(1)
    used = set()
    for m in re.finditer(r"Self::(\w+)", body):
        used.add(m.group(1))

    violations = []
    for variant in sorted(used & DEPRECATED_VARIANTS):
        violations.append(
            f"check_runtimefn_deprecation: WARN {variant} is in emission_order "
            f"but has a SpecOp equivalent — consider migrating"
        )
    return violations


def check_runtimefn_enum() -> list[str]:
    path = REPO_ROOT / "crates" / "runtime-catalog" / "src" / "runtime_fn.rs"
    text = path.read_text()

    enum_match = re.search(r"pub enum RuntimeFn \{(.*?)^\}", text, re.MULTILINE | re.DOTALL)
    if not enum_match:
        return ["cannot find RuntimeFn enum"]

    body = enum_match.group(1)
    violations = []
    for variant in sorted(DEPRECATED_VARIANTS):
        pattern = rf"^\s+{re.escape(variant)}\b"
        if not re.search(pattern, body, re.MULTILINE):
            violations.append(
                f"check_runtimefn_deprecation: WARN SpecOp-mapped variant {variant} "
                f"not found in RuntimeFn enum (already removed?)"
            )
    return violations


def main():
    violations = []
    violations.extend(check_runtimefn_enum())
    violations.extend(check_emission_order())

    for v in violations:
        print(v, file=sys.stderr)

    if any("ERROR" in v for v in violations):
        sys.exit(1)
    print(
        f"check_runtimefn_deprecation: OK ({len(DEPRECATED_VARIANTS)} tracked, "
        f"{len([v for v in violations if 'migrating' in v])} in emission_order)",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
