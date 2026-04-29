---
id: 062g
title: "Define and implement heap closure object ABI and rooting"
type: feature
area: runtime/abi
class: blocked
priority: P1
depends_on: ["256", "257", "258"]
blocks: ["062e"]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Define and implement the closure value representation needed for ordinary
functions or arrows that escape the declaring activation.

Problem: Returned closures need a heap object ABI and GC rooting contract before
implementation can safely preserve captured activations.

Problem: Returned closures cannot be represented by the current devirtualized
generated-function lowering because captured values are passed as hidden call
arguments, not stored in a heap environment that survives the declaring scope.

## Current failure

`fixtures/core-semantics/ordinary-function-closure-escape-unsupported.ts`
is intentionally rejected with `issue-062e:` because returning a nested
ordinary function would require a heap closure object and GC-rooted captured
environment.

The current implementation evidence is:

- `LoweredExpr::ArrowFn` is an opaque local token, not a callable heap object.
- Calls to known local closures are rewritten to direct `FunctionCallKind::User`
  calls with captured locals appended as hidden parameters.
- Returning that token would not preserve the captured activation after the
  declaring function returns.

## Desired final state

Escaping function values have a concrete runtime representation containing the
code identity and captured environment. Returned closures can be called after
the declaring scope has returned, and their captured heap values remain live
across allocation pressure.

## Scope

In scope:

- [ ] Define the closure object/environment layout and code identity contract.
- [ ] Lower returned ordinary function closures to heap closure values.
- [ ] Dispatch calls through supported heap closure values.
- [ ] Root closure environments during GC mark/sweep.
- [ ] Add Node/iwasm differential fixtures for returned closure capture under
      allocation pressure.

Out of scope:

- Dynamic `Function` constructor semantics.
- `eval` / Annex B function declaration semantics.
- Function metadata (`name`, `length`, `prototype`) beyond existing 062f scope.
- Generator/async closure semantics.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/core-semantics/`
- `crates/cli/tests/`
- `docs/14-runtime-abi.md` if the closure ABI changes final-state contracts

Do not touch:

- `crates/runtime-abi/` unless the ABI change is explicitly reviewed.
- Issue 062f function metadata behavior.
- Issue 225 eval behavior.

## Acceptance criteria

- [ ] A returned closure keeps an immutable captured local live after the
      declaring function returns.
- [ ] A returned closure with a captured heap value survives allocation pressure
      in a Node/iwasm differential fixture.
- [ ] Unsupported mutable environment forms either work correctly or produce an
      issue-linked diagnostic with a follow-up.
- [ ] Runtime/ABI docs or current-state notes are synchronized with the closure
      object contract.

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

- [ ] updated: `docs/14-runtime-abi.md` if the runtime closure ABI changes

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] created: `issues/done/256-lower-returned-immutable-closures-to-heap-values.md`
- [x] created: `issues/open/257-emit-heap-closure-allocation-and-dispatch.md`
- [x] created: `issues/open/258-mark-heap-closure-captures-and-add-allocation-pressure-fixture.md`

## Notes

Start from the current non-escaping closure path: it already collects immutable
captures and passes them as hidden params for direct known calls. Do not reuse
that opaque token for escaping closures without a heap environment.

2026-04-29 design slice: `docs/14-runtime-abi.md` now defines the closure heap
object ABI. Closure values are object-tagged heap values with a closure sentinel,
`code_id`, immutable capture count, reserved flags, and raw capture slots. The
remaining implementation is split into issues 256, 257, and 258 so this broad
parent remains a tracking blocker rather than an executable work order.

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
