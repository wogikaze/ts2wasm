---
id: 3482
title: "Implement Nestedloopwithonlyinnerletcaptured"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nestedLoopWithOnlyInnerLetCaptured across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed after refreshed evidence showed the current blocker is not closure
capture yet. The representative is blocked by unbraced nested `for..of` loop
body parsing, split to
`issues/open/5461-parse-nested-single-statement-for-of-loop-bodies.md`.

## Problem

Reference test results show 1 cases fail in directory `nestedLoopWithOnlyInnerLetCaptured` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: nestedLoopWithOnlyInnerLetCaptured has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to `issues/open/5461-parse-nested-single-statement-for-of-loop-bodies.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts
result: pass; UnsupportedSyntax / parser-or-frontend-unsupported at the inner unbraced `for`
date: 2026-05-08
```

Remaining risks:

- Follow-up parser implementation remains open in issue 5461.
