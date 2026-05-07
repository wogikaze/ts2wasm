---
id: 1413
title: "Implement Computedpropertynameandtypeparameterconflict"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1413.

## Summary

Triage computedPropertyNameAndTypeParameterConflict across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results originally showed 1 case failing in directory `computedPropertyNameAndTypeParameterConflict` with diagnostics: parser-syntax. Fresh triage on 2026-05-07 shows this bucket is stale: the representative case now builds successfully.

Problem: computedPropertyNameAndTypeParameterConflict has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/computedPropertyNameAndTypeParameterConflict.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyNameAndTypeParameterConflict.ts --detail
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
- [x] No child issue is needed because fresh triage is `BuildPass`
- [x] Completion evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyNameAndTypeParameterConflict.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/computedPropertyNameAndTypeParameterConflict.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/computedPropertyNameAndTypeParameterConflict.ts`

## Duplicate detection

- No matching open implementation issue is needed because the representative case is now `build_pass`.
- Search for the reference path and issue id found only this generated bucket.

## Smart triage

Generated on 2026-05-07:

```text
Issue class: none
Feature label: build-pass
Diagnostic: BuildPass / pass
Path: reference/typescript/tests/cases/compiler/computedPropertyNameAndTypeParameterConflict.ts
Failure location: ts2wasm build succeeded
Visible symbols before failure:
- binding O
- class Bar
```

Compiler evidence:

```text
tokens: ok; declare const O: unique symbol; declare class Bar<O> { [O]: number; }
ast: ok
resolved: ok
TypeScript oracle: ok, diagnostics=[]
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyNameAndTypeParameterConflict.ts --detail --no-dashboard-data
result: executed=1, build_pass=1, unsupported=0, blocked=0, fail=0
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Stale bucket closed; no child issue required.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
