---
id: 045
title: "Implement class declaration and expression"
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

Implement class declaration and expression syntax.

## Problem

Class syntax is not implemented. It is a fundamental ES6 feature for object-oriented programming.

## Desired final state

`class C { ... }` and `const C = class { ... };` parse and execute correctly.

## Scope

In scope:

- [ ] Add class declaration to lexer/parser
- [ ] Add class expression to lexer/parser
- [ ] Lower class to constructor function
- [ ] Add fixtures for class syntax

Out of scope:

- extends (046)
- super (047)
- static members (P2)
- private fields (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Class declaration parses correctly
- [ ] Class expression parses correctly
- [ ] Class creates constructor function
- [ ] Fixtures cover class syntax
- [ ] No regression in existing fixtures

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

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] 046 (extends)
- [ ] 047 (super)

## Notes

Note: fixtures/classes-and-inheritance/*.ts exist but may not be fully implemented.

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
