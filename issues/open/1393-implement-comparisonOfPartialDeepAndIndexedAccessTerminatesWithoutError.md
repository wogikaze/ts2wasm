---
id: 1393
title: "Implement Comparisonofpartialdeepandindexedaccessterminateswithouterror"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1393.

## Summary

Closed as a stale generated bucket after fresh triage showed the representative
now builds successfully.

`comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError.ts` no longer
reports the generated `unknown-unsupported` blocker. Current focused coverage
reports `build_pass=1`, `unsupported=0`, and `blocked=0`.

## Problem

Reference test results originally showed 1 case failing in directory
`comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError` with diagnostics:
unknown-unsupported. Fresh focused triage on 2026-05-07 reports `BuildPass`.

Problem: the generated blocker is stale; there is no current compiler blocker
to split into an implementation-ready child issue.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
Smart triage: Build pass: comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError
coverage: executed=1, build_pass=1, unsupported=0, blocked=0
semantic_enabled=0
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
diagnostics: []
```

## Desired final state

This generated bucket is closed. No implementation issue is created because the
current compiler build and TypeScript oracle both accept the representative
source.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close this stale generated bucket
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Creating a child issue without a current failing diagnostic

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
- [x] Focused triage reports `BuildPass`
- [x] Focused coverage reports `build_pass=1`
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError.ts`

## Duplicate detection

- No matching implementation-ready issue is needed because the current build
  passes.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Build pass: comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError.ts
```

Source context:

```text
type PartialDeep<T> = {[K in keyof T]?: PartialDeep<T[K]>};
type Many<T> = T | readonly T[];

interface Collection<T> {
    sortBy(...iteratees: Many<PartialDeep<T>>[]): Collection<T>;
}

const x: Collection<{x: number}> =
  (null as any as Collection<{x: number, y: number}>);

export {};
```

AST/resolved evidence:

```text
AST: Let x = Null; ExportNamed {}
Resolved: ok=True; Let("x", Null), Expr(Undefined)
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
diagnostics: []
```

## Completion evidence

Commits:

- stale generated bucket; current build passes

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError.ts
result: pass; BuildPass, no compiler blocker found
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/comparisonOfPartialDeepAndIndexedAccessTerminatesWithoutError.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-07
```

Remaining risks:

- `semantic_enabled=0` for this focused tsc coverage run, so this closure only
  claims the generated unknown-unsupported blocker is gone.
