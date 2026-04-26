---
id: 062
title: "Implement Setyear (name-resolution)"
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

Implement support for reference/test262/test/annexB/built-ins/Date/prototype/setYear to handle 14 failing test cases in reference tests.

## Problem

Reference test results show 14 cases fail in directory `reference/test262/test/annexB/built-ins/Date/prototype/setYear` with diagnostics: date, name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

reference/test262/test/annexB/built-ins/Date/prototype/setYear is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for reference/test262/test/annexB/built-ins/Date/prototype/setYear
- [ ] Add fixtures for reference/test262/test/annexB/built-ins/Date/prototype/setYear behavior
- [ ] Update diagnostics appropriately

Out of scope:

- [ ] Related directories (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] reference/test262/test/annexB/built-ins/Date/prototype/setYear passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for reference/test262/test/annexB/built-ins/Date/prototype/setYear
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 28
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

- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/B.2.5.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/date-value-read-before-tonumber-when-date-is-invalid.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/date-value-read-before-tonumber-when-date-is-valid.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/length.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/name.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-not-date.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-time-nan.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-time-valid.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/time-clip.js`
- ... and 4 more files

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
