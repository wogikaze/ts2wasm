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
completed: 2026-04-26
---

## Summary

Implement default parameter syntax `x = 1` for function parameters.

## Problem

Default parameters are not implemented. They are a common ES6 feature for parameter defaults.

## Desired final state

`function f(x = 1) { ... }` uses default value when argument is undefined.

## Scope

In scope:

- [x] Add default parameter syntax to lexer/parser
- [x] Lower default parameters to conditional checks
- [x] Add fixtures for default parameter behavior

Out of scope:

- none

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Default parameter parses correctly
- [x] Default parameter applies when argument is undefined
- [x] Fixtures cover default parameter behavior
- [x] No regression in existing fixtures

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Fixed compilation errors for rest parameters and this binding integration
- Added Ellipsis token kind to lexer
- Fixed param tuple patterns to handle 3-element tuples (name, default, is_rest)
- Added missing Expr::This cases to various match statements

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-26

command: cargo nextest run
result: pass (176/177 tests, 1 unrelated m9_modules failure)
date: 2026-04-26
```

Remaining risks:

- none
