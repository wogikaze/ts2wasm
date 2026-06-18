#!/usr/bin/env python3
"""Check that no new RuntimeFn variants overlap with SpecOp equivalents.

This script:
  1. Reads the RuntimeFn enum from crates/runtime-catalog/src/runtime_fn.rs
  2. Checks a hardcoded list of "deprecatable" variants (those with SpecOp equivalents)
  3. Fails if a variant on the deprecation list is added to emission_order
     (meaning the new pipeline can't avoid it)

Usage:
  python scripts/check/check-runtimefn-deprecation.py
  python scripts/check/check-runtimefn-deprecation.py --migration-complete

Exit code:
  0 = no newly-added deprecated variants in active use
  1 = newly-added deprecated variant found in emission_order or checker parse failure
"""

import re
import sys
import subprocess
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
            f"check_runtimefn_deprecation: ERROR {variant} is in emission_order "
            f"but has a SpecOp equivalent — migrate new-path output away from RuntimeFn"
        )
    return violations


def check_added_deprecated_emission_order() -> list[str]:
    try:
        result = subprocess.run(
            ["git", "diff", "--unified=0", "HEAD", "--", "crates/runtime-catalog/src/runtime_fn.rs"],
            capture_output=True,
            text=True,
            cwd=REPO_ROOT,
            timeout=10,
        )
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return ["check_runtimefn_deprecation: ERROR git unavailable; cannot verify RuntimeFn diff"]
    if result.returncode != 0:
        return ["check_runtimefn_deprecation: ERROR git diff failed; cannot verify RuntimeFn diff"]

    violations = []
    for line in result.stdout.splitlines():
        if not line.startswith("+") or line.startswith("+++"):
            continue
        for variant in sorted(DEPRECATED_VARIANTS):
            if re.search(rf"\bSelf::{re.escape(variant)}\b", line):
                violations.append(
                    f"check_runtimefn_deprecation: ERROR newly added deprecated RuntimeFn::{variant} "
                    "in emission_order — use SpecOp"
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
            # Already removed from RuntimeFn is acceptable.
            continue
    return violations


def main():
    args = sys.argv[1:]
    migration_complete = "--migration-complete" in args
    if "-h" in args or "--help" in args:
        print(__doc__.strip())
        return
    unknown = [arg for arg in args if arg not in {"--migration-complete"}]
    if unknown:
        print(f"check_runtimefn_deprecation: unknown arguments: {' '.join(unknown)}", file=sys.stderr)
        sys.exit(2)

    violations = []
    violations.extend(check_runtimefn_enum())
    if migration_complete:
        violations.extend(check_emission_order())
    else:
        violations.extend(check_added_deprecated_emission_order())

    for v in violations:
        print(v, file=sys.stderr)

    if violations:
        print(
            f"check_runtimefn_deprecation: FAILED ({len(violations)} errors)",
            file=sys.stderr,
        )
        sys.exit(1)
    print(
        f"check_runtimefn_deprecation: OK ({len(DEPRECATED_VARIANTS)} tracked, "
        f"{'migration complete' if migration_complete else 'no new deprecated emission_order entries'})",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
