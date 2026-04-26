---
id: 091
title: "Implement Comments (unknown-unsupported)"
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

Implement support for reference/test262/test/annexB/language/comments to handle 8 failing test cases in reference tests.

## Problem

Reference test results show 8 cases fail in directory `reference/test262/test/annexB/language/comments` with diagnostics: parser-syntax, unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

reference/test262/test/annexB/language/comments is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for reference/test262/test/annexB/language/comments
- [ ] Add fixtures for reference/test262/test/annexB/language/comments behavior
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

- [ ] reference/test262/test/annexB/language/comments passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for reference/test262/test/annexB/language/comments
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 16
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

- `reference/test262/test/annexB/language/comments/multi-line-html-close.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-asi.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-first-line-1.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-first-line-2.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-first-line-3.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-unicode-separators.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close.js`
- `reference/test262/test/annexB/language/comments/single-line-html-open.js`

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
