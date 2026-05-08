---
id: 3460
title: "Implement Narrowingofqualifiednames"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as stale: the current focused reference run builds successfully and
there is no compiler blocker to split from this generated bucket.

## Problem

Reference test results originally showed 1 case failing in directory
`narrowingOfQualifiedNames` with parser-syntax diagnostics.

Fresh coverage and smart triage on 2026-05-08 show the current compiler accepts
the file through build. This bucket no longer has a blocker to split.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingOfQualifiedNames.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingOfQualifiedNames.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingOfQualifiedNames.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingOfQualifiedNames.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current focused build passes

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingOfQualifiedNames.ts`

## Duplicate detection

- No matching blocker owner is needed because the current focused run is
  `build_pass`.
- Semantic parity for TypeScript narrowing diagnostics is not represented by
  this generated blocker bucket because semantic comparison was not enabled in
  the focused coverage run.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingOfQualifiedNames.ts
```

Result:

```text
Feature label: build-pass
Diagnostic: BuildPass / pass
Message: ts2wasm build succeeded
tokens: ok
ast: ok; functions, nested for-of loops, type-only aliases, and property accesses parse
resolved: ok; qualified property accesses resolve
TypeScript oracle: reports TS2532 object-possibly-undefined diagnostics later in the file
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingOfQualifiedNames.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingOfQualifiedNames.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingOfQualifiedNames.ts
result: pass; BuildPass, no current compiler blocker
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- Semantic parity is not proven by this closure because semantic comparison was
  not enabled; this closure only removes the stale compiler-blocker bucket.
