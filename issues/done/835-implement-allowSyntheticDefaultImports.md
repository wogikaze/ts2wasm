---
id: 835
title: "Implement Allowsyntheticdefaultimports (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage allowSyntheticDefaultImports across 10 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 10 cases fail in directory `allowSyntheticDefaultImports` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: allowSyntheticDefaultImports has 10 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts --detail
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
mise run reference-coverage -- tsc --limit 20
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts
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

- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports2.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports4.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports10.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports3.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports6.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports7.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports5.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports9.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports8.ts`

## Duplicate detection

- `issues/open/135-implement-allowSyntheticDefaultImports.md` - Implement Allowsyntheticdefaultimports (same reference path, same feature label, same group key, title overlap)
- `issues/open/136-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` - Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (same feature label, same group key, title overlap)
- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, same group key, title overlap)
- `issues/open/462-implement-ExportAssignment.md` - Implement Exportassignment (same feature label, same group key, title overlap)
- `issues/open/514-implement-allowSyntheticDefaultImports.md` - Implement Allowsyntheticdefaultimports (same reference path, same feature label, same group key, title overlap)
- `issues/open/515-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` - Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (same feature label, same group key, title overlap)
- `issues/open/548-implement-ExportAssignment.md` - Implement Exportassignment (same feature label, same group key, title overlap)
- `issues/open/600-implement-allowSyntheticDefaultImports.md` - Implement Allowsyntheticdefaultimports (same reference path, same feature label, same group key, title overlap)
- `issues/open/601-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` - Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (same feature label, same group key, title overlap)
- `issues/open/715-implement-assign.md` - Implement Assign (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/600-implement-allowSyntheticDefaultImports.md` に統合されました。
そちらを参照してください。
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
