---
id: 084
title: "Implement Sub (parser-syntax)"
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

Implement support for reference/test262/test/annexB/built-ins/String/prototype/sub to handle 6 failing test cases in reference tests.

## Problem

Reference test results show 6 cases fail in directory `reference/test262/test/annexB/built-ins/String/prototype/sub` with diagnostics: function, name-resolution, parser-syntax, unsupported-expression. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

reference/test262/test/annexB/built-ins/String/prototype/sub is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for reference/test262/test/annexB/built-ins/String/prototype/sub
- [ ] Add fixtures for reference/test262/test/annexB/built-ins/String/prototype/sub behavior
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

- [ ] reference/test262/test/annexB/built-ins/String/prototype/sub passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for reference/test262/test/annexB/built-ins/String/prototype/sub
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 12
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

- `reference/test262/test/annexB/built-ins/String/prototype/sub/B.2.3.13.js`
- `reference/test262/test/annexB/built-ins/String/prototype/sub/length.js`
- `reference/test262/test/annexB/built-ins/String/prototype/sub/name.js`
- `reference/test262/test/annexB/built-ins/String/prototype/sub/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/sub/prop-desc.js`
- `reference/test262/test/annexB/built-ins/String/prototype/sub/this-val-tostring-err.js`

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
