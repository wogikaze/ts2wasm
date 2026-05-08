---
id: 2026
title: "Implement Downlevelletconst Parser Syntax"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage downlevelLetConst-parser-syntax across 15 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 15 cases fail in directory `downlevelLetConst-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: downlevelLetConst-parser-syntax has 15 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/downlevelLetConst1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/downlevelLetConst1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 30
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/downlevelLetConst1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/downlevelLetConst1.ts
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

- `reference/typescript/tests/cases/compiler/downlevelLetConst1.ts`
- `reference/typescript/tests/cases/compiler/downlevelLetConst12.ts`
- `reference/typescript/tests/cases/compiler/downlevelLetConst15.ts`
- `reference/typescript/tests/cases/compiler/downlevelLetConst11.ts`
- `reference/typescript/tests/cases/compiler/downlevelLetConst10.ts`
- `reference/typescript/tests/cases/compiler/downlevelLetConst14.ts`
- `reference/typescript/tests/cases/compiler/downlevelLetConst16.ts`
- `reference/typescript/tests/cases/compiler/downlevelLetConst19.ts`
- `reference/typescript/tests/cases/compiler/downlevelLetConst5.ts`
- `reference/typescript/tests/cases/compiler/downlevelLetConst3.ts`
- ... and 5 more files

## Duplicate detection

- `issues/done/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/done/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/done/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/done/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/done/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/done/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

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
