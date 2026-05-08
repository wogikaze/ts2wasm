---
id: 3443
title: "Implement Narrowbyparenthesizedswitchexpression"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed as stale: fresh focused coverage now reports build pass.

## Problem

Reference test results previously showed 1 case failing in directory
`narrowByParenthesizedSwitchExpression` with diagnostics: parser-syntax. Fresh
coverage now reports `build_pass=1`, so there is no current compiler blocker to
split.

Problem: narrowByParenthesizedSwitchExpression had 1 generated reference
failure and needed smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByParenthesizedSwitchExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByParenthesizedSwitchExpression.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale build-pass bucket
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
- [x] Fresh evidence contains an exact `reference-triage` command
- [x] Evidence includes build-pass parser/resolved output
- [x] No child issue needed because the representative path builds

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByParenthesizedSwitchExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByParenthesizedSwitchExpression.ts
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

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowByParenthesizedSwitchExpression.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByParenthesizedSwitchExpression.ts --detail --no-dashboard-data

result:
executed=1
build_pass=1
unsupported=0
semantic_enabled=0
```

Fresh triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByParenthesizedSwitchExpression.ts

result:
BuildPass: ts2wasm build succeeded
```

Compiler evidence:

```text
tokens: ok
ast: ok; switch expression is `v.type` despite source spelling `((v.type))`
resolved: ok; switch cases access v.bar and v.foo
TypeScript oracle: ok, diagnostics=[]
```

## Completion evidence

Closed as stale build-pass bucket; no child issue created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByParenthesizedSwitchExpression.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByParenthesizedSwitchExpression.ts
result: pass; BuildPass
date: 2026-05-08
```

Remaining risks:

- none
