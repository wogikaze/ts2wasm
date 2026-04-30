---
id: 339
title: "Callback thisArg for Array.prototype.map"
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

Implement callback `thisArg` support for `Array.prototype.map`. The second
argument to `map` should be used as the `this` value when calling the callback
function.

## Problem

Problem: current `Array.prototype.map` implementation does not support the
`thisArg` parameter. Code like `array.map(callback, thisArg)` is not handled
correctly.

## Desired final state

`Array.prototype.map` correctly supports the `thisArg` parameter:
- The second argument to `map` is passed as the `this` value to the callback
- Callbacks can access `this` inside the function body
- Matching Node behavior for thisArg

## Scope

In scope:

- [ ] Accept `thisArg` parameter in map calls
- [ ] Pass `thisArg` as `this` value to callback functions
- [ ] Support thisArg for named function callbacks
- [ ] Support thisArg for arrow function callbacks (where applicable)
- [ ] Add thisArg map fixtures
- [ ] Validate with Test262 thisArg map tests

Out of scope:

- Dense array behavior (already implemented in issues 270, 295)
- Sparse array holes (tracked by issue 338)
- Generic call (tracked by issue 340)

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- Dense array map implementation
- Sparse array implementation
- Generic call implementation

## Acceptance criteria

- [ ] A thisArg map fixture (e.g., `array.map(function(x) { return this.multiplier * x; }, {multiplier: 2})`) matches Node output under `iwasm`.
- [ ] Callback receives correct `this` value from thisArg.
- [ ] Existing map fixtures without thisArg still pass.
- [ ] Selected Test262 thisArg map tests pass.

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
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` when thisArg map behavior is implemented

## Notes

Arrow functions have lexical `this` and may ignore thisArg. The implementation
should follow JavaScript semantics for this behavior.

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

- Arrow function lexical `this` may complicate thisArg implementation

## Progress evidence

2026-04-30 child-338-340:

- Implemented static array-literal map lowering for top-level named function callbacks with `thisArg` receiver binding.
- Added `fixtures/core-semantics/array-map-thisarg-named-callback.ts`.
- Also permits arrow callbacks to receive `value`, `index`, and `array`; map `thisArg` is ignored for arrows per JS lexical-this semantics.
- `cargo fmt --all --check`: pass.
- `cargo check -p ts2wasm-ir`: pass.
- `cargo nextest run -p ts2wasm-cli array_map`: blocked before running tests by pre-existing compile error in unmodified `crates/backend-wasm/src/expr_emit.rs` (`array_push_grow_linear_growth_threshold` name mismatch and unused format argument).

Remaining:

- Not DONE. Inline `function (x) { return this... }` callbacks still hit the existing nested-function `this` restriction in `lower_nested_function`.
- Full CLI/iwasm fixture validation is blocked until the backend compile error is fixed.
