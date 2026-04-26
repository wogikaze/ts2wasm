---
id: 092
title: "Implement Direct (parser-syntax)"
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

Implement support for reference/test262/test/annexB/language/eval-code/direct to handle 251 failing test cases in reference tests.

## Problem

Reference test results show 251 cases fail in directory `reference/test262/test/annexB/language/eval-code/direct` with diagnostics: function, parser-syntax, unknown-unsupported, unsupported-expression. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

reference/test262/test/annexB/language/eval-code/direct is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for reference/test262/test/annexB/language/eval-code/direct
- [ ] Add fixtures for reference/test262/test/annexB/language/eval-code/direct behavior
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

- [ ] reference/test262/test/annexB/language/eval-code/direct passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for reference/test262/test/annexB/language/eval-code/direct
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 502
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

- `reference/test262/test/annexB/language/eval-code/direct/block-decl-nostrict.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-fn-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-fn-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-no-skip-param.js`
- ... and 241 more files

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
