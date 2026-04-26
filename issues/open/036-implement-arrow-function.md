---
id: 036
title: "Implement arrow function"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: [037]
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement arrow function syntax with lexical `this` binding.

## Problem

Arrow functions are not implemented. They are a common ES6 feature with lexical `this` binding.

## Desired final state

`const f = (x) => x * 2;` parses and executes correctly with lexical `this`.

## Scope

In scope:

- [ ] Add arrow function syntax to lexer/parser
- [ ] Implement lexical `this` binding
- [ ] Lower arrow function to closure
- [ ] Add fixtures for arrow function behavior

Out of scope:

- Async arrow functions (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Arrow function parses correctly
- [ ] Arrow function has lexical `this`
- [ ] Fixtures cover arrow function behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/arrow-function-test.ts -o /tmp/test.wasm
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

Requires `this` implementation.

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
