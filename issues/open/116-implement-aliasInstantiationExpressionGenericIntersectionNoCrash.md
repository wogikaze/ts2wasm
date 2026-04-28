---
id: 116
title: "Implement Aliasinstantiationexpressiongenericintersectionnocrash"
type: feature
area: frontend
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-29
---

## Summary

Implement aliasInstantiationExpressionGenericIntersectionNoCrash to handle 2 failing test cases in reference tests.

## Problem

Reference test results show 2 cases fail in directory `aliasInstantiationExpressionGenericIntersectionNoCrash` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

aliasInstantiationExpressionGenericIntersectionNoCrash is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for aliasInstantiationExpressionGenericIntersectionNoCrash
- [ ] Add fixtures for aliasInstantiationExpressionGenericIntersectionNoCrash behavior
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

- [ ] aliasInstantiationExpressionGenericIntersectionNoCrash passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for aliasInstantiationExpressionGenericIntersectionNoCrash
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
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

- `reference/TypeScript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts`
- `reference/TypeScript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts`

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
