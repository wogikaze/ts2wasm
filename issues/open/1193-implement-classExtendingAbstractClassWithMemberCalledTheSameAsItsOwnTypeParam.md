---
id: 1193
title: "Implement Classextendingabstractclasswithmembercalledthesameasitsowntypeparam"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1193.

## Summary

Triage classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts --detail
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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5312-parse-export-abstract-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-07.

```text
### Smart triage: Triage import export: classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam

- Feature label: import-export
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts
```

Failure location:

```text
line 19, column 1
issue-055: unsupported static export; module resolution and loading are not implemented at 356..362
```

Source context:

```ts
export abstract class ConvenientObservable<T, TChange> implements IObservable<T, TChange> {
    get TChange(): TChange { return null!; }
    public abstract get(): T;
}
```

Compiler evidence:

```text
tokens: ok; includes Export, Abstract, Class, Ident("ConvenientObservable"), generic parameters, implements clause, and members
ast: fails with UnsupportedModule issue-055 at the export keyword
resolved: fails with the same issue-055 diagnostic
```

TypeScript oracle:

```text
ok: true
diagnostics: []
topLevel includes exported InterfaceDeclaration, exported ClassDeclaration ConvenientObservable, and exported ClassDeclaration BaseObservable
```

Split child: `issues/open/5312-parse-export-abstract-class-declarations.md`.

Related issues are no-match for this exact parser blocker:

- Issue 5232 owns module-build behavior after `ExportDecl(ClassDecl)` exists.
- Issue 1213 covered `declare abstract class`, not `export abstract class`.
- Issue 432 is the broad generated import/export bucket and is too wide to implement directly.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts
result: pass; current blocker identified as export abstract class parser/static-export boundary, split to issue 5312
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts --detail --no-dashboard-data
result: pass; unsupported=1 with UnsupportedSyntax/unknown-unsupported for the same representative reference path
date: 2026-05-07
```

Remaining risks:

- After issue 5312 advances, the path may expose issue 5232 or later abstract-class/type-parameter semantics.
