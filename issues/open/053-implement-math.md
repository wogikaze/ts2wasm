---
id: 053
title: "Implement Math"
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

Implement Math object with common math functions.

## Problem

Math is not implemented. It is essential for mathematical operations.

## Desired final state

`Math.abs()`, `Math.floor()`, etc. work correctly.

## Scope

In scope:

- [ ] Implement Math.abs
- [ ] Implement Math.floor
- [ ] Implement Math.ceil
- [ ] Implement Math.round
- [ ] Implement Math.min
- [ ] Implement Math.max
- [ ] Implement Math.random
- [ ] Add fixtures for Math behavior

Out of scope:

- Full Math API (start with common functions)

## Affected paths

Expected:

- `crates/cli/src/backend/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Common Math functions work correctly
- [ ] Fixtures cover Math behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/math-test.ts -o /tmp/test.wasm
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

Note: fixtures/builtins-and-io/math-*.ts exist for some functions.

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
