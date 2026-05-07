---
id: 1314
title: "Implement Collisionsuperandlocalvarinproperty"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed as stale. Fresh triage and focused coverage show `reference/typescript/tests/cases/compiler/collisionSuperAndLocalVarInProperty.ts` now build-passes, so there is no current property/local `var _super` compiler blocker to split into a child issue.

## Problem

The generated issue snapshot said this reference case failed with `parser-syntax`, but 2026-05-07 re-triage with the current compiler shows a build pass.

Problem: resolved for build coverage. The stale generated bucket no longer represents an actionable compiler blocker.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionSuperAndLocalVarInProperty.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndLocalVarInProperty.ts --detail
```

## Desired final state

This generated bucket is closed because the representative reference path now builds successfully and TypeScript reports no diagnostics.

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
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndLocalVarInProperty.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionSuperAndLocalVarInProperty.ts
```

Not run:

- `cargo fmt --all --check` (not run; metadata-only issue closure)
- `cargo nextest run` (not run; metadata-only issue closure)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionSuperAndLocalVarInProperty.ts`

## Duplicate detection

- Fresh triage found only this issue by same path/title and no open implementation duplicate.

## Smart triage

2026-05-07 fresh result:

```text
### Smart triage: Build pass: collisionSuperAndLocalVarInProperty

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/collisionSuperAndLocalVarInProperty.ts`
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

reference/typescript/tests/cases/compiler/collisionSuperAndLocalVarInProperty.ts: build_pass
```

Source context:

```ts
// @target: es2015
var _super = 10; // No Error
class Foo {
   public prop1 = {
        doStuff: () => {
            var _super = 10; // No error
        }
```

Visible symbols include top-level `_super`, property-initializer arrow-local `_super`, subclass `b extends Foo`, and subclass property-initializer arrow-local `_super`.

Compiler evidence:

- tokens: ok
- ast: ok; class declarations parsed
- resolved: ok; subclass `extends Foo` resolved

TypeScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "typescriptVersion": "6.0.3"
  }
}
```

## Completion evidence


Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionSuperAndLocalVarInProperty.ts
result: pass; Build pass: collisionSuperAndLocalVarInProperty
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndLocalVarInProperty.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0 blocked=0 semantic_enabled=0
date: 2026-05-07
```

Remaining risks:

- semantic execution is not enabled for this case; this closure removes the generated build blocker only, not broader class-field semantic parity work.
