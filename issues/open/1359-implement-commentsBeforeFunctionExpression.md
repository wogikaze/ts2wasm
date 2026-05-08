---
id: 1359
title: "Implement Commentsbeforefunctionexpression"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1359.

## Summary

Closed as stale: the representative reference case now builds successfully.

## Problem

Reference test results previously showed 1 case failing in directory
`commentsBeforeFunctionExpression` with parser-syntax diagnostics. Fresh
focused coverage now reports `build_pass` for the representative case.

Problem: stale generated bucket; no current compiler blocker was reproduced for
`commentsBeforeFunctionExpression1.ts`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsBeforeFunctionExpression1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsBeforeFunctionExpression1.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
build_pass=1
unsupported=0
reference/typescript/tests/cases/compiler/commentsBeforeFunctionExpression1.ts: build_pass
```

## Desired final state

This generated bucket is closed. No child issue is needed unless semantic parity
coverage later exposes a concrete behavior gap.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Confirm the representative reference window now builds
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is stale
- [x] Exact `reference-triage` command is preserved
- [x] This close record includes path, build-pass result, visible symbols, parser AST, and TypeScript oracle evidence
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsBeforeFunctionExpression1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsBeforeFunctionExpression1.ts
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

- `reference/typescript/tests/cases/compiler/commentsBeforeFunctionExpression1.ts`

## Duplicate detection

- Only self-match found by path. No child issue created because focused
  coverage and triage now report `BuildPass`.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Build pass: commentsBeforeFunctionExpression1

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/commentsBeforeFunctionExpression1.ts
```

Source context:

```ts
var v = {
    f: /**own f*/ (a) => 0
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; object literal property `f` is an arrow function
resolved: ok; binding v resolves to object with arrow function property
build: ok
```

TypeScript oracle:

```text
ok: true
diagnostics: []
binding v: { f: (a: any) => number; }
parameter a: any
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsBeforeFunctionExpression1.ts --detail --no-dashboard-data
result: build_pass=1, unsupported=0
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsBeforeFunctionExpression1.ts
result: BuildPass; tokens/AST/resolved and TypeScript oracle succeeded
date: 2026-05-06
```

Remaining risks:

- Semantic parity was not enabled in this focused coverage run; this close only
  removes the stale compiler-blocker bucket.
