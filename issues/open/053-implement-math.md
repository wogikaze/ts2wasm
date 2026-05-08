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

- [x] Implement Math.abs
- [x] Implement Math.floor
- [x] Implement Math.ceil
- [x] Implement Math.round
- [x] Implement Math.min
- [x] Implement Math.max
- [x] Implement Math.random
- [x] Add fixtures for Math behavior

Out of scope:

- Full Math API (start with common functions)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Common Math functions work correctly
- [x] Fixtures cover Math behavior
- [x] No regression in existing fixtures

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] Math.random capability/randomness policy tracked by `issues/open/215-define-math-random-capability-policy.md`

## Notes

Note: fixtures/builtins-and-io/math-*.ts exist for some functions.
Implemented Math.random() to complete the Math implementation. Math.abs, Math.floor, Math.ceil, Math.round, Math.min, and Math.max were already implemented with existing fixtures. Added math-random.ts fixture.

## Completion evidence

Commits:

- (pending commit for implementation)

Validation result:

```text
command: cargo nextest run
result: 202 passed, 4 skipped
date: 2026-04-26
```

Remaining risks:

- Math.random() capability/randomness policy is tracked by `issues/open/215-define-math-random-capability-policy.md`.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/053-implement-math.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
