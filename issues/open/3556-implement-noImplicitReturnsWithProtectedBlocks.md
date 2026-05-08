---
id: 3556
title: "Implement Noimplicitreturnswithprotectedblocks"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as stale. Fresh coverage shows all three
`noImplicitReturnsWithProtectedBlocks` reference files now build successfully,
so this generated name-resolution blocker no longer needs a child issue.

## Problem

Older reference results showed 3 cases failing in directory
`noImplicitReturnsWithProtectedBlocks` with diagnostics `name-resolution`.
Fresh evidence on 2026-05-08 shows all three files are `build_pass`:

```text
reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks1.ts: build_pass
reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks2.ts: build_pass
reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks3.ts: build_pass
```

Problem: no current compiler blocker remains for this generated bucket.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks3.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=3 build_pass=3 unsupported=0 blocked=0
triage noImplicitReturnsWithProtectedBlocks1.ts: BuildPass
triage noImplicitReturnsWithProtectedBlocks2.ts: BuildPass
triage noImplicitReturnsWithProtectedBlocks3.ts: BuildPass
```

## Desired final state

This generated bucket is closed as stale. Do not implement directly from this
bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm no current name-resolution blocker remains
- [x] Close the stale generated bucket without creating a child issue
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is closed stale
- [x] This done issue contains exact focused `reference-triage` commands
- [x] Evidence includes all affected paths, build-pass status, visible symbols, and parser/TypeScript AST evidence
- [x] No child issue was created because there is no current name-resolution blocker

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks3.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only.
- `cargo nextest run`; issue metadata only.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks2.ts`
- `reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks1.ts`
- `reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks3.ts`

## Duplicate detection

- none needed; all representative files are current build passes.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Build pass: noImplicitReturnsWithProtectedBlocks1

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks1.ts

### Smart triage: Build pass: noImplicitReturnsWithProtectedBlocks2

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks2.ts

### Smart triage: Build pass: noImplicitReturnsWithProtectedBlocks3

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks3.ts
```

Compiler evidence:

```text
noImplicitReturnsWithProtectedBlocks1.ts: tokens/AST/resolved ok for try/finally with return in try
noImplicitReturnsWithProtectedBlocks2.ts: tokens/AST/resolved ok for try/catch/finally
noImplicitReturnsWithProtectedBlocks3.ts: tokens/AST/resolved ok for try/catch
```

TypeScript oracle notes:

```text
noImplicitReturnsWithProtectedBlocks1.ts: diagnostics=[]
noImplicitReturnsWithProtectedBlocks2.ts: TS2366 for main1 return type
noImplicitReturnsWithProtectedBlocks3.ts: TS2366 for main1 return type
```

The remaining TypeScript oracle diagnostics in files 2 and 3 are semantic
parity evidence, not a current compiler unsupported/name-resolution blocker in
this generated issue.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks --detail --no-dashboard-data
result: pass; executed=3 build_pass=3 unsupported=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks1.ts
result: pass; BuildPass
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks2.ts
result: pass; BuildPass
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsWithProtectedBlocks3.ts
result: pass; BuildPass
date: 2026-05-08
```

Remaining risks:

- Semantic parity for the TS2366 cases may need later tracking when the TSC
  suite runs with semantic comparison enabled, but it is not a current
  unsupported/name-resolution blocker.
