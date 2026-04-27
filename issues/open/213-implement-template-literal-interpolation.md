---
id: 213
title: "Implement template literal interpolation"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Implement `${...}` expression interpolation for JavaScript template literals.

## Problem

Issue 041 added basic backtick literal syntax but recorded interpolation as deferred work. Literal-only parsing must stay separate from full template-literal semantic support.

## Desired final state

Template literals concatenate cooked string segments and expression values using JavaScript string conversion, matching Node.js for the supported subset.

## Scope

In scope:

- [ ] Parse template literal parts with embedded `${...}` expressions.
- [ ] Lower interpolation to string conversion and concatenation.
- [ ] Add Node differential fixtures for one expression, multiple expressions, empty segments, and escaped backticks.
- [ ] Update docs/current-state/issues when semantic status changes.

Out of scope:

- Tagged template literals.
- Full raw/cooked template object semantics.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [ ] Template literal interpolation with `${...}` parses and executes.
- [ ] Interpolated values use the project's JavaScript string conversion path.
- [ ] Node differential fixtures cover multiple interpolation shapes.
- [ ] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(template)'
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

Created from issue 203 audit of `issues/done/041-implement-template-literals.md`.

## Completion evidence

Fill only when moving to `done/`.
