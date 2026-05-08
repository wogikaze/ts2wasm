---
id: 3533
title: "Implement Noimplicitanyandprivatememberswithouttypeannotations"
type: spike
area: backend-wasm
class: superseded
priority: P1
depends_on: [5472]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the old name-resolution
classification is stale; the current direct build blocker is a backend WAT
global declaration bug split to issue 5472.

## Problem

Older reference results reported a name-resolution blocker for
`noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts`. Fresh triage now
tokenizes, parses, resolves, and emits WAT for the declaration-only ambient
class plus `var x = new Something()`.

The current direct `ts2wasm build` blocker is `BackendIo`: `wat2wasm` rejects
WAT that sets `$current_module_id` without declaring that global. This backend
scope is split to
`issues/open/5472-declare-current-module-id-for-virtual-file-inits.md`.

Problem: generated name-resolution bucket is stale; current blocker is
undefined `$current_module_id` in backend WAT for virtual file init.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts --detail --no-dashboard-data
```

Direct build reproduction:

```sh
/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm build reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts -o /tmp/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.wasm
```

Observed 2026-05-08:

```text
coverage: executed=1, build_pass=1, unsupported=0, blocked=0, semantic_enabled=0
triage: BackendIo / backend-io
direct build: undefined global variable "$current_module_id"
```

Direct build stderr:

```text
/tmp/ts2wasm-2-0.wat:527:17: error: undefined global variable "$current_module_id"
    (global.set $current_module_id (i32.const 1))
                ^^^^^^^^^^^^^^^^^^
```

Compiler evidence:

```text
tokens: ok through declare class Something and var x = new Something()
ast: ok; runtime AST contains Let x = New Something()
resolved: ok; resolved IR contains Let("x", New { class_name: "Something", args: [] })
wat: emitted, then wat2wasm fails on missing $current_module_id global
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5472-declare-current-module-id-for-virtual-file-inits.md`. Do not
implement directly from this bucket.

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
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts
/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm build reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts -o /tmp/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.wasm
```

Not run:

- `cargo fmt --all --check`; metadata-only issue split.
- `cargo nextest run`; metadata-only issue split.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5472-declare-current-module-id-for-virtual-file-inits.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts`

## Duplicate detection

- No exact existing owner found for WAT that references `$current_module_id`
  without declaring it.
- `issues/open/5155-fix-exception-pending-runtime-link-for-top-level-statements.md`
  is related backend-io infrastructure but covers missing `$exception_pending`,
  not `$current_module_id`.

## Smart triage

### Smart triage: Triage backend io: noImplicitAnyAndPrivateMembersWithoutTypeAnnotations

- Issue class: `triage-needed`
- Feature label: `backend-io`
- Diagnostic: `BackendIo` / `backend-io`
- Path: `reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts`

Direct build evidence:

```text
error: [BackendIo] wat2wasm failed
/tmp/ts2wasm-2-0.wat:527:17: error: undefined global variable "$current_module_id"
    (global.set $current_module_id (i32.const 1))
                ^^^^^^^^^^^^^^^^^^
```

Split to issue 5472 because the current actionable blocker is backend WAT
declaration/linkage, not frontend name resolution.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
