---
id: 042
title: "Implement string methods"
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

Implement common string methods (trim, split, etc.).

## Problem

String methods are not implemented. They are essential for string manipulation.

## Desired final state

`str.trim()`, `str.split()`, etc. work correctly.

## Scope

In scope:

- [x] Implement String.prototype.trim
- [x] Implement String.prototype.split
- [x] Implement String.prototype.substring
- [x] Implement String.prototype.slice
- [x] Implement String.prototype.toUpperCase
- [x] Implement String.prototype.toLowerCase
- [x] Add fixtures for string methods

Out of scope:

- All string methods (start with common ones)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Common string methods work correctly
- [x] Fixtures cover string method behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/string-methods-test.ts -o /tmp/test.wasm
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

- [x] Placeholder string methods completed by `issues/done/214-replace-string-method-placeholders.md`

## Notes

Implemented String.trim, String.toUpperCase, and String.toLowerCase. Existing string methods (charAt, substring, slice, indexOf, split) were already implemented. Added fixtures for the new methods. The placeholder follow-up is completed by `issues/done/214-replace-string-method-placeholders.md`.

## Completion evidence

Commits:

- (pending commit for implementation)

Validation result:

```text
command: cargo nextest run
result: 205 passed, 4 skipped
date: 2026-04-26
```

Remaining risks:

- String.trim, toUpperCase, and toLowerCase placeholder behavior is completed by `issues/done/214-replace-string-method-placeholders.md`.
