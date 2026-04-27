---
id: 216
title: "Implement abstract equality coercion"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Implement ECMAScript coercion rules for abstract equality operators `==` and `!=`.

## Problem

Issue 058 added equality operator support but records that abstract equality currently delegates to strict equality. That is a semantic placeholder for mixed-type comparisons.

## Desired final state

`==` and `!=` match ECMAScript abstract equality for the supported value types, with unsupported object/primitive coercions diagnosed or tracked.

## Scope

In scope:

- [ ] Implement primitive coercion cases for `undefined`, `null`, boolean, number, and string.
- [ ] Preserve strict equality semantics for `===` and `!==`.
- [ ] Add Node differential fixtures for mixed-type equality and inequality.
- [ ] Track object/ToPrimitive gaps explicitly if not completed in this slice.

Out of scope:

- Full object `ToPrimitive` behavior if the object model is not ready.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `docs/05-compatibility-and-semantics.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [ ] `==` and `!=` no longer behave as strict equality for supported mixed primitive cases.
- [ ] `===` and `!==` behavior is unchanged.
- [ ] Node differential fixtures cover representative coercion rules.
- [ ] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(equal|equality)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update `docs/language-reference/javascript-features.md`
- [ ] update `docs/05-compatibility-and-semantics.md`

Current state:

- [ ] update `current-state.md`

Follow-up issues:

- [ ] none

## Notes

Created from issue 203 audit of `issues/done/058-implement-equality-operators.md`.

## Completion evidence

Fill only when moving to `done/`.
