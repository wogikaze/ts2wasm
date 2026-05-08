---
id: 1350
title: "Implement Commentonexportenumdeclaration"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: [5277]
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1350.

## Summary

Closed after splitting the current module-syntax blocker into
`issues/done/5277-parse-export-enum-declarations-to-enum-boundary.md`.

## Problem

Reference test results show 1 cases fail in directory `commentOnExportEnumDeclaration` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: `commentOnExportEnumDeclaration.ts` fails at the generic static export
boundary before `export enum` can reach the existing enum-specific TypeScript
boundary.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 67..73
line 6, column 8
unsupported_features=import-export:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/done/5277-parse-export-enum-declarations-to-enum-boundary.md`.

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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5277-parse-export-enum-declarations-to-enum-boundary.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts`

## Duplicate detection

- `issues/done/432-implement-import-export.md` is a broad import/export bucket,
  not an executable implementation slice.
- `issues/done/055-implement-import-export.md` is a completed umbrella and
  explicitly leaves broader import/export forms outside the narrow completed
  subset.
- Existing export declaration slices cover `export function` and `export let`,
  not `export enum`.
- Existing enum issues cover enum transform/runtime or non-export enum syntax,
  not the generic `export enum` module boundary in this reference path.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage import export: commentOnExportEnumDeclaration

- Issue class: triage-needed
- Feature label: import-export
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts
```

Failure location:

```text
issue-055: unsupported static export; module resolution and loading are not implemented at 67..73
line 6, column 8
```

Source context:

```text
3 | /**
4 |  * comment
5 |  */
6 | export enum Color {
7 |     r, g, b
8 | }
```

Token evidence:

```text
Export, Ident("enum"), Ident("Color"), LeftBrace,
Ident("r"), Comma, Ident("g"), Comma, Ident("b"), RightBrace
```

TypeScript AST evidence:

```text
SourceFile
- EnumDeclaration "export enum Color { r, g, b }"
  - ExportKeyword "export"
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_features=import-export:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts
result: UnsupportedModule / unsupported-feature-boundary
date: 2026-05-06
```

Remaining risks:

- The child issue intentionally stops at the enum-specific frontend boundary;
  full enum transform/runtime semantics remain out of scope.
