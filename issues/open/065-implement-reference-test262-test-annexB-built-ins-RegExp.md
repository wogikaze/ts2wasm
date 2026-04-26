---
id: 065
title: "Implement Regexp (regexp-literal)"
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

Implement support for reference/test262/test/annexB/built-ins/RegExp to handle 10 failing test cases in reference tests.

## Problem

Reference test results show 10 cases fail in directory `reference/test262/test/annexB/built-ins/RegExp` with diagnostics: regexp-literal. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

reference/test262/test/annexB/built-ins/RegExp is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for reference/test262/test/annexB/built-ins/RegExp
- [ ] Add fixtures for reference/test262/test/annexB/built-ins/RegExp behavior
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

- [ ] reference/test262/test/annexB/built-ins/RegExp passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for reference/test262/test/annexB/built-ins/RegExp
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 20
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

- `reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-not-capturing.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-invalid-control-escape-character-class-range.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-invalid-control-escape-character-class.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-leading-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-leading-escape.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape.js`
- `reference/test262/test/annexB/built-ins/RegExp/incomplete_hex_unicode_escape.js`

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
