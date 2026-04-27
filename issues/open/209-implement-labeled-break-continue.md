---
id: 209
title: "Implement labeled break and continue"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: [035]
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Implement ECMAScript labeled `break` and `continue` for nested loops and labeled statements.

## Problem

Issue 035 completed unlabeled loop control but left labeled control flow as future work. The docs should not imply that all `break`/`continue` semantics are complete.

## Desired final state

`break label;` and `continue label;` target the correct enclosing labeled statement or loop, and invalid label usage is diagnosed.

## Scope

In scope:

- [ ] Parse labeled statements and labeled break/continue targets.
- [ ] Resolve labels with correct nesting and target validity.
- [ ] Lower and emit labeled control flow for nested loops.
- [ ] Add Node differential fixtures for nested target selection and invalid-label diagnostics.

Out of scope:

- Switch fall-through semantics; tracked by issue 208.

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

- [ ] Labeled `break` exits the matching labeled statement.
- [ ] Labeled `continue` continues the matching labeled loop and rejects non-loop targets.
- [ ] Invalid or duplicate label cases produce source diagnostics.
- [ ] Node differential fixtures cover nested label behavior.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(break|continue|label)'
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

Created from issue 203 audit of `issues/done/035-implement-break-continue.md`.

## Completion evidence

Fill only when moving to `done/`.
