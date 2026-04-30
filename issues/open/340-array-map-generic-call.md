---
id: 340
title: "Generic call for Array.prototype.map (Array.prototype.map.call(...))"
type: feature
area: runtime/builtins
class: ready
priority: P2
depends_on: [334]
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Implement generic call semantics for `Array.prototype.map`, allowing
`Array.prototype.map.call(arrayLike, callback)` to work with array-like objects.

## Problem

Problem: current `Array.prototype.map` implementation does not support generic
call via `Array.prototype.map.call(...)`. Code like
`Array.prototype.map.call(arrayLike, callback)` is unsupported.

## Desired final state

`Array.prototype.map` supports generic call semantics:
- `Array.prototype.map.call(arrayLike, callback)` works with array-like objects
- Array-like objects with `length` property and indexed properties are handled
- Matching Node behavior for generic map calls

## Scope

In scope:

- [ ] Detect `Array.prototype.map.call(...)` pattern
- [ ] Handle array-like objects (objects with `length` property)
- [ ] Iterate using `length` property and indexed access
- [ ] Support generic call for named function callbacks
- [ ] Support generic call for arrow function callbacks
- [ ] Add generic call map fixtures
- [ ] Validate with Test262 generic call map tests (e.g., 15.4.4.19-2-19.js)

Out of scope:

- Dense array direct calls (already implemented in issues 270, 295)
- Sparse array holes (tracked by issue 338)
- thisArg (tracked by issue 339)
- Full iterator protocol

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- Dense array direct map implementation
- Sparse array implementation
- thisArg implementation

## Acceptance criteria

- [ ] A generic call map fixture (e.g., `Array.prototype.map.call({0: 1, 1: 2, length: 2}, x => x * 2)`) matches Node output under `iwasm`.
- [ ] The representative Test262 case `reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js` passes.
- [ ] Existing direct map fixtures still pass.
- [ ] Selected Test262 generic call map tests pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py update-issue-index --check
python scripts/manager.py check issues
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli array_map
cargo run -q -- build reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js -o /tmp/array-map-call.wasm --host-deny
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` when generic call map behavior is implemented

## Notes

Generic call requires handling array-like objects, which may need property
access via `PropertyGet` instead of direct array indexing.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- Array-like object handling may require broader property access changes

## Progress evidence

2026-04-30 child-338-340:

- Implemented static lowering for `Array.prototype.map.call(...)` when the receiver is an array literal or dense object literal with `"0"..."length - 1"` properties and numeric `length`.
- Added `fixtures/core-semantics/array-map-generic-call-object-literal.ts`.
- Promoted `fixtures/builtins-and-io/array-map-call-unsupported.ts` to a positive fixture path and extended it to check mapped values.
- `cargo fmt --all --check`: pass.
- `cargo check -p ts2wasm-ir`: pass.
- `cargo nextest run -p ts2wasm-cli array_map`: blocked before running tests by pre-existing compile error in unmodified `crates/backend-wasm/src/expr_emit.rs` (`array_push_grow_linear_growth_threshold` name mismatch and unused format argument).

Remaining:

- Not DONE. Generic map is currently static/dense only; arbitrary runtime array-like objects and sparse/missing indexed properties still require broader property iteration/runtime support.
- The requested representative Test262 path `reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-2-19.js` is absent in this worktree, so it could not be validated here.
