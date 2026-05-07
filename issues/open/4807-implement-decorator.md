---
id: 4807
title: "Implement decorator support"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage decorator feature across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail with decorator diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: decorator feature has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/esDecoratorExtendsNull.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/esDecoratorExtendsNull.ts --detail
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
mise run reference-coverage -- tsgo --limit 8
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/esDecoratorExtendsNull.ts --detail
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/esDecoratorExtendsNull.ts
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

- `reference/typescript-go/testdata/tests/cases/compiler/esDecoratorExtendsNull.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/esDecoratorsPropertyAccessSameNameAsClass.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/legacyDecoratorClassWithoutModifiers.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/parameterDecoratorInJsFile.ts`

## Duplicate detection

- `issues/done/1035-implement-baseConstraintOfDecorator.md` - Implement Baseconstraintofdecorator (same feature label, same group key, title overlap; superseded by issue 5144)
- `issues/open/1186-implement-classExpressionWithDecorator.md` - Implement Classexpressionwithdecorator (same feature label, same group key, title overlap; split to issue 5253)
- `issues/open/1892-implement-decoratorInJsFile.md` - Implement Decoratorinjsfile (same feature label, same group key, title overlap)
- `issues/open/1893-implement-decoratorMetadataConditionalType.md` - Implement Decoratormetadataconditionaltype (same feature label, same group key, title overlap)
- `issues/open/1894-implement-decoratorMetadataElidedImport.md` - Implement Decoratormetadataelidedimport (same feature label, same group key, title overlap)
- `issues/open/1895-implement-decoratorMetadataElidedImportOnDeclare.md` - Implement Decoratormetadataelidedimportondeclare (same feature label, same group key, title overlap)
- `issues/open/1896-implement-decoratorMetadataForMethodWithNoReturnTypeAnnotation.md` - Implement Decoratormetadataformethodwithnoreturntypeannotation (same feature label, same group key, title overlap)
- `issues/open/1897-implement-decoratorMetadataGenericTypeVariable.md` - Implement Decoratormetadatagenerictypevariable (same feature label, same group key, title overlap)
- `issues/open/1898-implement-decoratorMetadataGenericTypeVariableDefault.md` - Implement Decoratormetadatagenerictypevariabledefault (same feature label, same group key, title overlap)
- `issues/open/1899-implement-decoratorMetadataGenericTypeVariableInScope.md` - Implement Decoratormetadatagenerictypevariableinscope (same feature label, same group key, title overlap)

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
