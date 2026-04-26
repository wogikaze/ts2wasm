---
id: 094
title: "Implement Compiler (unknown-unsupported)"
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

Implement support for reference/typescript-go/testdata/tests/cases/compiler to handle 95 failing test cases in reference tests.

## Problem

Reference test results show 95 cases fail in directory `reference/typescript-go/testdata/tests/cases/compiler` with diagnostics: equality-operator, name-resolution, parser-syntax, unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

reference/typescript-go/testdata/tests/cases/compiler is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for reference/typescript-go/testdata/tests/cases/compiler
- [ ] Add fixtures for reference/typescript-go/testdata/tests/cases/compiler behavior
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

- [ ] reference/typescript-go/testdata/tests/cases/compiler passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for reference/typescript-go/testdata/tests/cases/compiler
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh tsgo --limit 190
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
- `reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/automaticTypeDirectiveResolutionBundler.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/bindingPatternOptionalParameterCached.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/blockedScopeVariableNotUnused1.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/checkInheritedProperty.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/circularControlFlowNarrowingWithCurrentElement01.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/circularDestructuring.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classExpressionWithComputedPropertyInLoop.ts`
- ... and 85 more files

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
