---
id: 3514
title: "Implement Nocollisionthisexpressionandclassinglobal"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5192]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5192-support-first-class-class-constructor-values.md`. Fresh triage shows the current blocker is `issue-5011` for using the class `_this` as an expression value inside an arrow body, not the original generated name-resolution bucket.

## Problem

Reference test results originally showed 1 case failing in `noCollisionThisExpressionAndClassInGlobal` with name-resolution diagnostics. Fresh triage on 2026-05-08 shows parser and AST construction succeed, and name resolution reaches the explicit class-value unsupported boundary:

```text
issue-5011: class `_this` cannot be used as a value - class runtime is not yet supported at 52..57
```

Problem: this generated bucket is not a standalone implementation order. The current observable blocker is first-class class constructor value support, already tracked by issue 5192.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndClassInGlobal.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndClassInGlobal.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by issue 5192. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm issue 5192 covers the current `issue-5011` class constructor value blocker
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

- [x] Duplicate candidates below are confirmed as superseded by issue 5192
- [x] Closed issue contains an exact `reference-triage` command
- [x] Closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance owns the current class constructor value diagnostic family

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndClassInGlobal.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndClassInGlobal.ts
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

- `reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndClassInGlobal.ts`

## Duplicate detection

- `issues/open/5192-support-first-class-class-constructor-values.md` owns class constructor bindings used as expression values after resolution reaches `issue-5011`.

## Smart triage

Fresh focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndClassInGlobal.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1 semantic_enabled=0
date: 2026-05-08
```

Fresh representative triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndClassInGlobal.ts
result: UnsupportedSyntax / parser-or-frontend-unsupported
diagnostic: issue-5011: class `_this` cannot be used as a value - class runtime is not yet supported at 52..57
source: class _this {}; var f = () => _this;
visible symbols: class _this; binding f
tokens: ok
ast: ok; ClassDecl `_this` plus `var f = () => _this`
resolved: fails in resolve_names on class value `_this`
TypeScript oracle: ok, diagnostics none; binding f has type () => typeof _this
date: 2026-05-08
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closing commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndClassInGlobal.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0; current blocker is issue-5011 class value
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCollisionThisExpressionAndClassInGlobal.ts
result: pass; reproduced issue-5011 for class `_this` used as an arrow return value; superseded by issue 5192
date: 2026-05-08
```

Remaining risks:

- Issue 5192 currently scopes first-class constructor values; this bucket adds arrow-return evidence but does not by itself implement class runtime value semantics.
