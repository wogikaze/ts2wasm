---
id: 3520
title: "Implement Nocollisionthisexpressionandlocalvarinproperty"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5273]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`. Fresh triage shows the current blocker is the nested zero-argument arrow parser failure in `doStuff: (callback) => () => { ... }`.

## Problem

Reference test results originally showed 1 case failing in `noCollisionThisExpressionAndLocalVarInProperty` with parser-syntax diagnostics. Fresh triage on 2026-05-08 confirms the current first blocker is AST construction at the second arrow in a nested arrow expression:

```text
unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 109, end: 110 } }) at 111..113
```

Problem: this generated bucket is not a standalone implementation order. The current observable parser blocker is already tracked by issue 5273.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInProperty.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInProperty.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by issue 5273. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm issue 5273 covers the current nested zero-argument arrow parser blocker
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

- [x] Duplicate candidates below are confirmed as superseded by issue 5273
- [x] Closed issue contains an exact `reference-triage` command
- [x] Closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the nested zero-argument arrow parser diagnostic family

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInProperty.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInProperty.ts
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

- `reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInProperty.ts`

## Duplicate detection

- `issues/open/5273-parse-nested-zero-argument-arrow-returns.md` owns nested zero-argument arrow parsing in object literal property initializers and class property initializers.

## Smart triage

Fresh focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInProperty.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1 semantic_enabled=0
date: 2026-05-08
```

Fresh representative triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInProperty.ts
result: UnsupportedSyntax / parser-or-frontend-unsupported
diagnostic: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 109, end: 110 } }) at 111..113
source context: class property initializer contains object literal property `doStuff: (callback) => () => { var _this = 2; return callback(_this); }`
visible symbols: class class1
tokens: ok through `(callback) => () => { ... }`
ast: fails at the second arrow before resolved AST
TypeScript oracle: ok, diagnostics none; nested ArrowFunction nodes under PropertyAssignment inside PropertyDeclaration
date: 2026-05-08
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closing commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInProperty.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0; current blocker is nested zero-argument arrow parser failure
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInProperty.ts
result: pass; reproduced nested `() =>` parser failure; superseded by issue 5273
date: 2026-05-08
```

Remaining risks:

- Issue 5273 must still implement the parser fix; this bucket only removes duplicate generated tracking.
