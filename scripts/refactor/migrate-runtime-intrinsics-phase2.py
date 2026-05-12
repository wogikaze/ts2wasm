#!/usr/bin/env python3
"""Phase 2: Handle remaining runtime_fn patterns in expr.rs and extra.rs.

After running phase 1 (which handled literal `"Foo".to_owned()` cases),
this script handles:
  1. Binary op match for BigInt (5 blocks)
  2. Date UTC getter match
  3. BigInt runtime_fn_name -> bigint_runtime_fn_intrinsic
  4. $instanceof special case
  5. format!("Array{}{}...") -> match
  6. format!("{class_name}New") -> match
  7. if/else for search/RegExpMatch, indexOf/Includes
  8. Method call on string objects
  9. resolve_method_to_runtime_fn -> now returns RuntimeIntrinsic
"""

import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent

BINARY_OP_ARMS = {
    # First block (BigIntAdd, BigIntSub)
    'first': {
        'intros': [
            'let runtime_fn = match op {',
            '// BinaryOp::Add => "BigIntAdd",',
        ],
        'arms': [
            'BinaryOp::Add => RuntimeIntrinsic::BigIntAdd,',
            'BinaryOp::Sub => RuntimeIntrinsic::BigIntSub,',
        ],
        'outro': '}',
    },
}

def process_expr_rs():
    path = REPO_ROOT / "crates/ir/src/lowered/resolver/expr.rs"
    text = path.read_text()
    original = text
    changes = 0

    # 1. Change bigint_runtime_fn_name to bigint_runtime_fn_intrinsic
    text = text.replace(
        'crate::builtin_resolver::bigint_runtime_fn_name',
        'bigint_runtime_fn_intrinsic'
    )

    # 2. Change resolve_method_to_runtime_fn usage - it now returns RuntimeIntrinsic
    # Callers use `runtime_fn,` directly with the value
    # We need: `let runtime_fn =` → `let intrinsic =` for the BigInt and Date patterns
    # then remove .to_owned() from runtime_fn: runtime_fn.to_owned()

    # 3. Handle let runtime_fn = match op { ... } blocks
    # We need to change:
    #   let runtime_fn = match op {
    #       BinaryOp::BigIntAdd => "BigIntAdd",
    #       ...
    #   };
    #   ...
    #   runtime_fn: runtime_fn.to_owned(),
    # To:
    #   let intrinsic = match op {
    #       BinaryOp::BigIntAdd => RuntimeIntrinsic::BigIntAdd,
    #       ...
    #   };
    #   ...
    #   intrinsic: intrinsic,

    # Replace all `runtime_fn: runtime_fn.to_owned(),` → `intrinsic: intrinsic,`
    text = re.sub(
        r'runtime_fn:\s*runtime_fn\.to_owned\(\),',
        'intrinsic: intrinsic,',
        text
    )

    # Replace `let runtime_fn = match op {` → `let intrinsic = match op {`
    text = text.replace(
        'let runtime_fn = match op {',
        'let intrinsic = match op {'
    )

    # Replace `let runtime_fn = match method.as_str() {` → `let intrinsic: RuntimeIntrinsic = match method.as_str() {`
    text = text.replace(
        'let runtime_fn = match method.as_str() {',
        'let intrinsic: RuntimeIntrinsic = match method.as_str() {'
    )

    # Replace the BinaryOp arm strings with RuntimeIntrinsic variants
    # Match patterns like: BinaryOp::BigIntAdd => "BigIntAdd",
    text = re.sub(
        r'(BinaryOp::(\w+))\s*=>\s*"(\2)"',
        r'\1 => RuntimeIntrinsic::\2',
        text
    )

    # Replace Date method string arms
    # Match: "getUTCDate" => "DateGetUtcDate",
    text = re.sub(
        r'"getUTC(Date|Day|FullYear|Hours|Milliseconds|Minutes|Month|Seconds)"\s*=>\s*"DateGetUtc\1"',
        r'"getUTC\1" => RuntimeIntrinsic::DateGetUtc\1',
        text
    )

    # Replace `runtime_fn: runtime_fn.to_owned()` (the variable ref)
    text = text.replace(
        'runtime_fn: runtime_fn.to_owned()',
        'intrinsic: intrinsic'
    )

    # .to_owned() on return from resolve_method_to_runtime_fn → just use directly
    text = text.replace(
        'runtime_fn: runtime_fn.to_owned()',
        'intrinsic: intrinsic'
    )

    # Replace `runtime_fn,` (used as value from a variable) in RuntimeCall
    text = re.sub(
        r'RuntimeCall \{\s*runtime_fn,',
        'RuntimeCall { intrinsic: runtime_fn,',
        text
    )

    # The format! pattern for Array methods — replace with match
    # Find the block that does format!("Array{}{}", ...)
    old_format = 'runtime_fn: format!("Array{}{}", method[0..1].to_uppercase(), &method[1..])'
    new_match = 'intrinsic: match method.as_str() { "find" => RuntimeIntrinsic::ArrayFind, "findIndex" => RuntimeIntrinsic::ArrayFindIndex, "findLast" => RuntimeIntrinsic::ArrayFindLast, "findLastIndex" => RuntimeIntrinsic::ArrayFindLastIndex, "filter" => RuntimeIntrinsic::ArrayFilter, "every" => RuntimeIntrinsic::ArrayEvery, "some" => RuntimeIntrinsic::ArraySome, _ => unreachable!() }'

    # But format! returns a String, the replacement is a RuntimeIntrinsic.
    # Remove the .to_owned() from it.
    text = text.replace(old_format, new_match)

    # Handle format!("{class_name}New")
    text = text.replace(
        'runtime_fn: format!("{class_name}New"),',
        'intrinsic: match class_name.as_str() { "Map" => RuntimeIntrinsic::MapNew, "Set" => RuntimeIntrinsic::SetNew, "WeakMap" => RuntimeIntrinsic::WeakMapNew, "WeakSet" => RuntimeIntrinsic::WeakSetNew, _ => unreachable!() },'
    )

    # Handle inline conditionals for search/RegExpMatch
    # runtime_fn: if method == "search" { "RegExpSearch".to_owned() } else { "RegExpMatch".to_owned() },
    text = re.sub(
        r'runtime_fn:\s*if\s+method\s*==\s*"search"\s*\{\s*"(\w+)"\s*\.\s*to_owned\s*\(\s*\)\s*\}\s*else\s*\{\s*"(\w+)"\s*\.\s*to_owned\s*\(\s*\)\s*\}',
        r'intrinsic: if method == "search" { RuntimeIntrinsic::\1 } else { RuntimeIntrinsic::\2 }',
        text
    )

    # Handle inline conditionals for indexOf/ArrayIncludes
    # runtime_fn: if method == "indexOf" { "ArrayIndexOf".to_owned() } else { "ArrayIncludes".to_owned() },
    text = re.sub(
        r'runtime_fn:\s*if\s+method\s*==\s*"indexOf"\s*\{\s*"(\w+)"\s*\.\s*to_owned\s*\(\s*\)\s*\}\s*else\s*\{\s*"(\w+)"\s*\.\s*to_owned\s*\(\s*\)\s*\}',
        r'intrinsic: if method == "indexOf" { RuntimeIntrinsic::\1 } else { RuntimeIntrinsic::\2 }',
        text
    )

    # Handle "$instanceof".to_string() → RuntimeIntrinsic::InstanceOf
    text = text.replace(
        'runtime_fn: "$instanceof".to_string()',
        'intrinsic: RuntimeIntrinsic::InstanceOf,'
    )

    # Handle var runtime_fn = method match pattern
    # Replace runtime_fn: runtime_fn.to_owned() → intrinsic: intrinsic
    text = re.sub(
        r'runtime_fn:\s*runtime_fn\.to_owned\(\)',
        'intrinsic: intrinsic',
        text
    )

    # Handle remaining runtime_fn: runtime_fn (variable ref)
    text = text.replace(
        'runtime_fn: runtime_fn,',
        'intrinsic: runtime_fn,'
    )

    # Handle `if let Some(runtime_fn)` → `if let Some(intrinsic)`
    text = re.sub(
        r'if let Some\(runtime_fn\)',
        'if let Some(intrinsic)',
        text
    )

    # Also handle && let Some(runtime_fn) patterns
    text = re.sub(
        r'\&\& let Some\(runtime_fn\)',
        '&& let Some(intrinsic)',
        text
    )

    # Replace `runtime_fn: runtime_fn.to_owned(),` → `intrinsic: intrinsic,`
    # (for cases where the old script already changed runtime_fn: but var is still runtime_fn)
    text = re.sub(
        r'runtime_fn:\s*(runtime_fn\.to_owned\(\))',
        r'intrinsic: \1',
        text
    )

    # Remove .to_owned() from RuntimeIntrinsic values
    # runtime_fn: SomeFunction().to_owned() → intrinsic: SomeFunction()
    # But only when the function returns RuntimeIntrinsic
    text = text.replace(
        'intrinsic: resolve_method_to_runtime_fn(object, method).to_owned(),',
        'intrinsic: resolve_method_to_runtime_fn(object, method),'
    )

    # Handle collection_method_runtime_fn_arg(method).to_owned() → now returns RuntimeIntrinsic, no .to_owned()
    text = text.replace(
        'collection_method_runtime_fn_arg(method).to_owned()',
        'collection_method_runtime_fn_arg(method)'
    )
    text = text.replace(
        'collection_method_runtime_fn(class_name, method).to_owned()',
        'collection_method_runtime_fn(class_name, method)'
    )

    # Handle `runtime_fn: runtime_fn.clone(),` → `intrinsic: intrinsic.clone(),`
    # These are cases where one RuntimeCall feeds into another
    text = text.replace(
        'runtime_fn: runtime_fn.clone(),',
        'intrinsic: intrinsic.clone(),'
    )

    # Handle inline if/else for conditionals
    text = text.replace(
        "runtime_fn: if method == \"indexOf\" {",
        "intrinsic: if method == \"indexOf\" {",
    )
    text = text.replace(
        "runtime_fn: if method == \"search\" {",
        "intrinsic: if method == \"search\" {",
    )

    # Handle `runtime_fn,` in RuntimeCall destructuring (irreducible to pattern)
    text = text.replace(
        'RuntimeCall { runtime_fn,',
        'RuntimeCall { intrinsic,'
    )

    if text != original:
        path.write_text(text)
        print(f"  expr.rs: updated")
        return True
    print(f"  expr.rs: no changes")
    return False


def process_extra_rs():
    path = REPO_ROOT / "crates/ir/src/lowered/resolver/extra.rs"
    text = path.read_text()
    original = text
    changes = 0

    # Replace bigint_runtime_fn_name with bigint_runtime_fn_intrinsic
    text = text.replace(
        'bigint_runtime_fn_name(name).is_some()',
        'bigint_runtime_fn_intrinsic(name).is_some()'
    )
    text = text.replace(
        'bigint_runtime_fn_name(method).is_some()',
        'bigint_runtime_fn_intrinsic(method).is_some()'
    )

    # Handle `if let Some(runtime_fn)` → `if let Some(intrinsic)`
    text = re.sub(
        r'if let Some\(runtime_fn\)',
        'if let Some(intrinsic)',
        text
    )

    # Handle `runtime_fn: runtime_fn.clone(),` → `intrinsic: intrinsic.clone(),`
    text = text.replace(
        'runtime_fn: runtime_fn.clone(),',
        'intrinsic: intrinsic.clone(),'
    )

    # Handle RuntimeCall { runtime_fn, .. } pattern match
    text = text.replace(
        'RuntimeCall { runtime_fn,',
        'RuntimeCall { intrinsic,'
    )

    if text != original:
        path.write_text(text)
        print(f"  extra.rs: updated")
        return True
    print(f"  extra.rs: no changes")
    return False


def process_mod_rs():
    path = REPO_ROOT / "crates/ir/src/lowered/resolver/mod.rs"
    text = path.read_text()
    original = text

    # Handle `let runtime_fn = resolve_method_to_runtime_fn` calls
    text = re.sub(
        r'let runtime_fn = resolve_method_to_runtime_fn',
        'let intrinsic = resolve_method_to_runtime_fn',
        text
    )

    # Handle if let Some(runtime_fn) = collection_method_runtime_fn
    text = re.sub(
        r'if let Some\(runtime_fn\) = collection_method_runtime_fn',
        'if let Some(intrinsic) = collection_method_runtime_fn',
        text
    )

    # Handle runtime_fn, in RuntimeCall
    text = text.replace(
        'runtime_fn,',
        'intrinsic,'
    )

    if text != original:
        path.write_text(text)
        print(f"  mod.rs: updated")
        return True
    print(f"  mod.rs: no changes")
    return False


def main():
    r1 = process_expr_rs()
    r2 = process_extra_rs()
    r3 = process_mod_rs()
    if not (r1 or r2 or r3):
        print("No changes needed")


if __name__ == "__main__":
    main()
