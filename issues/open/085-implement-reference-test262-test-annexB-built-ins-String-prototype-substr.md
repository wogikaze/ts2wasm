---
id: 085
title: "Implement Substr (name-resolution)"
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

Implement support for reference/test262/test/annexB/built-ins/String/prototype/substr to handle 15 failing test cases in reference tests.

## Problem

Reference test results show 15 cases fail in directory `reference/test262/test/annexB/built-ins/String/prototype/substr` with diagnostics: function, name-resolution, parser-syntax, unknown-unsupported, unsupported-expression. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

reference/test262/test/annexB/built-ins/String/prototype/substr is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for reference/test262/test/annexB/built-ins/String/prototype/substr
- [ ] Add fixtures for reference/test262/test/annexB/built-ins/String/prototype/substr behavior
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

- [ ] reference/test262/test/annexB/built-ins/String/prototype/substr passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for reference/test262/test/annexB/built-ins/String/prototype/substr
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 30
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

- `reference/test262/test/annexB/built-ins/String/prototype/substr/B.2.3.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/length-falsey.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/length-negative.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/length-positive.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/length-to-int-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/length-undef.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/length.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/name.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/start-and-length-as-numbers.js`
- ... and 5 more files

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
