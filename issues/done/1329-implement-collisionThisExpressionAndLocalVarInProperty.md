---
id: 1329
title: "Implement Collisionthisexpressionandlocalvarinproperty"
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

Closed as superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`.

## Problem

Reference test results show 1 case failing in directory `collisionThisExpressionAndLocalVarInProperty` with parser-syntax diagnostics. Fresh triage confirms tokens succeed, but AST construction stops at the nested zero-argument arrow expression `(callback) => () => { ... }` inside an initialized class property's object literal.

Problem: `collisionThisExpressionAndLocalVarInProperty.ts` reports `unsupported expression: Some(SpannedToken { kind: RightParen ... })` at the second arrow in `doStuff: (callback) => () => { ... }`, which is already tracked by issue 5273.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInProperty.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInProperty.ts --detail
```

## Desired final state

This generated bucket is superseded by implementation-ready issue 5273, which owns nested zero-argument arrow returns. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm issue 5273 already covers the current blocker
- [x] Close this generated bucket as superseded
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

- [x] Duplicate candidates below are confirmed and this issue is superseded by 5273
- [x] Issue 5273 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This issue records failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5273 acceptance names the exact parser diagnostic change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInProperty.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInProperty.ts
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

- [x] none; superseded by existing issue 5273

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInProperty.ts`

## Duplicate detection

- `issues/open/5273-parse-nested-zero-argument-arrow-returns.md` is the exact owner for `(callback) => () => { ... }` parser failures.
- Class property parsing and lexical `this` behavior after the nested arrow parses remain unproven until issue 5273 advances past the current parser failure.

## Smart triage

Generated 2026-05-07.

Fresh commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInProperty.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInProperty.ts
```

Observed result:

```text
coverage: executed=1 build_pass=0 unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 109, end: 110 } }) at 111..113
Source: doStuff: (callback) => () => {
Visible symbols before failure: class class1
tokens: ok; includes public prop1 class field, object literal property, `(callback) => () => { ... }`, local `_this`, and callback(this)
AST: fails at RightParen before the second Arrow token
TypeScript oracle: ok, no diagnostics; nested ArrowFunction under PropertyAssignment inside PropertyDeclaration
Superseded by: 5273
```

## Completion evidence


Commits:

- Superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`; see local commit for this issue cleanup.

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInProperty.ts
result: pass; reproduced nested zero-argument arrow parser failure and confirmed issue 5273 owns it
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInProperty.ts --detail --no-dashboard-data
result: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

Remaining risks:

- Class property lowering, accessor-local `_this` collision, and nested lexical `this` behavior remain unproven until issue 5273 advances past the parser failure.
