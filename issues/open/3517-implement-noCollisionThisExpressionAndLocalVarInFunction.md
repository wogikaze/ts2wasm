---
id: 3517
title: "Implement Nocollisionthisexpressionandlocalvarinfunction"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5340]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5340-preserve-function-after-object-type-declaration.md`. Fresh triage shows the current blocker is the unterminated erased object type annotation before `function x()`, not a standalone local `_this` collision issue.

## Problem

Reference test results originally showed 1 case failing in `noCollisionThisExpressionAndLocalVarInFunction` with parser-syntax diagnostics. Fresh triage on 2026-05-08 confirms the current first blocker:

```text
UnsupportedTypeScriptSyntax: unterminated TypeScript type annotation at 163..164
```

Problem: this generated bucket is not a standalone implementation order. The parser must first erase `var console: { log(val: any); }` and preserve the following `function x()`, which is tracked by issue 5340.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInFunction.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInFunction.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by issue 5340. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm issue 5340 covers the current erased object type annotation blocker
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

- [x] Duplicate candidates below are confirmed as superseded by issue 5340
- [x] Closed issue contains an exact `reference-triage` command
- [x] Closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the erased object type annotation before function declaration boundary

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInFunction.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInFunction.ts
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

- `reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInFunction.ts`

## Duplicate detection

- `issues/open/5340-preserve-function-after-object-type-declaration.md` owns the current `var name: { ... }\nfunction next() {}` parser boundary.

## Smart triage

Fresh focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInFunction.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1 semantic_enabled=0
date: 2026-05-08
```

Fresh representative triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInFunction.ts
result: UnsupportedTypeScriptSyntax / unsupported-feature-boundary
diagnostic: unterminated TypeScript type annotation at 163..164
source: var console: { log(val: any); } followed by function x() { var _this = 5; x => { console.log(_this); }; }
visible symbols: binding console; function x; binding _this
tokens: ok through typed var, function declaration, local _this, and arrow expression
ast: fails before preserving the following function body as a runtime declaration
TypeScript oracle: TS2403 duplicate console diagnostic; AST includes the typed var declaration and FunctionDeclaration `x`
date: 2026-05-08
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closing commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInFunction.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0; current blocker is erased object type annotation before function declaration
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndLocalVarInFunction.ts
result: pass; reproduced unterminated TypeScript type annotation; superseded by issue 5340
date: 2026-05-08
```

Remaining risks:

- Issue 5340 must still implement the parser fix; this bucket only removes duplicate generated tracking.
