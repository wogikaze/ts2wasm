---
id: 4808
title: "Implement import/export module syntax (dup)"
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

Triage import-export feature across 75 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 75 cases fail with import-export diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: import-export feature has 75 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/assertionWithNoArgument.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/assertionWithNoArgument.ts --detail
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
mise run reference-coverage -- tsgo --limit 150
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/assertionWithNoArgument.ts --detail
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/assertionWithNoArgument.ts
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

- `reference/typescript-go/testdata/tests/cases/compiler/assertionWithNoArgument.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/blockedScopeVariableNotUnused1.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/bindingPatternOptionalParameterCached.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/allowSyntheticDefaultImports9.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsPrivatePropertyAccessSameNameAsClass.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/conditionalContextualReturnSubstitutionCache.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsPropertyAccessSameNameAsClass.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitAugmentationUsesCorrectSourceFile.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitExpandoFunction.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitBigInt.ts`
- ... and 65 more files

## Duplicate detection

- `issues/done/075-implement-ExportAssignment.md` - Implement Exportassignment (same feature label, same group key, title overlap)
- `issues/done/1001-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, same group key, title overlap)
- `issues/done/1003-implement-augmentedTypesExternalModule.md` - Implement Augmentedtypesexternalmodule (same feature label, same group key, title overlap)
- `issues/done/1006-implement-augmentedTypesModules.md` - Implement Augmentedtypesmodules (same feature label, same group key, title overlap)
- `issues/done/1015-implement-avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.md` - Implement Avoidnarrowingusingconstvariablefrombindingelementwithliteralinitializer (same feature label, same group key, title overlap)
- `issues/done/1028-implement-badExternalModuleReference.md` - Implement Badexternalmodulereference (same feature label, same group key, title overlap; superseded by issue 232)
- `issues/done/1032-implement-bangInModuleName.md` - Implement Banginmodulename (same feature label, same group key, title overlap; superseded by issue 232)
- `issues/done/1035-implement-baseConstraintOfDecorator.md` - Implement Baseconstraintofdecorator (same feature label, same group key, title overlap; superseded by issue 5144)
- `issues/done/1057-implement-bind.md` - Implement Bind (same feature label, same group key, title overlap)
- `issues/done/1063-implement-bindingPatternOmittedExpressionNesting.md` - Implement Bindingpatternomittedexpressionnesting (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/432-implement-import-export.md` に統合されました。
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
- `issues/done/4808-implement-import-export.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
