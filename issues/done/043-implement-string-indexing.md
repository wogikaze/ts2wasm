---
id: 043
title: "Implement string indexing"
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

Implement string indexing `str[n]` to access UTF-16 code units.

## Problem

String indexing is not implemented. It is a common way to access string characters.

## Desired final state

`str[n]` returns the UTF-16 code unit at position n.

## Scope

In scope:

- [x] Implement string indexing in runtime
- [x] Add fixtures for string indexing behavior

Out of scope:

- Unicode code point indexing (P2)

## Affected paths

Expected:

- `crates/cli/src/backend/` (runtime)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] String indexing works correctly
- [x] Fixtures cover string indexing behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/string-indexing-test.ts -o /tmp/test.wasm
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
