---
id: 3562
title: "Implement Noinferunionexcesspropertycheck"
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

Triage noInferUnionExcessPropertyCheck across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this generated bucket is stale: the affected reference case
now builds successfully in ts2wasm. The TypeScript oracle still reports TS2353
excess-property diagnostics, but semantic parity is not enabled for this
coverage window and is not the generated name-resolution blocker.

Problem: `noInferUnionExcessPropertyCheck1.ts` no longer has a current compiler
blocker to split from this generated bucket.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noInferUnionExcessPropertyCheck1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noInferUnionExcessPropertyCheck1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale build-pass evidence
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] No child issue is needed for the stale build blocker
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the remaining TypeScript oracle diagnostics as semantic parity risk only

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noInferUnionExcessPropertyCheck1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noInferUnionExcessPropertyCheck1.ts
```

Not run:

- broad Rust gates; no source implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none for this generated build-blocker bucket

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noInferUnionExcessPropertyCheck1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-08:

- Diagnostic: `BuildPass` / `pass`
- Message: `ts2wasm build succeeded`
- Coverage: `executed=1`, `build_pass=1`, `unsupported=0`, `blocked=0`
- Tokens, AST, and resolved dumps succeed through the ambient `declare function`
  declarations and all six object-literal call expressions.
- TypeScript oracle reports TS2353 for the three calls with extra property
  `y`, so `NoInfer` union excess-property semantic parity remains unproven
  outside this build-blocker cleanup.

## Completion evidence

Closed as stale build-pass evidence.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noInferUnionExcessPropertyCheck1.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noInferUnionExcessPropertyCheck1.ts
result: pass; BuildPass, no current compiler blocker
date: 2026-05-08
```

Remaining risks:

- TypeScript oracle TS2353 excess-property diagnostics are semantic parity work,
  not a current build blocker in this bucket.
