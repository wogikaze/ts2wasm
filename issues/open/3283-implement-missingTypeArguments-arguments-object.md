---
id: 3283
title: "Implement Missingtypearguments Arguments Object"
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

Triage missingTypeArguments-arguments-object across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `missingTypeArguments-arguments-object` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: missingTypeArguments-arguments-object has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/missingTypeArguments2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/missingTypeArguments2.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/missingTypeArguments2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/missingTypeArguments2.ts
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

- `reference/typescript/tests/cases/compiler/missingTypeArguments2.ts`
- `reference/typescript/tests/cases/compiler/missingTypeArguments1.ts`

## Duplicate detection

- `issues/done/311-fix-test262-arguments-object-index-assignment.md` - Fix test262 arguments object index assignment semantics (same feature label, title overlap)
- `issues/done/646-implement-arguments.md` - Implement Arguments (same feature label, title overlap)
- `issues/done/647-implement-argumentsAsPropertyName-arguments-object.md` - Implement Argumentsaspropertyname Arguments Object (same feature label, title overlap)
- `issues/done/653-implement-argumentsReferenceInConstructor-arguments-object.md` - Implement Argumentsreferenceinconstructor Arguments Object (same feature label, title overlap)
- `issues/done/656-implement-argumentsReferenceInMethod-arguments-object.md` - Implement Argumentsreferenceinmethod Arguments Object (same feature label, title overlap)

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
