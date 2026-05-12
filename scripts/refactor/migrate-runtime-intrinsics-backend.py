#!/usr/bin/env python3
"""Fix backend crate for RuntimeIntrinsic migration.

Handles:
1. Add `use ts2wasm_ir::RuntimeIntrinsic;` import
2. Replace `runtime_fn:` field with `intrinsic:` in RuntimeCall construction
3. Replace `runtime_fn_from_name(runtime_fn)` with `runtime_fn_from_name(runtime_fn.name())`
   where runtime_fn is now RuntimeIntrinsic
4. Fix pattern matches that still use `runtime_fn` variable name
"""

import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
BACKEND_SRC = REPO_ROOT / "crates/backend-wasm/src"

def fix_file(path):
    text = path.read_text()
    original = text
    rel = path.relative_to(REPO_ROOT)

    # 1. Add import if file uses RuntimeIntrinsic but doesn't import it
    if 'RuntimeIntrinsic' in text and 'use ts2wasm_ir::RuntimeIntrinsic' not in text:
        # Find the imports section and add after last ts2wasm_ir import
        # Or add as a new import line
        import_pattern = r'^(use ts2wasm_ir::.*;)'
        if re.search(import_pattern, text, re.MULTILINE):
            text = re.sub(
                import_pattern,
                r'\1\nuse ts2wasm_ir::RuntimeIntrinsic;',
                text,
                count=1
            )
        else:
            # Add after existing ts2wasm_ir related imports
            text = re.sub(
                r'(use ts2wasm_ir::LoweredStmt;)',
                r'\1\nuse ts2wasm_ir::RuntimeIntrinsic;',
                text
            )

    # 2. Replace `runtime_fn: "Foo".to_owned()` with `intrinsic: RuntimeIntrinsic::Foo,`
    text = re.sub(
        r'runtime_fn:\s*"(\w+)"\s*\.\s*to_owned\s*\(\s*\)',
        r'intrinsic: RuntimeIntrinsic::\1',
        text
    )

    # 3. Replace `runtime_fn: runtime_fn.` → `intrinsic: runtime_fn.`
    text = re.sub(
        r'runtime_fn:\s*runtime_fn\.to_owned\(\)',
        r'intrinsic: runtime_fn,',
        text
    )

    # 4. Fix runtime_fn_from_name patterns: when runtime_fn is a variable of type RuntimeIntrinsic
    # we need to call .name() on it
    text = re.sub(
        r'super::runtime_fn::runtime_fn_from_name\(runtime_fn\)',
        r'super::runtime_fn::runtime_fn_from_name(runtime_fn.name())',
        text
    )

    # 5. Fix pattern-match variable: In match blocks where the pattern is
    # `LoweredExpr::RuntimeCall { runtime_fn, .. }` → `{ intrinsic, .. }`
    text = re.sub(
        r'RuntimeCall\s*\{\s*runtime_fn\s*,',
        r'RuntimeCall { intrinsic,',
        text
    )

    # 6. Fix variable references: `runtime_fn` (as a let binding) → `intrinsic`
    # But only in expressions where it's clearly the RuntimeFn variable
    text = re.sub(
        r'let runtime_fn = ',
        r'let intrinsic: RuntimeIntrinsic = ',
        text
    )

    # 7. Fix string comparisons: `if runtime_fn == "Foo"` → `if intrinsic == RuntimeIntrinsic::Foo`
    text = re.sub(
        r'if runtime_fn\s*==\s*"(\w+)"',
        r'if intrinsic == RuntimeIntrinsic::\1',
        text
    )

    # 8. Fix `runtime_fn.as_str()` → `intrinsic.name()`
    text = text.replace(
        'runtime_fn.as_str()',
        'intrinsic.name()'
    )

    # 9. Fix `runtime_fn_from_name(runtime_fn)` where runtime_fn is RuntimeIntrinsic
    text = re.sub(
        r'runtime_fn_from_name\(runtime_fn\)',
        r'runtime_fn_from_name(runtime_fn.name())',
        text
    )

    # 10. Fix: `runtime_fn` variable reference in RuntimeCall where intrinsic is the field
    text = re.sub(
        r'intrinsic:\s*runtime_fn,',
        r'intrinsic: runtime_fn,',
        text
    )

    if text != original:
        path.write_text(text)
        return True
    return False


def main():
    changed = []
    for path in sorted(BACKEND_SRC.rglob("*.rs")):
        if fix_file(path):
            changed.append(path.relative_to(REPO_ROOT))
    if changed:
        print(f"Modified {len(changed)} files:")
        for c in changed:
            print(f"  {c}")
    else:
        print("No changes needed")


if __name__ == "__main__":
    main()
