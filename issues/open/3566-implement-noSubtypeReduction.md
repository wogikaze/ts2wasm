---
id: 3566
title: "Implement Nosubtypereduction"
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

Triage noSubtypeReduction across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this generated bucket is stale: the affected reference case
now builds successfully in ts2wasm. The parser handles exported interfaces,
exported function declarations, `for-of`, `in` checks, and the `useB(el.B)`
call in this file.

Problem: `noSubtypeReduction.ts` no longer has a current compiler blocker to
split from this generated bucket.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noSubtypeReduction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noSubtypeReduction.ts --detail
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
- [x] Completion evidence names the exact fixture and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noSubtypeReduction.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noSubtypeReduction.ts
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

- `reference/typescript/tests/cases/compiler/noSubtypeReduction.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-08:

- Diagnostic: `BuildPass` / `pass`
- Message: `ts2wasm build succeeded`
- Coverage: `executed=1`, `build_pass=1`, `unsupported=0`, `blocked=0`
- Tokens, AST, and resolved dumps succeed through `export interface IA`,
  `export interface IAB`, `export function F`, `const useB = ...`,
  `for (const el of x.arr)`, both `in` checks, and `useB(el.B)`.
- TypeScript oracle reports no diagnostics.

## Completion evidence

Closed as stale build-pass evidence.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noSubtypeReduction.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noSubtypeReduction.ts
result: pass; BuildPass, no current compiler blocker
date: 2026-05-08
```

Remaining risks:

- none
