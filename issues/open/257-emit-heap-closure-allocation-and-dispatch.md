---
id: 257
title: "Emit heap closure allocation and dispatch"
type: feature
area: backend
class: implementation-ready
priority: P1
depends_on: [256]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement backend support for the heap closure IR slice from issue 256.

Problem: The backend currently emits `LoweredExpr::ArrowFn` as an opaque numeric
token and only supports direct `FunctionCallKind::User` calls, so returned
closure values cannot be allocated or invoked.

## Current failure

After issue 256 introduces heap closure IR, backend validation/emission must not
silently treat closure values as numbers or generic objects.

## Desired final state

The backend allocates a closure payload matching `docs/14-runtime-abi.md`, roots
capture values during allocation, and dispatches supported heap closure calls by
loading `code_id` and capture slots.

## Scope

In scope:

- [ ] Emit closure object allocation with `OBJECT_TAG`, closure sentinel,
      `code_id`, `capture_count`, reserved flags, and capture slots.
- [ ] Root every evaluated capture across `$alloc_heap`.
- [ ] Emit a supported closure-call dispatch path for the arity used by the
      returned immutable closure fixture.
- [ ] Trap or compile-diagnose unsupported closure arities/code IDs rather than
      falling through to wrong calls.

Out of scope:

- Mutable environment cells.
- Function metadata/prototype properties on closure objects.
- `call_indirect` table migration unless it is the smallest safe dispatch
      implementation.
- `eval`, dynamic `Function`, generator, or async closure semantics.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `fixtures/core-semantics/`
- `crates/cli/tests/`

Do not touch:

- `crates/runtime-abi/` unless the closure sentinel/layout constants have been
      explicitly reviewed for ABI promotion.
- Issue 062f function metadata behavior.
- Issue 225 eval behavior.

## Acceptance criteria

- [ ] A returned immutable closure fixture compiles to WAT/WASM without using the
      opaque numeric `ArrowFn` token as the returned callable value.
- [ ] Calling the returned closure produces Node/iwasm-matching stdout for a
      fixture equivalent to `makeAdder(4)(5) -> 9`.
- [ ] Unsupported closure dispatch forms produce an issue-linked diagnostic or
      explicit runtime trap with a follow-up issue, not a wrong direct call.
- [ ] Emitted WAT keeps capture values rooted while allocating the closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(closure) or test(function) or test(node_diff)'
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 94 --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected unless the ABI contract changes

Current state:

- [ ] updated: `current-state.md`

Follow-up issues:

- [ ] none

## Notes

Start with generated switch-style dispatch over known closure `FuncId` values if
that is smaller than introducing wasm function tables.

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

- none
