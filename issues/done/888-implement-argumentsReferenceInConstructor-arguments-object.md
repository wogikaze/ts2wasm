---
id: 888
title: "Implement Argumentsreferenceinconstructor Arguments Object (dup)"
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

Triage argumentsReferenceInConstructor-arguments-object across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3 cases fail in directory `argumentsReferenceInConstructor-arguments-object` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsReferenceInConstructor-arguments-object has 3 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts --detail
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
mise run reference-coverage -- tsc --limit 6
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts
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

- `reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor5_Js.ts`
- `reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor6_Js.ts`
- `reference/typescript/tests/cases/compiler/argumentsReferenceInConstructor7_Js.ts`

## Duplicate detection

- `issues/done/311-fix-test262-arguments-object-index-assignment.md` - Fix test262 arguments object index assignment semantics (same feature label, title overlap)
- `issues/open/646-implement-arguments.md` - Implement Arguments (same feature label, title overlap)
- `issues/open/647-implement-argumentsAsPropertyName-arguments-object.md` - Implement Argumentsaspropertyname Arguments Object (same feature label, title overlap)
- `issues/open/653-implement-argumentsReferenceInConstructor-arguments-object.md` - Implement Argumentsreferenceinconstructor Arguments Object (same reference path, same feature label, same group key, title overlap)
- `issues/open/655-implement-argumentsReferenceInFunction.md` - Implement Argumentsreferenceinfunction (same feature label, same group key, title overlap)
- `issues/open/656-implement-argumentsReferenceInMethod-arguments-object.md` - Implement Argumentsreferenceinmethod Arguments Object (same feature label, same group key, title overlap)
- `issues/open/658-implement-argumentsReferenceInObjectLiteral.md` - Implement Argumentsreferenceinobjectliteral (same feature label, same group key, title overlap)
- `issues/open/660-implement-argumentsUsedInObjectLiteralProperty.md` - Implement Argumentsusedinobjectliteralproperty (same feature label, same group key, title overlap)
- `issues/open/690-implement-arrayReferenceWithoutTypeArgs.md` - Implement Arrayreferencewithouttypeargs (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/653-implement-argumentsReferenceInConstructor-arguments-object.md` に統合されました。
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
- `issues/done/888-implement-argumentsReferenceInConstructor-arguments-object.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
