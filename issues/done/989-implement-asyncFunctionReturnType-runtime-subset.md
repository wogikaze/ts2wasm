---
id: 989
title: "Implement Asyncfunctionreturntype Runtime Subset (dup)"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage asyncFunctionReturnType-runtime-subset across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncFunctionReturnType-runtime-subset` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncFunctionReturnType-runtime-subset has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts
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

- `reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts`

## Duplicate detection

- `issues/done/754-implement-asyncFunctionReturnType-runtime-subset.md` - Implement Asyncfunctionreturntype Runtime Subset (same reference path, same feature label, same group key, title overlap)
- `issues/done/756-implement-asyncFunctionWithForStatementNoInitializer.md` - Implement Asyncfunctionwithforstatementnoinitializer (same feature label, same group key, title overlap)
- `issues/done/760-implement-asyncImportNestedYield.md` - Implement Asyncimportnestedyield (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/754-implement-asyncFunctionReturnType-runtime-subset.md` に統合されました。
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
- `issues/done/989-implement-asyncFunctionReturnType-runtime-subset.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
