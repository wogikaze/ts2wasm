---
id: 207
title: "Complete instanceof prototype-chain semantics"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: [048]
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Complete `instanceof` runtime semantics so the operator tests the right-hand constructor's `prototype` against the left-hand object's prototype chain.

## Problem

Issue 030 added parser/lowering support but recorded that emitted behavior is a placeholder. The operator must not count as semantic compatibility until prototype-chain traversal is implemented and differentially verified.

## Desired final state

`obj instanceof Constructor` matches Node.js behavior for ordinary constructors and objects with a traversable prototype chain.

## Scope

In scope:

- [ ] Replace the current placeholder result with runtime prototype-chain traversal.
- [ ] Validate right-hand operands and report unsupported/diagnostic behavior for unsupported constructor forms.
- [ ] Add Node differential fixtures for true, false, and non-object left-hand operands.
- [ ] Update docs/current-state/issues when semantic status changes.

Out of scope:

- Custom `Symbol.hasInstance` behavior.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [ ] `instanceof` no longer returns a fixed placeholder result.
- [ ] Differential fixtures match Node.js for ordinary prototype-chain cases.
- [ ] Unsupported constructor or custom-hasInstance cases are explicitly diagnosed or tracked.
- [ ] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(instanceof)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update `docs/language-reference/javascript-features.md`

Current state:

- [ ] update `current-state.md`

Follow-up issues:

- [ ] none

## Notes

Created from issue 203 audit of `issues/done/030-implement-instanceof-operator.md`.

## Completion evidence

Fill only when moving to `done/`.
