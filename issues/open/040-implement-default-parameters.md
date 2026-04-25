---
id: 040
title: "Implement default parameters"
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

Implement default parameter syntax `x = 1` for function parameters.

## Problem

Default parameters are not implemented. They are a common ES6 feature for parameter defaults.

## Desired final state

`function f(x = 1) { ... }` uses default value when argument is undefined.

## Scope

In scope:

- [ ] Add default parameter syntax to lexer/parser
- [ ] Lower default parameters to conditional checks
- [ ] Add fixtures for default parameter behavior

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

- [ ] Default parameter parses correctly
- [ ] Default parameter applies when argument is undefined
- [ ] Fixtures cover default parameter behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/default-params-test.ts -o /tmp/test.wasm
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
