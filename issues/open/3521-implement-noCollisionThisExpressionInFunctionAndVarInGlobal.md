---
id: 3521
title: "Implement Nocollisionthisexpressioninfunctionandvaringlobal"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5179]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5179-report-implicit-this-before-closure-runtime-guard.md`. Fresh triage shows the current first diagnostic is unresolved `this` in an arrow expression inside `function x()`, while TypeScript reports TS2683 implicit `this` for the same source.

## Problem

Reference test results originally showed 1 case failing in `noCollisionThisExpressionInFunctionAndVarInGlobal` with arrow-function diagnostics. Fresh triage on 2026-05-08 reaches name resolution/lowering and reports:

```text
UnresolvedName: unresolved name: `this`
```

TypeScript reports TS2683 for the same `this` expression. Problem: this generated bucket is not a standalone implementation order; the current first diagnostic belongs to the implicit-this diagnostic owner, issue 5179.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionInFunctionAndVarInGlobal.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionInFunctionAndVarInGlobal.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by issue 5179. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm issue 5179 covers the current implicit-this diagnostic gap
- [x] Supersede this generated bucket without creating a duplicate child
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

- [x] Duplicate candidates below are confirmed as superseded by issue 5179
- [x] Closed issue contains an exact `reference-triage` command
- [x] Closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the implicit-this diagnostic behavior

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionInFunctionAndVarInGlobal.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionInFunctionAndVarInGlobal.ts
```

Not run:

- `cargo fmt --all --check` / `cargo nextest run`: issue metadata-only supersession; no Rust implementation changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noCollisionThisExpressionInFunctionAndVarInGlobal.ts`

## Duplicate detection

- `issues/open/5179-report-implicit-this-before-closure-runtime-guard.md` owns the current implicit-this diagnostic gap.
- `issues/open/5339-preserve-var-after-object-type-declaration.md` is related because the AST still folds the following `_this` initializer into the erased `console` declaration shape, but the current first reported diagnostic is `this`.

## Smart triage

Fresh focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionInFunctionAndVarInGlobal.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0 unsupported_diagcodes=UnresolvedName:1 unsupported_features=name-resolution:1 semantic_enabled=0
date: 2026-05-08
```

Fresh representative triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionInFunctionAndVarInGlobal.ts
result: UnresolvedName / resolver-symbol
diagnostic: unresolved name: `this`
source context: function x() contains `x => { console.log(this); };`
visible symbols: binding console; binding _this; function x
tokens: ok through typed `var console`, following `var _this`, function x, arrow, and `this`
ast: ok but still folds the following `_this` initializer into `Let console = Number(5)`, matching the related issue 5339 parser boundary
resolved/lowered: fails on the arrow-body `this`
TypeScript oracle: TS2403 duplicate console plus TS2683 implicit this at the `this` token; hints include `_this: number` and function `x: void`
date: 2026-05-08
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closing commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionInFunctionAndVarInGlobal.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0; current blocker is unresolved `this`
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionInFunctionAndVarInGlobal.ts
result: pass; reproduced unresolved `this`; superseded by issue 5179
date: 2026-05-08
```

Remaining risks:

- Issue 5179 must still implement the implicit-this diagnostic; issue 5339 remains related parser cleanup for preserving the following var binding after an erased object type declaration.
