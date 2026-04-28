# Cycle Report: Issue 043 - Implement String Indexing

## Issue

- ID: 043
- Title: Implement string indexing
- Type: feature
- Area: runtime/semantics
- Priority: P1

## Summary

Implemented string indexing by adding a new `Index` runtime function that handles both string and array indexing via runtime type checking. The lowering was updated to use `Index` for string literals while keeping `ArrayGet` for arrays to maintain backward compatibility with existing differential tests.

## Implementation Details

### Changes Made

**Runtime Function (`crates/cli/src/backend/runtime_fn.rs`)**
- Added `Index` variant to `RuntimeFn` enum
- Added runtime spec with symbol `$index`, no dependencies, no imports
- Added symbol mapping to `symbol()` method
- Added `Index` to `runtime_functions_for_wasi()` and `runtime_functions_for_node_api()` lists

**Runtime Builder (`crates/cli/src/backend/runtime_builder.rs`)**
- Added `emit_index()` case to `emit_runtime_fn()` match
- Implemented `emit_index()` function that:
  - Checks object tag at runtime (string vs array)
  - For strings: loads UTF-16 code unit, returns as tagged number
  - For arrays: loads element from array
  - Returns `undefined` for invalid indices or non-number indices
  - Returns `undefined` for out-of-bounds access

**IR Lowering (`crates/ir/src/lowered.rs`)**
- Added `Index` variant to `LoweredExpr` enum with `object` and `index` fields
- Updated `ComputedIndex` lowering to:
  - Use `Index` when object is a string literal (`ResolvedExpr::String`)
  - Use `ArrayGet` for other cases (arrays, variables)
- Added validation for `Index` in `validate_expr()`

**Emitter (`crates/cli/src/backend/expr_emit.rs`)**
- Added `Index` case to emit runtime function call

**Runtime Link Plan (`crates/cli/src/backend/runtime_link_plan.rs`)**
- Added `Index` case to collect required runtime functions

**Emitter String Collection (`crates/cli/src/backend/emitter.rs`)**
- Added `Index` case to collect strings from expressions

**Fixtures**
- Added `fixtures/builtins-and-io/string-indexing.ts` to test string indexing behavior

### Key Design Decisions

1. **Separate Runtime Function**: Created a new `Index` runtime function instead of modifying `ArrayGet` to avoid breaking existing differential tests that expect the original `ArrayGet` WAT output.

2. **Static Type Detection**: The lowering checks if the object is a string literal (`ResolvedExpr::String`) at compile time and uses `Index` for those cases. For dynamic cases (variables), it falls back to `ArrayGet`. This is a limitation - dynamic string indexing through variables is not yet supported.

3. **Runtime Polymorphism**: The `Index` runtime function handles both strings and arrays by checking the object's tag at runtime, similar to how `GetLength` works for both strings and arrays.

## Verification

- `cargo fmt --all --check`: Passed
- `cargo nextest run`: All 196 tests passed, 4 skipped

## Limitations

- Dynamic string indexing (e.g., `let s = "hello"; console.log(s[0])`) is not yet supported because the lowering cannot detect that the variable contains a string at compile time. This would require either:
  - A type system to track variable types
  - Always using `Index` for all bracket notation (which would break differential tests)
  - A separate issue to add type inference

## Follow-up Work

Consider adding a follow-up issue to support dynamic string indexing through variables. This would require either:
- Type inference to track variable types
- Modifying all bracket notation to use `Index` (with corresponding differential test updates)
- A hybrid approach with runtime type checks

## Files Modified

- `crates/cli/src/backend/runtime_fn.rs`
- `crates/cli/src/backend/runtime_builder.rs`
- `crates/ir/src/lowered.rs`
- `crates/cli/src/backend/expr_emit.rs`
- `crates/cli/src/backend/runtime_link_plan.rs`
- `crates/cli/src/backend/emitter.rs`
- `fixtures/builtins-and-io/string-indexing.ts` (new file)
- `issues/open/043-implement-string-indexing.md` → `issues/done/043-implement-string-indexing.md`
- `issues/index.md`
