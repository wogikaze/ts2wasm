---
id: 061
title: "Implement Date object support"
type: feature
area: frontend
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement date feature to handle 17 failing test cases in reference tests.

## Problem

Reference test results show 17 cases fail with date diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

date feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for date feature
- [ ] Add fixtures for date feature behavior
- [ ] Update diagnostics appropriately

Out of scope:

- [ ] Related features (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] date feature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for date feature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/manager reference-coverage test262 --limit 34
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/nan.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/return-value.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/this-not-date.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/date-value-read-before-tonumber-when-date-is-invalid.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/date-value-read-before-tonumber-when-date-is-valid.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-not-date.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-time-nan.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-time-valid.js`
- ... and 7 more files

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
