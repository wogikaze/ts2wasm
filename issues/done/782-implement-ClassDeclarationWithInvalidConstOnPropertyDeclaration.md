---
id: 782
title: "Implement Classdeclarationwithinvalidconstonpropertydeclaration"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage ClassDeclarationWithInvalidConstOnPropertyDeclaration across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `ClassDeclarationWithInvalidConstOnPropertyDeclaration` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ClassDeclarationWithInvalidConstOnPropertyDeclaration has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts --detail
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

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration2.ts`

## Duplicate detection

- `issues/done/072-implement-ClassDeclaration.md` - Implement Classdeclaration (same feature label, same group key, title overlap)
- `issues/done/073-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` - Implement Classdeclarationwithinvalidconstonpropertydeclaration (same reference path, same feature label, same group key, title overlap)
- `issues/open/199-implement-reference-typescript-tests-cases-compiler.md` - Implement Compiler (same feature label, same group key, title overlap)
- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, same group key, title overlap)
- `issues/done/460-implement-ClassDeclaration.md` - Implement Classdeclaration (same feature label, same group key, title overlap)
- `issues/done/461-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` - Implement Classdeclarationwithinvalidconstonpropertydeclaration (same reference path, same feature label, same group key, title overlap)
- `issues/open/546-implement-ClassDeclaration.md` - Implement Classdeclaration (same feature label, same group key, title overlap)
- `issues/done/547-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` - Implement Classdeclarationwithinvalidconstonpropertydeclaration (same reference path, same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.

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

## Status

Superseded by issue #073. Duplicate from separate coverage run.
