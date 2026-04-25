---
id: 041
title: "Implement template literals"
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

- [ ] Add template literal syntax to lexer/parser
- [ ] Implement string interpolation
- [ ] Add fixtures for template literal behavior

Out of scope:

- Tagged template literals (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Template literal parses correctly
- [ ] Template literal interpolates expressions
- [ ] Fixtures cover template literal behavior
- [ ] No regression in existing fixtures

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

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

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
