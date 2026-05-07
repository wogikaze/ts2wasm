---
id: 1349
title: "Implement Commentonelidedmodule"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1349.

## Summary

Closed as a stale generated reference bucket. The representative
`commentOnElidedModule1.ts` case now builds successfully and smart triage
reports `BuildPass`, so there is no current compiler blocker to split from this
bucket.

## Problem

Reference test results show 1 cases fail in directory `commentOnElidedModule` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: stale generated bucket; no implementation-ready child issue is needed
for the current representative path.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnElidedModule1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnElidedModule1.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
reference/typescript/tests/cases/compiler/commentOnElidedModule1.ts: build_pass
Diagnostic: BuildPass / pass
Feature label: build-pass
TypeScript oracle: ok, no diagnostics
```

## Desired final state

This generated bucket is closed as stale with the current smart-triage and
coverage evidence preserved.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale when the representative path is `BuildPass`
- [x] Preserve exact reproduction commands and representative evidence

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded/stale
- [x] Exact `reference-triage` command is preserved
- [x] Current path, diagnostic code, source context, visible symbols, parser AST, and TypeScript oracle evidence are recorded
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnElidedModule1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnElidedModule1.ts
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

- `reference/typescript/tests/cases/compiler/commentOnElidedModule1.ts`

## Duplicate detection

- Smart triage found only this issue by exact path.
- The existing dependency on issue 432 is not needed for this generated bucket
  because the current representative path reaches `BuildPass`.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Build pass: commentOnElidedModule1

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/commentOnElidedModule1.ts
```

Source context:

```ts
// @target: es2015
//@filename: a.ts
/*!=================
    Keep this pinned
   =================
*/

/*! Don't keep this pinned comment */
namespace ElidedModule {
}
```

Parser evidence:

```text
tokens: ok
ast: ok, []
resolved: ok, []
```

TypeScript oracle:

```text
ok: true
diagnostics: []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnElidedModule1.ts --detail --no-dashboard-data
result: build_pass=1, semantic_pass=0, fail=0
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnElidedModule1.ts
result: BuildPass / pass
date: 2026-05-06
```

Remaining risks:

- Semantic coverage is not enabled for this path, so this closure only proves
  the generated build-blocker bucket is stale.
