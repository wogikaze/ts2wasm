---
id: 257
title: "Emit heap closure allocation and dispatch"
type: feature
area: backend
class: implementation-ready
priority: P1
status: done
depends_on: [256]
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
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

- [x] Emit closure object allocation with `OBJECT_TAG`, closure sentinel,
      `code_id`, `capture_count`, reserved flags, and capture slots.
- [x] Root every evaluated capture across `$alloc_heap`.
- [x] Emit a supported closure-call dispatch path for the arity used by the
      returned immutable closure fixture.
- [x] Trap or compile-diagnose unsupported closure arities/code IDs rather than
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

- [x] A returned immutable closure fixture compiles to WAT/WASM without using the
      opaque numeric `ArrowFn` token as the returned callable value.
- [x] Calling the returned closure produces Node/iwasm-matching stdout for a
      fixture equivalent to `makeAdder(4)(5) -> 9`.
- [x] Unsupported closure dispatch forms produce an issue-linked diagnostic or
      explicit runtime trap with a follow-up issue, not a wrong direct call.
- [x] Emitted WAT keeps capture values rooted while allocating the closure.

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

- [x] not affected unless the ABI contract changes

Current state:

- [x] updated: `current-state.md`

Follow-up issues:

- [x] none

## Notes

Start with generated switch-style dispatch over known closure `FuncId` values if
that is smaller than introducing wasm function tables.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `115d5cf74a9d19840303ff951463264529deb415`

Validation result:

```text
command:
cargo fmt --all --check
result: passed
date: 2026-04-29

command:
cargo nextest run -p ts2wasm-cli -E 'test(returned_ordinary_function_closure_fixtures_match_node_output_under_iwasm) or test(lowering_represents_known_heap_closure_local_call_explicitly) or test(validate_accepts_heap_closure_creation_for_backend_dispatch)'
result: passed (3 tests)
date: 2026-04-29

command:
cargo nextest run -p ts2wasm-backend-wasm -E 'test(heap_closure_allocation_and_dispatch_emit_abi_payload_and_roots)'
result: passed (1 test)
date: 2026-04-29

command:
cargo nextest run -E 'test(closure) or test(function) or test(node_diff)'
result: passed (32 tests)
date: 2026-04-29

command:
cargo nextest run
result: passed (469 tests, 4 skipped)
date: 2026-04-29
```

Remaining risks:

- GC marking for closure capture slots remains issue 258. This issue does not
  claim closure retention under later allocation pressure.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/257-emit-heap-closure-allocation-and-dispatch.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
