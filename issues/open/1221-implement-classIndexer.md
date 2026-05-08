---
id: 1221
title: "Implement Classindexer"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1221.

## Summary

Closed as stale. Fresh coverage and smart triage show all five
`classIndexer` references are now build-pass, so there is no remaining compiler
blocker to split from this generated bucket.

## Problem

Reference test results previously showed 5 cases failing in directory
`classIndexer` with diagnostics: parser-syntax.

Problem: the representative family now builds successfully; the stale bucket
should not remain open as executable work.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classIndexer2.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter classIndexer --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm there is no remaining compiler blocker to split
- [x] Close the stale generated bucket
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is stale
- [x] No child issue needed because fresh coverage is all BuildPass
- [x] This issue includes representative paths, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 10
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classIndexer2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classIndexer2.ts
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

- `reference/typescript/tests/cases/compiler/classIndexer2.ts`
- `reference/typescript/tests/cases/compiler/classIndexer.ts`
- `reference/typescript/tests/cases/compiler/classIndexer3.ts`
- `reference/typescript/tests/cases/compiler/classIndexer5.ts`
- `reference/typescript/tests/cases/compiler/classIndexer4.ts`

Fresh coverage on 2026-05-06:

```text
build_pass=5
unsupported=0

build-pass: classIndexer.ts, classIndexer2.ts, classIndexer3.ts,
classIndexer4.ts, classIndexer5.ts
```

Representative source:

```ts
class C123 {
    [s: string]: number;
    x: number;
    y: string;
    constructor() {
    }
}
```

## Duplicate detection

- none needed; fresh coverage is all BuildPass

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter classIndexer --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classIndexer2.ts
```

Observed result on 2026-05-06:

```text
coverage: build_pass=5 unsupported=0
triage: BuildPass / ts2wasm build succeeded
tokens: ok; class index signature, typed fields, constructor
AST: ok; index signature and type-only fields erased, constructor retained
resolved: ok; ClassDecl C123 with constructor
TypeScript oracle for classIndexer2.ts: TS2564 and TS2411 semantic diagnostics
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale build-pass; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classIndexer2.ts
result: pass; BuildPass
date: 2026-05-06
```

Remaining risks:

- TypeScript still reports semantic diagnostics for `classIndexer2.ts`, but
  semantic parity is not enabled in this coverage window and no compiler build
  blocker remains.
