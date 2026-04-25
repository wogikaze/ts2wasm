---
id: 050
title: "Implement Date"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement Date object for date/time operations.

## Problem

Date is not implemented. It is a common built-in for date/time operations.

## Desired final state

`new Date()` and Date methods work for basic operations.

## Scope

In scope:

- [ ] Implement Date constructor
- [ ] Implement Date.now()
- [ ] Implement Date.prototype.getTime
- [ ] Implement Date.prototype.toString
- [ ] Add fixtures for Date behavior

Out of scope:

- Full Date API (start with basic methods)

## Affected paths

Expected:

- `crates/cli/src/backend/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Date constructor works correctly
- [ ] Basic Date methods work correctly
- [ ] Fixtures cover Date behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/date-test.ts -o /tmp/test.wasm
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
