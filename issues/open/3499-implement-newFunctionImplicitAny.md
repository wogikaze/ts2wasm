---
id: 3499
title: "Implement Newfunctionimplicitany"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage newFunctionImplicitAny across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed after refreshed evidence showed this generated semantic bucket is
stale. The representative now build-passes in ts2wasm, and the TypeScript
oracle reports no diagnostics.

## Problem

Reference test results show 1 cases fail in directory `newFunctionImplicitAny` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: newFunctionImplicitAny no longer has a current build failure or
TypeScript oracle diagnostic in the representative coverage window.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newFunctionImplicitAny.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newFunctionImplicitAny.ts --detail
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
- [x] At least one exact `reference-triage` command is recorded below
- [x] This issue includes failing path, diagnostic code, source context,
  visible symbols, parser evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newFunctionImplicitAny.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newFunctionImplicitAny.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/newFunctionImplicitAny.ts`

## Duplicate detection

- Fresh triage found no current compiler blocker.
- This is ordinary function declaration plus `new Test()`, not dynamic
  `new Function(...)`; dynamic constructor diagnostics are tracked by
  `issues/open/062b-dynamic-function-constructor-diagnostics.md`.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Build pass: newFunctionImplicitAny

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/newFunctionImplicitAny.ts
```

Focused coverage:

```text
suite=tsc
executed=1
build_pass=1
mismatch=0
runtime_error=0
fail=0
unsupported=0
blocked=0
semantic_enabled=0

reference/typescript/tests/cases/compiler/newFunctionImplicitAny.ts: build_pass
```

Source context:

```ts
//@noimplicitany: true
function Test() { }
var test = new Test();
```

Compiler evidence:

```text
tokens: ok through function declaration and `new Test()`
ast: ok; Function Test and Let test = New(Ident Test)
resolved: ok; Function Test and New { class_name: "Test", args: [] }
visible symbols: function Test and binding test
```

TypeScript oracle evidence:

```text
diagnostics: none
hint: Test has type void
hint: test has type any
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newFunctionImplicitAny.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0 semantic_enabled=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newFunctionImplicitAny.ts
result: pass; BuildPass / pass; TypeScript oracle reports no diagnostics
date: 2026-05-08
```

Remaining risks:

- none for the current build blocker; semantic execution remains disabled for
  this coverage command, but the TypeScript oracle has no diagnostics.
