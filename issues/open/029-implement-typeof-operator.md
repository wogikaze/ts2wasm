---
id: 029
title: "Implement typeof operator"
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

Implement the `typeof` operator to return the type tag of a value.

## Problem

The `typeof` operator is not implemented. It is a fundamental JavaScript operator used for type checking at runtime.

## Desired final state

`typeof x` returns the correct type string for all primitive types and objects:
- `"undefined"` for undefined
- `"object"` for null
- `"boolean"` for boolean
- `"number"` for number
- `"string"` for string
- `"function"` for function objects
- `"object"` for objects and arrays

## Scope

In scope:

- [ ] Add `typeof` to lexer/parser
- [ ] Lower `typeof` expression to runtime call
- [ ] Implement runtime type tag check
- [ ] Add fixtures for typeof behavior

Out of scope:

- `typeof` for symbols (P2)
- `typeof` for bigint (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] `typeof` expression parses correctly
- [ ] `typeof` returns correct type strings for all primitive types
- [ ] `typeof null` returns `"object"` (ECMAScript spec compliance)
- [ ] Fixtures cover typeof behavior for all types
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/typeof-test.ts -o /tmp/test.wasm
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

The `typeof` operator should lower to a runtime function that checks the value tag and returns the appropriate string constant.

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
