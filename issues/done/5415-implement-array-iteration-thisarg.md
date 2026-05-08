---
id: 5415
title: "W4: Implement Array iteration methods with thisArg correctness"
type: feature
area: runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Implement Array.prototype.forEach/find/filter/every/some/some with complete thisArg semantics and callback argument correctness to achieve Node/iwasm semantic parity.

## Problem

Array iteration methods (forEach, find, filter, every, some) currently pass build_smoke but may not handle all edge cases correctly: callback `thisArg`, sparse array hole skipping, callback argument arity (element, index, array), and receiver binding. This blocks test262 semantic_pass for array-builtin tests.

Problem: Array iteration methods lack semantic_diff parity for thisArg and edge cases.

## Current failure

```sh
# Build_smoke passes but semantic behavior may differ
# Array.prototype.forEach with thisArg object returns different results
```

## Desired final state

Array.prototype.forEach, find, filter, every, some have Node/iwasm semantic_diff parity including:
- Callback thisArg binding
- Sparse array hole skipping (already partially implemented)
- Correct callback arguments: (element, index, array)
- Mutation during iteration (spec-conformant behavior)
- Empty slot handling in sparse arrays

## Scope

In scope:

- [ ] Verify and fix callback `thisArg` binding in forEach/find/filter/every/some
- [ ] Verify sparse array hole skipping in all iteration methods
- [ ] Add semantic_diff fixtures for each method with thisArg object
- [ ] Add semantic_diff fixtures for sparse array iteration
- [ ] Add test file `crates/cli/tests/m6_array_iteration.rs`

Out of scope:

- Array.prototype.reduce/reduceRight (separate issue)
- Array.prototype.sort (separate issue)
- Array.prototype.flat/flatMap (separate issue)
- TypedArray iteration
- Iterator protocol ([Symbol.iterator])

## Affected paths

Expected:

- `crates/backend-wasm/src/runtime_arrays.rs` — fix iteration method WAT if needed
- `fixtures/builtins-and-io/array-foreach-thisarg.ts` — new fixture
- `fixtures/builtins-and-io/array-find-thisarg.ts` — new fixture
- `fixtures/builtins-and-io/array-filter-thisarg.ts` — new fixture
- `fixtures/builtins-and-io/array-sparse-iteration.ts` — new fixture
- `crates/cli/tests/m6_array_iteration.rs` — new test file

Do not touch:

- `crates/frontend/` — parser out of scope
- `crates/ir/` — IR out of scope
- `crates/backend-wasm/src/runtime_fn.rs` — no new RuntimeFn variants
- `crates/backend-wasm/src/runtime_fn_impl.rs` — no catalog changes
- `crates/backend-wasm/src/runtime_objects.rs`, `runtime_strings.rs`
- `crates/cli/tests/m6_builtin_methods.rs` — use separate test file

## Acceptance criteria

- [ ] `arr.forEach(callback, obj)` invokes callback with `this === obj`
- [ ] `arr.forEach(callback)` invokes callback with `this === undefined` (non-strict → Window, strict → undefined)
- [ ] Sparse array `[1, , 3].forEach(...)` skips the hole
- [ ] `[1, 2, 3].find(cb, obj)` and `filter`, `every`, `some` all accept thisArg
- [ ] All 4 new semantic_diff fixtures pass (Node output matches iwasm)

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -- m6_array_iteration
cargo nextest run -- m2_node_diff
```

Impacted commands:

```sh
# Verify individual fixtures
ts2wasm build fixtures/builtins-and-io/array-foreach-thisarg.ts
iwasm output.wasm > actual.txt
node fixtures/builtins-and-io/array-foreach-thisarg.ts > expected.txt
diff actual.txt expected.txt
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

- runtime_arrays.rs has existing forEach/find/filter/every/some WAT — the thisArg binding may need a WAT wrapping layer
- In WASM: the callback `thisArg` is passed as a separate JsValue; the WAT function needs to forward it
- Look at how map's thisArg is implemented for the pattern (map may already handle it)
- Create minimal focused fixtures — each fixture should test exactly one method + one edge case
