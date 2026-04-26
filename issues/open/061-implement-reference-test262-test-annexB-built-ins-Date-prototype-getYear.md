---
id: 061
title: "Implement Getyear (name-resolution)"
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

Implement support for reference/test262/test/annexB/built-ins/Date/prototype/getYear to handle 7 failing test cases in reference tests.

## Problem

Reference test results show 7 cases fail in directory `reference/test262/test/annexB/built-ins/Date/prototype/getYear` with diagnostics: date, name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

reference/test262/test/annexB/built-ins/Date/prototype/getYear is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for reference/test262/test/annexB/built-ins/Date/prototype/getYear
- [ ] Add fixtures for reference/test262/test/annexB/built-ins/Date/prototype/getYear behavior
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

- [ ] reference/test262/test/annexB/built-ins/Date/prototype/getYear passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for reference/test262/test/annexB/built-ins/Date/prototype/getYear
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 14
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

- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/length.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/name.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/nan.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/return-value.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/this-not-date.js`

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
