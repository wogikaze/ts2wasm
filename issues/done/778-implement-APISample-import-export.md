---
id: 778
title: "Implement Apisample Import Export (dup)"
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

Triage APISample-import-export across 7 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 7 cases fail in directory `APISample-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: APISample-import-export has 7 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts --detail
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
mise run reference-coverage -- tsc --limit 14
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts
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

- `reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts`
- `reference/typescript/tests/cases/compiler/APISample_compile.ts`
- `reference/typescript/tests/cases/compiler/APISample_WatchWithDefaults.ts`
- `reference/typescript/tests/cases/compiler/APISample_linter.ts`
- `reference/typescript/tests/cases/compiler/APISample_transform.ts`
- `reference/typescript/tests/cases/compiler/APISample_Watch.ts`
- `reference/typescript/tests/cases/compiler/APISample_parseConfig.ts`

## Duplicate detection

- `issues/open/070-implement-APISample.md` - Implement Apisample (same reference path, title overlap)
- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/open/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same reference path, same feature label, same group key, title overlap)
- `issues/open/462-implement-ExportAssignment.md` - Implement Exportassignment (same feature label, same group key, title overlap)
- `issues/open/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, same group key, title overlap)
- `issues/open/475-implement-acceptableAlias.md` - Implement Acceptablealias (same feature label, same group key, title overlap)
- `issues/open/481-implement-accessorDeclarationEmitVisibilityErrors.md` - Implement Accessordeclarationemitvisibilityerrors (same feature label, same group key, title overlap)
- `issues/done/483-implement-accessorInAmbientContextES.md` - Implement Accessorinambientcontextes (same feature label, same group key, title overlap)
- `issues/open/484-implement-accessorInferredReturnTypeErrorInReturnStatement.md` - Implement Accessorinferredreturntypeerrorinreturnstatement (same feature label, same group key, title overlap)
- `issues/done/489-implement-accessorsInAmbientContext.md` - Implement Accessorsinambientcontext (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/543-implement-APISample-import-export.md` に統合されました。
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
