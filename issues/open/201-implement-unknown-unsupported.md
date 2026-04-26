---
id: 201
title: "Investigate and classify unknown-unsupported cases"
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

Implement unknown-unsupported feature to handle 41 failing test cases in reference tests.

## Problem

Reference test results show 41 cases fail with unknown-unsupported diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

unknown-unsupported feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for unknown-unsupported feature
- [ ] Add fixtures for unknown-unsupported feature behavior
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

- [ ] unknown-unsupported feature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for unknown-unsupported feature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh tsgo --limit 82
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

- `reference/typescript-go/testdata/tests/cases/compiler/allowSyntheticDefaultImports9.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/assertionWithNoArgument.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/bindingPatternOptionalParameterCached.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/blockedScopeVariableNotUnused1.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsPropertyAccessSameNameAsClass.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/conditionalContextualReturnSubstitutionCache.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/contextuallyTypedJsxChildren2.tsx`
- `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitAsConstSatisfiesNonReadonlyResult.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitBigInt.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitConstObjectLiteralGenericMethod1.ts`
- ... and 31 more files

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
