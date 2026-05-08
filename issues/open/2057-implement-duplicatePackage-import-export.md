---
id: 2057
title: "Implement Duplicatepackage Import Export"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage duplicatePackage-import-export across 5 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 5 cases fail in directory `duplicatePackage-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: duplicatePackage-import-export has 5 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/duplicatePackage.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/duplicatePackage.ts --detail
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
mise run reference-coverage -- tsc --limit 10
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/duplicatePackage.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/duplicatePackage.ts
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

- `reference/typescript/tests/cases/compiler/duplicatePackage.ts`
- `reference/typescript/tests/cases/compiler/duplicatePackage_globalMerge.ts`
- `reference/typescript/tests/cases/compiler/duplicatePackage_packageIdIncludesSubModule.ts`
- `reference/typescript/tests/cases/compiler/duplicatePackage_relativeImportWithinPackage_scoped.ts`
- `reference/typescript/tests/cases/compiler/duplicatePackage_relativeImportWithinPackage.ts`

## Duplicate detection

- `issues/done/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/done/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/done/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/done/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/done/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

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
