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

- `crates/backend-wasm/src/` (runtime builtins)
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

2026-04-28 blocker evidence:

- `new Date(0)` currently reaches class-constructor lowering and fails before backend Date runtime code can be used:

  ```text
  command: cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-date-ZvvJxH.ts -o /tmp/ts2wasm-date-test.wasm
  result: exit 1
  stderr: error: [UnsupportedSyntax] issue-207: instanceof right-hand side must be a supported class constructor `Date`
  ```

- `Date.now()` currently fails in name/lowering before backend emission:

  ```text
  command: cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-date-now-jjxJat.ts -o /tmp/ts2wasm-date-now-test.wasm
  result: exit 1
  stderr: error: [UnresolvedName] unresolved name: `Date`
  ```

- The required recognition/lowering changes live in `crates/ir/src/name_resolver.rs` and `crates/ir/src/lowered.rs`, which are outside the child assignment's allowed files. Completing `Date.now()` or zero-argument `new Date()` also requires an auditable time capability policy; the assignment explicitly forbids inventing untracked host time imports.

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
