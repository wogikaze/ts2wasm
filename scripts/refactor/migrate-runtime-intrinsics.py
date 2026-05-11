#!/usr/bin/env python3
"""Migrate RuntimeCall { runtime_fn: String } to RuntimeCall { intrinsic: RuntimeIntrinsic }.

Performs mechanical replacements:
1. `runtime_fn: "Foo".to_owned()` → `intrinsic: RuntimeIntrinsic::Foo`
2. `runtime_fn: runtime_fn` (variable) → `intrinsic: runtime_fn` then rename var
3. Field access: `.runtime_fn` → `.intrinsic.name()` in backend string contexts
4. `if runtime_fn == "Foo"` → `if intrinsic == RuntimeIntrinsic::Foo`
5. `LoweredExpr::RuntimeCall { runtime_fn, .. }` pattern → `{ intrinsic, .. }`

Usage:
  python3 scripts/refactor/migrate-runtime-intrinsics.py [--ir-only]
"""

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent

IR_FILES = sorted((REPO_ROOT / "crates/ir/src/lowered/resolver").rglob("*.rs"))
IR_FILES.append(REPO_ROOT / "crates/ir/src/lowered/resolver/mod.rs")
# extra.rs
IR_FILES.append(REPO_ROOT / "crates/ir/src/lowered/resolver/extra.rs")
# program_builtins.rs is in lowered/ itself
IR_FILES.append(REPO_ROOT / "crates/ir/src/lowered/program_builtins.rs")
# validate.rs
IR_FILES.append(REPO_ROOT / "crates/ir/src/lowered/validate.rs")

BACKEND_FILES = list((REPO_ROOT / "crates/backend-wasm/src").rglob("*.rs"))

def literal_replacement(line: str) -> str:
    """Replace `runtime_fn: "VariantName".to_owned()` with `intrinsic: RuntimeIntrinsic::VariantName`."""
    m = re.match(r'^(\s*)runtime_fn:\s*"(\w+)"\s*\.\s*to_owned\s*\(\s*\)\s*,?\s*$', line)
    if m:
        indent = m.group(1)
        name = m.group(2)
        return f"{indent}intrinsic: RuntimeIntrinsic::{name},"
    return None


def string_comparison_replacement(line: str) -> str:
    """Replace `runtime_fn == "Foo"` with `intrinsic == RuntimeIntrinsic::Foo`."""
    # Match patterns like: if runtime_fn == "Foo" or runtime_fn == "Foo"
    line = re.sub(
        r'if\s+runtime_fn\s*==\s*"(\w+)"',
        r'if intrinsic == RuntimeIntrinsic::\1',
        line
    )
    # Match: || runtime_fn == "Foo"
    line = re.sub(
        r'\|\|\s*runtime_fn\s*==\s*"(\w+)"',
        r'|| intrinsic == RuntimeIntrinsic::\1',
        line
    )
    # Match: if runtime_fn == "Foo" { with no space before {
    line = re.sub(
        r'if\s+runtime_fn\s*==\s*"(\w+)"\s*\{',
        r'if intrinsic == RuntimeIntrinsic::\1 {',
        line
    )
    # Match: (runtime_fn == "Foo" ... )
    line = re.sub(
        r'\(runtime_fn\s*==\s*"(\w+)"',
        r'(intrinsic == RuntimeIntrinsic::\1',
        line
    )
    # Simple equality check: runtime_fn == "Foo"
    line = re.sub(
        r'runtime_fn\s*==\s*"(\w+)"',
        r'intrinsic == RuntimeIntrinsic::\1',
        line
    )
    return line


def pattern_match_replacement(line: str) -> str:
    """Replace match patterns like `LoweredExpr::RuntimeCall { runtime_fn, .. }`."""
    # Match: LoweredExpr::RuntimeCall { runtime_fn, args, .. }
    line = re.sub(
        r'RuntimeCall\s*\{\s*runtime_fn\s*,',
        r'RuntimeCall { intrinsic,',
        line
    )
    # Match: LoweredExpr::RuntimeCall { runtime_fn, .. }
    line = re.sub(
        r'RuntimeCall\s*\{\s*runtime_fn\s*,\s*\.\.\s*\}',
        r'RuntimeCall { intrinsic, .. }',
        line
    )
    # Match: RuntimeCall { runtime_fn, .. }
    line = re.sub(
        r'RuntimeCall\s*\{\s*runtime_fn\s*,\s*args',
        r'RuntimeCall { intrinsic, args',
        line
    )
    return line


def runtime_fn_variable_replacement(line: str) -> str:
    """Replace `runtime_fn: runtime_fn,` (variable reference) with `intrinsic: runtime_fn,`.
    Then the variable needs to be typed as RuntimeIntrinsic.
    """
    line = line.replace("runtime_fn: runtime_fn,", "intrinsic: runtime_fn,")
    return line


def migrate_ir_file(path: Path) -> tuple[int, int]:
    """Migrate a single IR file. Returns (changes, lines) count."""
    text = path.read_text()
    original = text
    lines = text.split('\n')
    changed_lines = 0

    for i, line in enumerate(lines):
        # Pattern 1: literal replacements
        result = literal_replacement(line)
        if result is not None:
            if lines[i] != result:
                lines[i] = result
                changed_lines += 1
            continue

        # Pattern 2: variable reference
        if 'runtime_fn: runtime_fn,' in line:
            old = lines[i]
            lines[i] = runtime_fn_variable_replacement(lines[i])
            if lines[i] != old:
                changed_lines += 1
            continue

        # Pattern 3: pattern match destructuring
        new_line = pattern_match_replacement(line)
        if new_line != line:
            lines[i] = new_line
            changed_lines += 1
            continue

        # Pattern 4: string comparisons (in validate.rs mainly)
        new_line2 = string_comparison_replacement(line)
        if new_line2 != line:
            lines[i] = new_line2
            changed_lines += 1
            continue

        # Direct field access: .runtime_fn → .intrinsic.name() needs manual review

    new_text = '\n'.join(lines)
    if new_text != original:
        path.write_text(new_text)

    return changed_lines, len(lines)


def main():
    ir_only = "--ir-only" in sys.argv

    total_changes = 0
    for path in IR_FILES:
        if not path.exists():
            print(f"SKIP (not found): {path.relative_to(REPO_ROOT)}")
            continue
        rel = path.relative_to(REPO_ROOT)
        changes, total = migrate_ir_file(path)
        total_changes += changes
        if changes > 0:
            print(f"  {rel}: {changes} changes in {total} lines")

    if not ir_only:
        for path in BACKEND_FILES:
            rel = path.relative_to(REPO_ROOT)
            changes, total = migrate_ir_file(path)
            total_changes += changes
            if changes > 0:
                print(f"  {rel}: {changes} changes in {total} lines")

    print(f"\nTotal: {total_changes} changes across {len(IR_FILES)} IR files")
    if not ir_only:
        print(f"  and {len(BACKEND_FILES)} backend files")


if __name__ == "__main__":
    main()
