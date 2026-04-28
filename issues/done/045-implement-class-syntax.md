---
id: 045
title: "Implement class declaration and expression"
type: feature
area: frontend/semantics
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Implement class declaration and expression syntax.

## Problem

Class syntax is not implemented. It is a fundamental ES6 feature for object-oriented programming.

## Desired final state

`class C { ... }` and `const C = class { ... };` parse and execute correctly.

## Scope

In scope:

- [x] Add class declaration to lexer/parser
- [x] Add class expression to lexer/parser
- [x] Lower class to constructor function
- [x] Add fixtures for class syntax

Out of scope:

- extends (046)
- super (047)
- static members (P2)
- private fields (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Class declaration parses correctly
- [x] Class expression parses correctly
- [x] Class creates constructor function
- [x] Fixtures cover class syntax
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/class-test.ts -o /tmp/test.wasm
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

- [x] 046 (extends)
- [x] 047 (super)

## Notes

Note: fixtures/classes-and-inheritance/*.ts exist but may not be fully implemented.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `3f32481` issue-045: support class expression bindings

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli class_expression
result: 2 passed, including Node/iwasm differential for fixtures/classes-and-inheritance/class-expression.ts
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli class
result: 13 passed
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli oop
result: 5 passed
date: 2026-04-28

command: cargo nextest run
result: 249 passed, 4 skipped
date: 2026-04-28
```

Remaining risks:

- none
