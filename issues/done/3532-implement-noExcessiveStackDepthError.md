---
id: 3532
title: "Implement Noexcessivestackdeptherror"
type: spike
area: frontend/resolver
class: superseded
priority: P2
depends_on: [5205]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the old duplicate-local
blocker is stale; the remaining TS2403 false build-pass gap is owned by issue
5205.

## Problem

Older reference results reported a duplicate-local blocker for
`noExcessiveStackDepthError.ts`. Fresh triage on 2026-05-08 now returns
`BuildPass`: the recursive interface/type declarations are erased, AST and
resolved IR are empty, and the compiler no longer reports duplicate-local.

TypeScript still reports TS2403 at the second repeated `var x` declaration,
where `FindConditions<Entity>` conflicts with the earlier
`FindConditions<any>`. That semantic diagnostic gap is covered by
`issues/open/5205-report-incompatible-var-redeclaration-type-diagnostics.md`.

Problem: the generated duplicate-local bucket is stale; the remaining
incompatible `var` redeclaration type diagnostic is owned by issue 5205.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noExcessiveStackDepthError.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noExcessiveStackDepthError.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1, build_pass=1, unsupported=0, blocked=0, semantic_enabled=0
triage: BuildPass / pass
```

Compiler evidence:

```text
tokens: ok through interface FindOperator<T>, recursive mapped type FindConditions<T>, function foo, and both var x declarations
ast: ok, empty after erasing type-only declarations and repeated bodyless vars
resolved: ok, empty
visible symbols: two bindings named x at lines 16 and 17
```

TypeScript oracle:

```text
TS2403: Subsequent variable declarations must have the same type. Variable 'x' must be of type 'FindConditions<any>', but here has type 'FindConditions<Entity>'. at line 17, character 9
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5205-report-incompatible-var-redeclaration-type-diagnostics.md`.
Do not implement directly from this bucket.

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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Owner issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Owner issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Owner issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noExcessiveStackDepthError.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noExcessiveStackDepthError.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue lifecycle change.
- `cargo nextest run`; metadata-only issue lifecycle change.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing: `issues/open/5205-report-incompatible-var-redeclaration-type-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noExcessiveStackDepthError.ts`

## Duplicate detection

- `issues/open/5205-report-incompatible-var-redeclaration-type-diagnostics.md`
  is the matching owner for the current TS2403 incompatible same-scope `var`
  redeclaration false build-pass gap.
- `issues/open/5162-allow-compatible-var-redeclarations.md` is related but
  covers accepting compatible `var` redeclarations, not this later TS2403
  diagnostic.

## Smart triage

### Smart triage: Build pass: noExcessiveStackDepthError

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/noExcessiveStackDepthError.ts`

TypeScript oracle diagnostic:

```text
TS2403: Subsequent variable declarations must have the same type. Variable 'x' must be of type 'FindConditions<any>', but here has type 'FindConditions<Entity>'.
```

Folded into issue 5205 because the current actionable gap is TS2403-style
incompatible `var` redeclaration diagnostics.

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
