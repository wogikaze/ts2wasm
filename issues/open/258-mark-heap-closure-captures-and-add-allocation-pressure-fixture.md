---
id: 258
title: "Mark heap closure captures and add allocation-pressure fixture"
type: feature
area: runtime
class: implementation-ready
priority: P1
depends_on: [257]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Finish closure GC rooting by teaching mark/sweep to scan closure capture slots
and by adding allocation-pressure differential coverage.

Problem: A closure object that stores captured heap values is unsafe unless the
GC mark phase traverses the closure payload and keeps those captured values
live.

## Current failure

The current `gc_mark_object_payload` path scans ordinary object prototype and
property entries. It has no closure-sentinel branch that marks capture slots.

## Desired final state

Closure objects are marked as object-tagged heap values, and their capture slots
are recursively marked before ordinary object scanning returns. A Node/iwasm
differential fixture proves a returned closure can retain a captured heap value
across allocation pressure.

## Scope

In scope:

- [ ] Add closure-sentinel detection to the runtime object mark path.
- [ ] Mark each `RawValue` capture slot from the closure payload.
- [ ] Add a fixture where a returned closure captures a heap value and is called
      after enough allocations to trigger GC.
- [ ] Update current-state support notes once the fixture passes.

Out of scope:

- Mutable environment cells.
- Function metadata/prototype properties on closure objects.
- Broader closure arity dispatch beyond the issue 257 supported slice.
- `eval`, dynamic `Function`, generator, or async closure semantics.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `fixtures/core-semantics/`
- `crates/cli/tests/`
- `current-state.md`

Do not touch:

- `crates/runtime-abi/` unless the closure sentinel/layout constants have been
      explicitly reviewed for ABI promotion.
- Issue 062f function metadata behavior.
- Issue 225 eval behavior.

## Acceptance criteria

- [ ] `gc_mark_object_payload` or its replacement detects the closure sentinel
      and marks all capture slots.
- [ ] A returned closure with a captured string or object survives allocation
      pressure in Node/iwasm differential coverage.
- [ ] Existing object, array, string, class prototype, module cache, top-level
      root, and call-frame GC fixtures still pass.
- [ ] `current-state.md` records the new supported closure/GC boundary.

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

The closure ABI is in `docs/14-runtime-abi.md`. The first capture slot is at
payload offset `16`; each slot is a raw tagged `i32` value.

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
