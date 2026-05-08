---
id: 3575
title: "Implement Nousedbeforedefinederrorintypecontext"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage noUsedBeforeDefinedErrorInTypeContext across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows the representative path now build-passes and TypeScript
reports no diagnostics.

Problem: the original unknown-unsupported blocker for
`noUsedBeforeDefinedErrorInTypeContext.ts` is stale.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noUsedBeforeDefinedErrorInTypeContext.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noUsedBeforeDefinedErrorInTypeContext.ts --detail
```

## Desired final state

This generated bucket is closed as stale build-pass evidence. Do not implement
directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm current behavior with focused coverage and triage
- [x] Close as stale build-pass evidence
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

- [x] Duplicate candidates below are confirmed as no-match or stale
- [x] Focused coverage shows `build_pass`
- [x] Focused triage shows TypeScript oracle diagnostics are empty
- [x] No child issue is needed for this representative path

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUsedBeforeDefinedErrorInTypeContext.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUsedBeforeDefinedErrorInTypeContext.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only stale bucket close.
- `cargo nextest run`; metadata-only stale bucket close.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noUsedBeforeDefinedErrorInTypeContext.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Build pass: noUsedBeforeDefinedErrorInTypeContext

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/noUsedBeforeDefinedErrorInTypeContext.ts`

Current evidence:

```text
coverage: build_pass=1 unsupported=0
triage: BuildPass / pass
TypeScript oracle: diagnostics=[]
```

Compiler evidence:

```text
tokens: ok through interface erasure, `as IThing<typeof ...>` type assertions, and typeof type-query tokens
ast: ok; runtime object literals for foo, baz, bar, qwe are preserved
resolved: ok
```

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUsedBeforeDefinedErrorInTypeContext.ts --detail --no-dashboard-data
result: pass; representative path reports build_pass
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUsedBeforeDefinedErrorInTypeContext.ts
result: pass; BuildPass and TypeScript oracle diagnostics=[]
date: 2026-05-08
```

Remaining risks:

- none
