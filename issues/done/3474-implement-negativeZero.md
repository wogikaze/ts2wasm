---
id: 3474
title: "Implement Negativezero"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as stale: fresh focused evidence shows `negativeZero.ts` now builds
successfully.

## Problem

Reference test results previously showed 1 case failing in directory
`negativeZero` with diagnostics: parser-syntax.

Fresh triage on 2026-05-08 reports `BuildPass`; no child implementation issue
is needed for this generated build-blocker bucket.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/negativeZero.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/negativeZero.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/negativeZero.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/negativeZero.ts
```

Not run:

- `cargo fmt --all --check` (issue-only stale closure; no Rust changes)
- `cargo nextest run` (issue-only stale closure; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/negativeZero.ts`

## Duplicate detection

- none needed; fresh triage reports BuildPass

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/negativeZero.ts
```

Result:

```text
Feature label: build-pass
Diagnostic: BuildPass / pass
Message: ts2wasm build succeeded
tokens: ok; Var, Ident("x"), Equal, Minus, Number(0)
ast: ok; Let x = Unary(Negate, Number(0))
resolved: ok; Let("x", Unary { op: Negate, expr: Number(0) })
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/negativeZero.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=1
unsupported=0
blocked=0
unsupported_diagcodes=
unsupported_features=
semantic_enabled=0
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/negativeZero.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/negativeZero.ts
result: pass; BuildPass, no compiler build blocker remains
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- Semantic parity for JavaScript negative zero behavior is not proven because
  this bucket tracks the build blocker only and semantic coverage was not
  enabled in the focused window.
