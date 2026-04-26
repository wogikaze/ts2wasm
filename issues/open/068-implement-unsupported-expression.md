---
id: 068
title: "Implement unsupported expression types"
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

Implement unsupported-expression feature to handle 23 failing test cases in reference tests.

## Problem

Reference test results show 23 cases fail with unsupported-expression diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

unsupported-expression feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for unsupported-expression feature
- [ ] Add fixtures for unsupported-expression feature behavior
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

- [ ] unsupported-expression feature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for unsupported-expression feature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 46
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

- `reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/big/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/blink/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/bold/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fixed/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontcolor/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/italics/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/link/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/small/not-a-constructor.js`
- ... and 13 more files

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
