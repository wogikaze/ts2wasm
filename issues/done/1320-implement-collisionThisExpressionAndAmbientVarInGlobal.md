---
id: 1320
title: "Implement Collisionthisexpressionandambientvaringlobal"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: []
blocks: [5161]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed as superseded by `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`.

## Problem

Reference test results show 1 case failing in directory `collisionThisExpressionAndAmbientVarInGlobal` with name-resolution diagnostics. Fresh triage confirms the current blocker is `UnresolvedName` for `_this` in `_this = 10` after the declaration-only ambient `declare var _this: number` was erased from the runtime AST.

Problem: `collisionThisExpressionAndAmbientVarInGlobal.ts` is blocked by the same ambient value declaration name-resolution gap already tracked by issue 5161.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientVarInGlobal.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientVarInGlobal.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation should proceed through `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5161's ambient value declaration name-resolution scope
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing child issue contains exact `reference-triage` commands for the same ambient `declare var` unresolved-name family
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing child issue acceptance names the exact ambient value diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientVarInGlobal.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientVarInGlobal.ts
```

Not run:

- `cargo fmt --all --check` (not run; issue metadata only)
- `cargo nextest run` (not run; issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientVarInGlobal.ts`

## Duplicate detection

- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md` is the exact implementation-ready owner for this ambient `declare var` name-resolution gap.
- Generic name-resolution buckets are not matches; they share only the broad feature label.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: collisionThisExpressionAndAmbientVarInGlobal

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientVarInGlobal.ts`
```

Failure location:

```text
5 | _this = 10; // Error
    ^^^^^
error: [UnresolvedName] unresolved name: `_this` at 115..126
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
```

Compiler evidence:

```text
tokens: ok; includes `declare var _this: number`, top-level arrow `this`, and `_this = 10`
ast: ok; ambient var is erased, remaining runtime AST has `var f = () => this` and `Assign _this = 10`
resolved: fails with UnresolvedName for `_this`
```

TypeScript oracle evidence:

```text
diagnostics: TS7041 for top-level arrow `this`
hints include ambient binding `_this: number`
```

Resolution:

```text
The current compiler blocker is an ambient value declaration name-resolution
gap. Issue 5161 already owns declaration-only `declare var` / `declare let` /
`declare const` names that must be resolver-visible without emitting runtime
declarations.
```

## Completion evidence


Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientVarInGlobal.ts
result: pass; reproduced ambient declare-var UnresolvedName for `_this`; superseded by issue 5161
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientVarInGlobal.ts --detail --no-dashboard-data
result: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnresolvedName:1 unsupported_features=name-resolution:1
date: 2026-05-07
```

Remaining risks:

- TypeScript currently reports the preceding TS7041 global `this` diagnostic before any `_this` assignment concern; issue 5161 still owns the compiler's ambient value name-resolution blocker.
