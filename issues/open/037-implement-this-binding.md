---
id: 037
title: "Implement this binding"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement `this` keyword with call site receiver binding.

## Problem

The `this` keyword is not implemented. It is essential for method calls and object-oriented patterns.

## Desired final state

`this` refers to the call site receiver in method calls and constructor calls.

## Scope

In scope:

- [ ] Add `this` to lexer/parser
- [ ] Implement call site receiver binding
- [ ] Implement constructor `this` binding
- [ ] Add fixtures for this behavior

Out of scope:

- Arrow function lexical this (036)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] `this` parses correctly
- [ ] `this` binds to call site receiver
- [ ] Fixtures cover this behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/this-test.ts -o /tmp/test.wasm
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

- [ ] 036 (arrow function)

## Notes

Requires method call implementation.

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
