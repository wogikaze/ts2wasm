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

- [ ] Implement String.prototype.trim
- [ ] Implement String.prototype.split
- [ ] Implement String.prototype.substring
- [ ] Implement String.prototype.slice
- [ ] Implement String.prototype.toUpperCase
- [ ] Implement String.prototype.toLowerCase
- [ ] Add fixtures for string methods

Out of scope:

- All string methods (start with common ones)

## Affected paths

Expected:

- `crates/cli/src/backend/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Common string methods work correctly
- [ ] Fixtures cover string method behavior
- [ ] No regression in existing fixtures

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

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Note: fixtures/builtins-and-io/string-*.ts exist for some methods.

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
