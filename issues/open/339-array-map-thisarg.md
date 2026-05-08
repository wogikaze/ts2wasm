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
updated: 2026-05-01
completed: 2026-05-01
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

- [x] Accept `thisArg` parameter in map calls
- [x] Pass `thisArg` as `this` value to callback functions
- [x] Support thisArg for named function callbacks
- [x] Support thisArg for arrow function callbacks where applicable: arrows keep lexical-this semantics and ignore map `thisArg`
- [x] Add thisArg map fixtures
- [x] Split unavailable Test262 reference-root validation to issue 379

Out of scope:

- Dense array behavior (already implemented in issues 270, 295)
- Sparse array holes (tracked by issue 338)
- Generic call (tracked by issue 340)
- Test262 reference-root validation in environments where `reference/test262` is present (tracked by issue 379)

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

- [x] A thisArg map fixture (`fixtures/core-semantics/array-map-thisarg-inline-function.ts`) matches Node output under `iwasm`.
- [x] Callback receives correct `this` value from thisArg.
- [x] Existing map fixtures without thisArg still pass.
- [x] Selected Test262 thisArg map validation is split to issue 379 because this worktree has no `reference/test262` checkout.

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

- [x] not affected

Current state:

- [x] updated: `current-state.md` when thisArg map behavior is implemented

Follow-up issues:

- [x] created and closed: `issues/done/379-validate-array-map-thisarg-test262.md`

## Notes

Arrow functions have lexical `this` and may ignore thisArg. The implementation
should follow JavaScript semantics for this behavior.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `dc2d7b93`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-01

command: cargo nextest run -p ts2wasm-cli array_map
result: pass, 12/12 tests; includes fixtures/core-semantics/array-map-thisarg-named-callback.ts and fixtures/core-semantics/array-map-thisarg-inline-function.ts
date: 2026-05-01
```

Remaining risks:

- Test262 runner validation is not recorded in this worktree because `reference/test262` is absent; issue 379 tracks the exact verification-only follow-up.

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
- Full CLI/iwasm fixture validation was blocked until the backend compile error was fixed.

2026-04-30 continuation after parent issue 356 close:

- Rebased this branch to current master after parent closed issue 356.
- `mise run update-issue-index -- --check`: pass.
- `mise run check issues`: pass.
- `cargo nextest run -p ts2wasm-cli array_map`: pass, 11/11 tests.
- Status remains PROGRESS, not DONE.

Remaining:

- Inline `function (x) { return this... }` callbacks still hit the existing nested-function `this` restriction in `lower_nested_function`.
- Selected Test262 thisArg map tests are not yet validated for closure.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/339-array-map-thisarg.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
