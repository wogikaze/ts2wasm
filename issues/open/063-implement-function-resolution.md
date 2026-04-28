---
id: 063
title: "Implement function resolution"
type: feature
area: frontend
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement function-resolution feature to handle 5 failing test cases in reference tests.

Problem: This issue overlaps with issue 062 and currently lists only Annex B dynamic Function constructor cases, so it is not an independent implementation slice.

Queue design note:

- Do not select this issue directly.
- Merge or cross-link these cases into the Function constructor diagnostic/policy child under issue 062, then close this issue as superseded.

## Problem

Reference test results show 5 cases fail with function-resolution diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

function-resolution feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for function-resolution feature
- [ ] Add fixtures for function-resolution feature behavior
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

- [ ] function-resolution feature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for function-resolution feature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 10
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

- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-no-line-terminator-html-close-comment-body.js`

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
