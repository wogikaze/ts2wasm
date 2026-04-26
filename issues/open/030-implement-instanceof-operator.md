---
id: 030
title: "Implement instanceof operator"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: [016]
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement the `instanceof` operator to check if an object is an instance of a constructor.

## Problem

The `instanceof` operator is not implemented. It is used to check the prototype chain of an object against a constructor function.

## Desired final state

`obj instanceof Constructor` returns true if `Constructor.prototype` is in the prototype chain of `obj`.

## Scope

In scope:

- [ ] Add `instanceof` to lexer/parser
- [ ] Lower `instanceof` expression to runtime call
- [ ] Implement prototype chain lookup
- [ ] Add fixtures for instanceof behavior

Out of scope:

- Custom `@@hasInstance` behavior (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] `instanceof` expression parses correctly
- [ ] `instanceof` checks prototype chain correctly
- [ ] Fixtures cover instanceof behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/instanceof-test.ts -o /tmp/test.wasm
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

Requires prototype chain implementation (blocked by 016).

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
