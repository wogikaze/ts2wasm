---
id: 064
title: "Implement name resolution"
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

Implement name-resolution feature to handle 72 failing test cases in reference tests.

## Problem

Reference test results show 72 cases fail with name-resolution diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

name-resolution feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for name-resolution feature
- [ ] Add fixtures for name-resolution feature behavior
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

- [ ] name-resolution feature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for name-resolution feature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 144
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
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/B.2.5.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/length.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/name.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/toGMTString/value.js`
- `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js`
- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/B.RegExp.prototype.compile.js`
- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/length.js`
- ... and 62 more files

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
