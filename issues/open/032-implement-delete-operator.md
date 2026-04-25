---
id: 032
title: "Implement delete operator"
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

Implement the `delete` operator to remove properties from objects.

## Problem

The `delete` operator is not implemented. It is used to remove properties from objects.

## Desired final state

`delete obj.prop` removes the property from the object and returns true if successful.

## Scope

In scope:

- [ ] Add `delete` to lexer/parser
- [ ] Lower `delete` expression to runtime call
- [ ] Implement property deletion
- [ ] Add fixtures for delete operator behavior

Out of scope:

- Deleting non-configurable properties (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/cli/src/backend/` (lowering)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] `delete` expression parses correctly
- [ ] `delete` removes properties correctly
- [ ] Fixtures cover delete operator behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/delete-test.ts -o /tmp/test.wasm
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

Requires property access implementation.

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
