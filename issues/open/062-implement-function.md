---
id: 062
title: "Implement function support"
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

Implement function feature to handle 47 failing test cases in reference tests.

## Problem

Reference test results show 47 cases fail with function diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

function feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for function feature
- [ ] Add fixtures for function feature behavior
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

- [ ] function feature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for function feature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/manager reference-coverage test262 --limit 94
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

- `reference/test262/test/annexB/built-ins/Function/createdynfn-no-line-terminator-html-close-comment-params.js`
- `reference/test262/test/annexB/built-ins/String/prototype/anchor/attr-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/anchor/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/big/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/blink/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/bold/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fixed/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontcolor/attr-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontcolor/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/attr-tostring-err.js`
- ... and 37 more files

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
