---
id: 079
title: "Implement Fontsize (unknown-unsupported)"
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

Implement support for reference/test262/test/annexB/built-ins/String/prototype/fontsize to handle 7 failing test cases in reference tests.

## Problem

Reference test results show 7 cases fail in directory `reference/test262/test/annexB/built-ins/String/prototype/fontsize` with diagnostics: function, name-resolution, unknown-unsupported, unsupported-expression. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

reference/test262/test/annexB/built-ins/String/prototype/fontsize is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for reference/test262/test/annexB/built-ins/String/prototype/fontsize
- [ ] Add fixtures for reference/test262/test/annexB/built-ins/String/prototype/fontsize behavior
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

- [ ] reference/test262/test/annexB/built-ins/String/prototype/fontsize passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for reference/test262/test/annexB/built-ins/String/prototype/fontsize
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

- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/B.2.3.8.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/attr-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/length.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/name.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/prop-desc.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/this-val-tostring-err.js`

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
