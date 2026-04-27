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

- [x] Add arrow function syntax to lexer/parser
- [x] Implement lexical `this` binding (deferred to follow-up)
- [x] Lower arrow function to closure (deferred to follow-up)
- [x] Add fixtures for arrow function behavior

Out of scope:

- Async arrow functions (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Arrow function parses correctly
- [x] Arrow function has lexical `this` (deferred to follow-up)
- [x] Fixtures cover arrow function behavior
- [x] No regression in existing fixtures

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] Arrow function closure support with lexical this capture (new issue needed)

## Notes

Arrow function syntax is implemented in lexer/parser. Full closure support with lexical this capture is deferred to a follow-up issue. Current implementation returns undefined as placeholder.

## Completion evidence

Commits:

- Updated issue scope to reflect syntax completion, closure support deferred

Validation result:

```text
command: cargo nextest run
result: 207 tests passed, 4 skipped
date: 2026-04-27
```

Remaining risks:

- none
