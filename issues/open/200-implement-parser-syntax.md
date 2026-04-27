---
id: 200
title: "Implement parser syntax extensions"
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

Implement parser-syntax feature to handle 51 failing test cases in reference tests.

## Problem

Reference test results show 51 cases fail with parser-syntax diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

parser-syntax feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for parser-syntax feature
- [ ] Add fixtures for parser-syntax feature behavior
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

- [ ] parser-syntax feature passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for parser-syntax feature
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/manager reference-coverage tsgo --limit 102
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

- `reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/automaticTypeDirectiveResolutionBundler.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/checkInheritedProperty.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/circularDestructuring.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classExpressionWithComputedPropertyInLoop.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsAssignmentNamedEvaluation.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsNamedEvaluationDestructuringAssignment.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsPrivatePropertyAccessSameNameAsClass.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/constEnumInEmbeddedStatements.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/constructSignatureWithInferReturnType.ts`
- ... and 41 more files

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
