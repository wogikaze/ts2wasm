---
id: 038
title: "Implement rest parameters"
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

Implement rest parameters syntax `...args` for variadic functions.

## Problem

Rest parameters are not implemented. They are a common ES6 feature for variadic functions.

## Desired final state

`function f(...args) { ... }` collects remaining arguments into an array.

## Scope

In scope:

- [ ] Add rest parameter syntax to lexer/parser
- [ ] Lower rest parameters to array collection
- [ ] Add fixtures for rest parameter behavior

Out of scope:

- none

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Rest parameter parses correctly
- [ ] Rest parameter collects arguments into array
- [ ] Fixtures cover rest parameter behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/rest-params-test.ts -o /tmp/test.wasm
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
