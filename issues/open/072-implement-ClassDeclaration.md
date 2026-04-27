---
id: 072
title: "Implement Classdeclaration"
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

Implement ClassDeclaration to handle 11 failing test cases in reference tests.

## Problem

Reference test results show 11 cases fail in directory `ClassDeclaration` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

ClassDeclaration is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope:

- [ ] Add required syntax to lexer/parser
- [ ] Implement semantics for ClassDeclaration
- [ ] Add fixtures for ClassDeclaration behavior
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

- [ ] ClassDeclaration passes for basic cases
- [ ] Related diagnostics reduced in reference tests
- [ ] Regression test added for ClassDeclaration
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/manager reference-coverage tsc --limit 22
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

- `reference/TypeScript/tests/cases/compiler/ClassDeclaration10.ts`
- `reference/TypeScript/tests/cases/compiler/ClassDeclaration11.ts`
- `reference/TypeScript/tests/cases/compiler/ClassDeclaration13.ts`
- `reference/TypeScript/tests/cases/compiler/ClassDeclaration14.ts`
- `reference/TypeScript/tests/cases/compiler/ClassDeclaration15.ts`
- `reference/TypeScript/tests/cases/compiler/ClassDeclaration21.ts`
- `reference/TypeScript/tests/cases/compiler/ClassDeclaration22.ts`
- `reference/TypeScript/tests/cases/compiler/ClassDeclaration25.ts`
- `reference/TypeScript/tests/cases/compiler/ClassDeclaration26.ts`
- `reference/TypeScript/tests/cases/compiler/ClassDeclaration8.ts`
- ... and 1 more files

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
