---
id: 1368
title: "Implement Commentsfunction"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed as a stale generated bucket after fresh triage showed the representative
reference case now builds successfully.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsFunction` with diagnostics: parser-syntax. Fresh focused coverage on
2026-05-07 reports `build_pass=1` and no unsupported diagnostics.

Problem: stale issue; `commentsFunction.ts` no longer has a current compiler
blocker.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsFunction.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsFunction.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
BuildPass: ts2wasm build succeeded
coverage: build_pass=1, unsupported=0, blocked=0
```

## Desired final state

This generated bucket is closed because the representative path is no longer
blocked.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm there is no current blocker to split
- [x] Close as stale build-pass bucket
- [x] Preserve exact reproduction commands and build-pass evidence

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
- [x] This issue contains an exact `reference-triage` command
- [x] This issue includes path, diagnostic code, source context, visible symbols, compiler dump evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and build-pass change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsFunction.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsFunction.ts
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

- `reference/typescript/tests/cases/compiler/commentsFunction.ts`

## Duplicate detection

- Only self-match found by fresh triage. No current child issue is needed
  because the representative path now builds.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Build pass: commentsFunction

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/commentsFunction.ts
```

Source context:

```ts
/** This comment should appear for foo*/
function foo() {
} /* trailing comment of function */
foo();
```

Compiler evidence:

```text
tokens: ok
ast: ok, includes functions, function expression, arrow functions, and calls
resolved: ok
build: ts2wasm build succeeded
```

TypeScript oracle:

```text
ok: true
diagnostics: []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsFunction.ts --detail --no-dashboard-data
result: build_pass=1, unsupported=0, blocked=0
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsFunction.ts
result: BuildPass; ts2wasm build succeeded
date: 2026-05-07
```

Remaining risks:

- Semantic/declaration emit parity is not proven by build-only coverage; this
  issue only tracked the stale compiler blocker.
