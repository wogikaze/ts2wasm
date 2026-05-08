---
id: 3539
title: "Implement Noimplicitanyfunctions"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5200]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the current blocker is the
existing top-level function overload grouping issue 5200.

## Problem

Fresh triage shows this fixture parses successfully through ambient functions,
ordinary functions, and the `f6` overload declarations. Validation then rejects
the second bodyless overload signature as a duplicate function:

```text
DuplicateFunction: duplicate function definition: `f6` at 254..262
```

TypeScript accepts the same overload group and reports no diagnostics.

Problem: this generated bucket is superseded by issue 5200, which owns
top-level function overload implementation grouping.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyFunctions.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyFunctions.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=DuplicateFunction:1 unsupported_features=duplicate-function:1
triage: DuplicateFunction duplicate function definition: `f6` at 254..262
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5200-validate-top-level-function-overload-implementations.md`. Do
not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold into existing issue 5200 for the same observable behavior
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
- [x] Existing issue 5200 now contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5200 acceptance names the exact fixture/reference path and diagnostic change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyFunctions.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyFunctions.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue fold.
- `cargo nextest run`; metadata-only issue fold.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into: `issues/open/5200-validate-top-level-function-overload-implementations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyFunctions.ts`

## Duplicate detection

- `issues/open/5200-validate-top-level-function-overload-implementations.md`
  is the exact owner for valid top-level overload signatures followed by one
  implementation currently being treated as duplicate functions.
- `issues/open/5226-w0-ast-node-span-requirement.md` is related
  but only covers ambient `declare function` overload declarations.
- `issues/open/5335-validate-nested-function-overload-implementations.md` is
  related but covers nested function overload declarations.
- Folded into issue 5200.

## Smart triage

### Smart triage: Triage duplicate function: noImplicitAnyFunctions

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/noImplicitAnyFunctions.ts`

Current compiler message:

```text
duplicate function definition: `f6` at 254..262
```

Source context:

```text
19 | function f6(x: string, y: number);
20 | function f6(x: string, y: string): any;
21 | function f6(x: string, y) {
22 |     return null;
23 | }
```

Compiler evidence:

```text
tokens: ok through declare functions, ordinary functions, and f6 overload group
ast: ok; two bodyless Function f6 signatures followed by one implemented Function f6
resolved: validate_ast fails with DuplicateFunction for the second bodyless f6 signature
visible symbols include f1, f2, f3, f4, f5, and the first f6 overload
```

TypeScript oracle:

```text
diagnostics=[]
AST contains the same three FunctionDeclaration nodes for f6
```

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
