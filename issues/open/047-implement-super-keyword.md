---
id: 047
title: "Implement super keyword"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: [045, 046]
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement `super` keyword for accessing parent class members.

## Problem

The `super` keyword is not implemented. It is essential for calling parent constructors and methods.

## Desired final state

`super()` calls parent constructor, `super.method()` calls parent method.

## Scope

In scope:

- [ ] Add super to lexer/parser
- [ ] Implement super constructor call
- [ ] Implement super method call
- [ ] Add fixtures for super behavior

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

- [ ] super parses correctly
- [ ] super() calls parent constructor
- [ ] super.method() calls parent method
- [ ] Fixtures cover super behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/super-test.ts -o /tmp/test.wasm
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

Requires class and extends implementation.

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
