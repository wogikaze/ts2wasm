---
id: 046
title: "Implement extends inheritance"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: [045]
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement class inheritance with `extends` keyword.

## Problem

Class inheritance is not implemented. It is essential for object-oriented programming.

## Desired final state

`class Child extends Parent { ... }` correctly sets up prototype chain.

## Scope

In scope:

- [ ] Add extends syntax to lexer/parser
- [ ] Implement prototype chain setup
- [ ] Add fixtures for inheritance behavior

Out of scope:

- super (047)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] extends parses correctly
- [ ] Prototype chain is set up correctly
- [ ] Fixtures cover inheritance behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/extends-test.ts -o /tmp/test.wasm
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

- [ ] 047 (super)

## Notes

Requires prototype chain implementation (016).

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
