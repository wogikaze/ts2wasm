---
id: 967
title: "Implement Assignmentcompatability Import Export (dup)"
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

Triage assignmentCompatability-import-export across 43 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 43 cases fail in directory `assignmentCompatability-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompatability-import-export has 43 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatability1.ts --detail
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
mise run reference-coverage -- tsc --limit 86
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatability1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability1.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompatability1.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability10.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability11.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability13.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability12.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability16.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability15.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability14.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability17.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability18.ts`
- ... and 33 more files

## Duplicate detection

- `issues/done/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/done/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/done/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/done/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same reference path, same feature label, same group key, title overlap)
- `issues/done/743-implement-assignmentToFunction.md` - Implement Assignmenttofunction (same feature label, same group key, title overlap)
- `issues/done/745-implement-assignmentToObjectAndFunction.md` - Implement Assignmenttoobjectandfunction (same feature label, same group key, title overlap)
- `issues/done/747-implement-assignmentToReferenceTypes.md` - Implement Assignmenttoreferencetypes (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/732-implement-assignmentCompatability-import-export.md` に統合されました。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/967-implement-assignmentCompatability-import-export.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
