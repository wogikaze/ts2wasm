---
id: 041
title: "Implement template literals (dup)"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement template literal syntax `` `...` `` with string interpolation.

## Problem

Template literals are not implemented. They are a common ES6 feature for string interpolation.

## Desired final state

`` `Hello ${name}` `` interpolates expressions into strings.

## Scope

In scope:

- [x] Add template literal syntax to lexer/parser
- [x] Implement string interpolation (completed by `issues/done/213-implement-template-literal-interpolation.md`)
- [x] Add fixtures for template literal behavior

Out of scope:

- Tagged template literals (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Template literal parses correctly
- [x] Template literal interpolates expressions (completed by `issues/done/213-implement-template-literal-interpolation.md`)
- [x] Fixtures cover template literal behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/template-literal-test.ts -o /tmp/test.wasm
iwasm /tmp/test.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] Template literal interpolation support completed by `issues/done/213-implement-template-literal-interpolation.md`

## Notes

Basic template literal syntax is implemented (backtick strings parse correctly). Full `${}` interpolation support is completed by `issues/done/213-implement-template-literal-interpolation.md`.

## Completion evidence

Commits:

- `f4e04cf` Add template literal syntax support

Validation result:

```text
command: cargo nextest run
result: 207 tests passed, 4 skipped
date: 2026-04-27
```

Remaining risks:

- Tagged template literals and full raw/cooked template object semantics remain out of scope.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/041-implement-template-literals.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
