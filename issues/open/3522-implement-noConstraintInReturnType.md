---
id: 3522
title: "Implement Noconstraintinreturntype"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed stale generated parser-syntax bucket for `noConstraintInReturnType1.ts`. Fresh focused coverage now build-passes the reference path, so no implementation child is needed for this generated build blocker.

## Problem

Reference test results originally showed 1 case failing in `noConstraintInReturnType` with parser-syntax diagnostics. Fresh triage on 2026-05-08 shows tokens, AST, and resolved AST all succeed.

Problem: stale generated parser-syntax bucket; the current compiler has no build blocker on this path.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noConstraintInReturnType1.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noConstraintInReturnType1.ts --detail --no-dashboard-data
```

## Desired final state

This stale generated bucket is closed with focused evidence that the affected file now build-passes. Future semantic parity for the TypeScript TS2322 null-return diagnostic should be tracked separately from this parser-syntax bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm no current compiler build blocker remains for this bucket
- [x] Close the stale bucket without creating a duplicate child
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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

- [x] Duplicate candidates below are unnecessary because the bucket is stale
- [x] Closed issue contains an exact `reference-triage` command
- [x] Closed issue includes affected path, current result, visible symbols, parser AST evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and current build-pass result

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noConstraintInReturnType1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noConstraintInReturnType1.ts
```

Not run:

- `cargo fmt --all --check` / `cargo nextest run`: issue metadata-only closure; no Rust implementation changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noConstraintInReturnType1.ts`

## Duplicate detection

- Fresh focused coverage found no current compiler build blocker for this generated bucket.
- No child issue was created because the affected reference now build-passes.

## Smart triage

Fresh focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noConstraintInReturnType1.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0 blocked=0 semantic_enabled=0
date: 2026-05-08
```

Fresh representative triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noConstraintInReturnType1.ts
result: BuildPass / pass
visible symbols: class List
tokens: ok through generic class constraint, static generic method constraint, return type `List<T>`, and `return null`
ast: ok; ClassDecl List with static::empty returning Null
resolved: ok; ClassDecl List with static method empty and static member placeholder
TypeScript oracle: TS2322, Type 'null' is not assignable to type 'List<T>'
date: 2026-05-08
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closing commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noConstraintInReturnType1.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0 blocked=0 semantic_enabled=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noConstraintInReturnType1.ts
result: BuildPass / pass
date: 2026-05-08
```

Remaining risks:

- Semantic execution/checking is not enabled for this reference coverage path.
- This closure does not claim TypeScript TS2322 null-return diagnostic parity; it only removes the stale generated parser-syntax/build blocker.
