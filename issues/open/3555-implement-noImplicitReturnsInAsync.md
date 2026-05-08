---
id: 3555
title: "Implement Noimplicitreturnsinasync"
type: spike
area: reference/triage
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as stale. Fresh coverage shows both `noImplicitReturnsInAsync` reference
files now build successfully, so this generated runtime-subset blocker no
longer needs a child issue.

## Problem

Older reference results showed 2 cases failing in directory
`noImplicitReturnsInAsync` with diagnostics `runtime-subset`. Fresh evidence on
2026-05-08 shows both files are `build_pass`:

```text
reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync1.ts: build_pass
reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync2.ts: build_pass
```

Problem: no current compiler blocker remains for this generated bucket.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync2.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=2 build_pass=2 unsupported=0 blocked=0
triage noImplicitReturnsInAsync1.ts: BuildPass
triage noImplicitReturnsInAsync2.ts: BuildPass
```

## Desired final state

This generated bucket is closed as stale. Do not implement directly from this
bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm no current unsupported/runtime blocker remains
- [x] Close the stale generated bucket without creating a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is closed stale
- [x] This done issue contains exact focused `reference-triage` commands
- [x] Evidence includes both affected paths, build-pass status, visible symbols, and parser/TypeScript AST evidence
- [x] No child issue was created because there is no current unsupported/runtime blocker

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync2.ts
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

- `reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync1.ts`
- `reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync2.ts`

## Duplicate detection

- none needed; both representative files are current build passes.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Build pass: noImplicitReturnsInAsync1

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync1.ts

### Smart triage: Build pass: noImplicitReturnsInAsync2

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync2.ts
```

Compiler evidence:

```text
noImplicitReturnsInAsync1.ts: tokens ok; AST/resolved contains async function test with default boolean parameter; build succeeds
noImplicitReturnsInAsync2.ts: tokens ok; AST/resolved contains async functions test3 through test7; build succeeds
```

TypeScript oracle notes:

```text
noImplicitReturnsInAsync1.ts: diagnostics=[]
noImplicitReturnsInAsync2.ts: TS2366 and TS2322 diagnostics for test6 are still reported by the oracle
```

The remaining TypeScript oracle diagnostics in `noImplicitReturnsInAsync2.ts`
are semantic parity evidence, not a current compiler unsupported/runtime
blocker in this generated issue.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync --detail --no-dashboard-data
result: pass; executed=2 build_pass=2 unsupported=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync1.ts
result: pass; BuildPass
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsInAsync2.ts
result: pass; BuildPass
date: 2026-05-08
```

Remaining risks:

- Semantic parity for `noImplicitReturnsInAsync2.ts` may need later tracking
  when the TSC suite runs with semantic comparison enabled, but it is not a
  current unsupported/runtime blocker.
