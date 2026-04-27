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
completed: 2026-04-26
---

## Summary

Implement the `delete` operator to remove properties from objects.

## Problem

The `delete` operator is not implemented. It is used to remove properties from objects.

## Desired final state

`delete obj.prop` removes the property from the object and returns true if successful.

## Scope

In scope:

- [x] Add `delete` to lexer/parser
- [x] Lower `delete` expression to runtime call
- [x] Implement property deletion
- [x] Add fixtures for delete operator behavior

Out of scope:

- Deleting non-configurable properties (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (lowering/backend)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] `delete` expression parses correctly
- [x] `delete` removes properties correctly
- [x] Fixtures cover delete operator behavior
- [x] No regression in existing fixtures

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Requires property access implementation.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `76a0a4c` ir/backend: add delete operator support
- `7bfbc40` runtime: implement delete operator with property deletion

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-26

command: cargo nextest run
result: pass (174/175 tests, 1 unrelated m9_modules failure)
date: 2026-04-26
```

Remaining risks:

- none
