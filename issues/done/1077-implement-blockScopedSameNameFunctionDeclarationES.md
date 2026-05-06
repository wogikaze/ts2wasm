---
id: 1077
title: "Implement Blockscopedsamenamefunctiondeclarationes"
type: spike
area: reference/triage
class: superseded
priority: P1
depends_on: [5188]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedSameNameFunctionDeclarationES across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `blockScopedSameNameFunctionDeclarationES` with diagnostics: arity. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedSameNameFunctionDeclarationES has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES5.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES5.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES5.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES5.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/done/5188-report-block-scoped-function-call-arity-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES5.ts`
- `reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES6.ts`

## Duplicate detection

- `issues/done/413-implement-arity.md` and `issues/done/5062-implement-arity.md` split builtin arity work and do not own this user-defined function false-pass.
- `issues/open/2460-implement-functionParameterArityMismatch.md` is an untriaged generated bucket, not an implementation-ready owner for this same-name block-scoped function pattern.

## Smart triage

Generated on 2026-05-06.

- Paths:
  - `reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES5.ts`
  - `reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES6.ts`
- Diagnostic: `BuildPass` / `pass`
- Current compiler message: `ts2wasm build succeeded`
- Source context: outer `function foo(a: number)` contains block-scoped `function foo() {}` in each branch, then calls `foo()` and `foo(10)` inside and outside the block.
- Compiler evidence: tokens, AST, and resolved IR succeed; resolved IR contains source spans for the wrong-arity call sites but no diagnostic is emitted.
- TypeScript oracle: four `TS2554` diagnostics per file, alternating `Expected 0 arguments, but got 1.` and `Expected 1 arguments, but got 0.`
- Superseding child: `issues/done/5188-report-block-scoped-function-call-arity-diagnostics.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES5.ts
result: pass; current blocker is a false-pass missing user-defined function arity diagnostics, split to issue 5188
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES6.ts
result: pass; current blocker is a false-pass missing user-defined function arity diagnostics, split to issue 5188
date: 2026-05-06
```

Remaining risks:

- none
