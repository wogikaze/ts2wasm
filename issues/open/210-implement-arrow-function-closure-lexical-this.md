---
id: 210
title: "Implement arrow function closure and lexical this semantics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: [211]
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Complete arrow function runtime semantics by lowering closures and preserving lexical `this`.

## Problem

Issue 036 completed arrow syntax but recorded placeholder emission. Arrow functions must not count as semantic compatibility until closure capture and lexical `this` behavior are differentially verified.

## Desired final state

Arrow functions execute with captured lexical variables and lexical `this`, matching Node.js behavior for the supported subset.

## Scope

In scope:

- [ ] Lower arrow functions to callable closure values instead of placeholder `undefined`.
- [ ] Capture referenced lexical variables needed by the arrow body.
- [ ] Preserve lexical `this` rather than binding a new receiver at call time.
- [ ] Add Node differential fixtures for expression body, block body, captured variable, and lexical `this`.

Out of scope:

- Async arrow functions.

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

- [ ] Arrow functions no longer lower or emit to a placeholder value.
- [ ] Closure capture works for at least supported local bindings.
- [ ] Lexical `this` matches Node.js in differential fixtures.
- [ ] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(arrow|closure|this)'
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

Created from issue 203 audit of `issues/done/036-implement-arrow-function.md`.

## Completion evidence

Fill only when moving to `done/`.
