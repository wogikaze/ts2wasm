---
name: m10-stream-b
description: Use when implementing M10 Stream B built-in function work for String, Array, Object, Math, JSON runtime functions, builtin resolution, WAT emission, and fixtures in ts2wasm.
---

# Stream B: Built-in Functions (Core Methods)

## Goal
Implement runtime functions for String, Array, Object, Math, and JSON to enable practical TypeScript/JavaScript.

## Scope (1-2 hour window, focused subset)

Implement 15-20 highest-impact built-in methods. Prioritize by usage frequency:
1. String methods (5): charAt, substring, slice, indexOf, split
2. Array methods (6): push, pop, slice, concat, join, reverse
3. Object statics (3): Object.keys, Object.values, Object.entries
4. Math functions (4): floor, ceil, round, abs, max, min (select 4)
5. JSON (2): JSON.stringify, JSON.parse (basic)

These are **not** method lookups; they are direct builtin resolver results (existing architecture in crates/cli/src/lib.rs near line ~600).

## Implementation strategy

### Phase 1: IR & Backend planning (10 min)
1. Extend RuntimeFn enum: StringCharAt, StringSubstring, StringSlice, StringIndexOf, StringSplit, ArrayPush, ArrayPop, ArraySlice, ArrayConcat, ArrayJoin, ArrayReverse, ObjectKeys, ObjectValues, ObjectEntries, MathFloor, MathCeil, MathRound, MathAbs, MathMax, MathMin, JsonStringify, JsonParse
2. Add entries to RuntimeLinkPlan (order doesn't matter for now; we'll optimize later)
3. Add each to capability_manifest function list

### Phase 2: WAT Emission (45 min)

For each RuntimeFn, implement emit function in WatEmitter:
- **StringCharAt**(s: i32, idx: i32) → i32: return char at idx as single-char string value or empty
- **StringSubstring**(s: i32, start: i32, end: i32) → i32: slice string heap object
- **StringSlice**(s: i32, start: i32, end: i32) → i32: ES slice semantics (negative indices)
- **StringIndexOf**(haystack: i32, needle: i32) → i32: return first position or -1
- **StringSplit**(s: i32, sep: i32) → i32: return array of strings
- **ArrayPush**(arr: i32, val: i32) → i32: append, return new length
- **ArrayPop**(arr: i32) → i32: remove last, return value
- **ArraySlice**(arr: i32, start: i32, end: i32) → i32: new array
- **ArrayConcat**(a: i32, b: i32) → i32: concatenate arrays
- **ArrayJoin**(arr: i32, sep: i32) → i32: stringify array with separator
- **ArrayReverse**(arr: i32) → i32: reverse in-place? or return new array (decide)
- **ObjectKeys**(obj: i32) → i32: return array of property names
- **ObjectValues**(obj: i32) → i32: return array of values
- **ObjectEntries**(obj: i32) → i32: return array of [key, value] pairs
- **MathFloor/Ceil/Round/Abs/Max/Min**: integer arithmetic WAT (no floating point needed yet)
- **JsonStringify**(val: i32) → i32: convert value to JSON string
- **JsonParse**(str: i32) → i32: parse JSON string to value (basic)

Implementation pattern (example: MathFloor):
```rust
fn emit_math_floor(&self, wat: &mut String) {
    wat.push_str(&format!(
        r#"
  (func $math_floor (param $v i32) (result i32)
    (if (result i32)
      (i32.eq (i32.and (local.get $v) (i32.const 7)) (i32.const {}))
      (then (local.get $v))  ;; already encoded number
      (else (i32.const {}))))  ;; not a number, return undefined
"#,
        ValueTag::NUMBER,
        ValueTag::UNDEFINED,
    ));
}
```

### Phase 3: Expression Emitter (15 min)

Update expr_emit.rs to detect built-in method calls:
- Pattern: `Expr::BinaryOp(BinaryOp::Dot, expr, name)` when name is recognized
- Map to RuntimeFn call
- Generate WAT: `(call $method_name args...)`

Example detection:
```rust
// In emit_expr, detect patterns like "str.charAt(0)"
match expr {
    Expr::MethodCall(obj, method, args) if method == "charAt" => {
        // emit: (call $string_char_at obj args[0])
    }
    ...
}
```

### Phase 4: Tests (10 min)
1. Unit tests: each RuntimeFn emits valid WAT (syntactic check only)
2. Integration tests: small fixtures like:
   - `let s = "hello"; console.log(s.charAt(0));` → "h"
   - `let a = [1,2,3]; console.log(a.push(4));` → 4
   - `let o = {x:1}; console.log(Object.keys(o));` → ["x"]
3. No performance tests yet (deferred to Stream H)

## Output

**Commits** (organize by logical boundaries):
1. `ir: extend RuntimeFn with string methods (charAt, substring, slice, indexOf, split)`
2. `ir: extend RuntimeFn with array methods (push, pop, slice, concat, join, reverse)`
3. `ir: extend RuntimeFn with object statics (keys, values, entries)`
4. `ir: extend RuntimeFn with math functions (floor, ceil, round, abs, max, min)`
5. `ir: extend RuntimeFn with JSON functions (stringify, parse)`
6. `backend: emit string method runtime functions`
7. `backend: emit array method runtime functions`
8. `backend: emit object static runtime functions`
9. `backend: emit math runtime functions`
10. `backend: emit JSON runtime functions`
11. `backend: wire builtin method calls through expr_emit`
12. `tests: add built-in method integration tests`

**Tests added**:
- `crates/cli/tests/m6_builtin_methods.rs` (15-20 fixture tests)
- Fixture files: `fixtures/m6/string-*.ts`, `fixtures/m6/array-*.ts`, `fixtures/m6/object-*.ts`, `fixtures/m6/json-*.ts`

**DiagCode impact**:
- Expect reduction in `UnresolvedFunction` for recognized methods
- New fixtures should show `pass` for basic cases

**Coverage matrix delta**:
- test262 `executed` count likely unchanged (many tests require features still missing)
- `unsupported` may decrease slightly for basic string/array/object operations
- `fail` may appear for incorrect implementations (acceptable for first pass)

## Validation before commit

```bash
cargo fmt --all --check
cargo test -q
cargo test -q --test m6_builtin_methods
# Spot check: verify JSON.stringify and JSON.parse produce reasonable output
./target/debug/ts2wasm-cli build fixtures/m6/json-basic.ts -o /tmp/t.wasm
iwasm /tmp/t.wasm
```

## Gatekeeper checklist

✓ Each RuntimeFn has exactly one emit function
✓ No string literals for WAT operators (all use ValueTag constants)
✓ All method calls lowered through expr_emit (not embedded in parser)
✓ JSON.parse is "basic" (only handles literals, not nested structures initially)
✓ String.split/Array.concat produce correct heap objects
✓ No floating-point (Math.floor works on encoded integers only)
✓ Tests have expected outputs (run against Node reference)

## Design decisions

1. **Array mutation**: push/pop modify in-place; slice/concat create new arrays (ES semantics)
2. **Object.keys** order: insertion order (simple to implement with object property list)
3. **JSON.parse** scope: only literals (numbers, strings, bools, arrays, objects) in Phase 1; no nested expressions
4. **String methods** return strings; non-string input → convert to string (loose type coercion)
5. **Math functions**: integer-only (floating-point is Stream H concern)

## References

- Current RuntimeFn enum: `crates/cli/src/backend/runtime_fn.rs`
- Current WatEmitter: `crates/cli/src/backend/runtime_builder.rs`
- Current expr_emit: `crates/cli/src/backend/expr_emit.rs`
- Existing string/array test fixtures: `fixtures/m5/`
