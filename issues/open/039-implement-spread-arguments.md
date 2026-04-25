---
id: 039
title: "Implement spread arguments"
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

Implement spread arguments syntax `...arr` for function calls.

## Problem

Spread arguments are not implemented. They are a common ES6 feature for spreading arrays into arguments.

## Desired final state

`f(...arr)` spreads array elements as individual arguments.

## Scope

In scope:

- [ ] Add spread argument syntax to lexer/parser
- [ ] Lower spread arguments to individual argument passing
- [ ] Add fixtures for spread argument behavior

Out of scope:

- Spread in array literals (P2)
- Spread in object literals (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Spread argument parses correctly
- [ ] Spread argument spreads array elements
- [ ] Fixtures cover spread argument behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/spread-args-test.ts -o /tmp/test.wasm
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
