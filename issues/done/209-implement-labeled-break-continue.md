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
completed: 2026-04-28
---

## Summary

Implement ECMAScript labeled `break` and `continue` for nested loops and labeled statements.

## Problem

Issue 035 completed unlabeled loop control but left labeled control flow as future work. The docs should not imply that all `break`/`continue` semantics are complete.

## Desired final state

`break label;` and `continue label;` target the correct enclosing labeled statement or loop, and invalid label usage is diagnosed.

## Scope

In scope:

- [x] Parse labeled statements and labeled break/continue targets.
- [x] Resolve labels with correct nesting and target validity.
- [x] Lower and emit labeled control flow for nested loops.
- [x] Add Node differential fixtures for nested target selection and invalid-label diagnostics.

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

- [x] Labeled `break` exits the matching labeled statement.
- [x] Labeled `continue` continues the matching labeled loop and rejects non-loop targets.
- [x] Invalid or duplicate label cases produce source diagnostics.
- [x] Node differential fixtures cover nested label behavior.

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

- [x] update `docs/language-reference/javascript-features.md`

Current state:

- [x] update `current-state.md`

Follow-up issues:

- [x] none

## Notes

Created from issue 203 audit of `issues/done/035-implement-break-continue.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `2cb1b9415ba7d8287bf7c6012c94f4040a6ea961`

Validation result:

```text
command: cargo nextest run -E 'test(break|continue|label)'
result: failed before execution because the literal assignment filter selected 0 tests (nextest exit 4: no tests to run)
date: 2026-04-28

command: cargo nextest run -E 'test(/break|continue|label/)'
result: passed; 5 tests run, 5 passed
date: 2026-04-28
```

Remaining risks:

- `crates/compiler/src/dump.rs` and `crates/compiler/src/lib.rs` needed minimal pass-through handling for the new AST node even though they were not listed in the child assignment allowed paths.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

