---
id: 3538
title: "Implement Noimplicitanyfunctionexpressionassignment"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5475]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence splits the current generic
function-expression parser gap to issue 5475.

## Problem

Fresh triage shows the representative stops at the anonymous generic function
expression:

```ts
var x: (a: any) => void = function <T>(x: T) {
```

The parser reports `UnsupportedSyntax: expected LeftParen, got Some(Less)` at
the `<` after `function`. TypeScript parses this as a `FunctionExpression` and
reports no diagnostics.

Problem: this generated bucket is superseded by issue 5475 for generic
anonymous and named function expression parsing.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyFunctionExpressionAssignment.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyFunctionExpressionAssignment.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
UnsupportedSyntax: expected LeftParen, got Some(Less) at 82..83
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5475-parse-generic-function-expressions.md`. Do not implement
directly from this bucket.

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
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyFunctionExpressionAssignment.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyFunctionExpressionAssignment.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue split.
- `cargo nextest run`; metadata-only issue split.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5475-parse-generic-function-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyFunctionExpressionAssignment.ts`

## Duplicate detection

- `issues/open/3425-implement-namedFunctionExpressionCall.md` is related to
  named function expressions after parsing; its child issue 5440 owns
  function-valued local call lowering, not generic function-expression parsing.
- `issues/open/5148-parse-generic-async-generator-declarations.md` covered
  generic async generator declarations, not generic function expressions.
- Split to `issues/open/5475-parse-generic-function-expressions.md`.

## Smart triage

### Smart triage: Triage parser syntax: noImplicitAnyFunctionExpressionAssignment

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/noImplicitAnyFunctionExpressionAssignment.ts`

Current compiler message:

```text
expected LeftParen, got Some(Less) at 82..83
```

Source context:

```text
4 | var x: (a: any) => void = function <T>(x: T) {
5 |     return null;
6 | };
8 | var x2: (a: any) => void = function f<T>(x: T) {
```

Compiler evidence:

```text
tokens: ok through `function <T>(x: T)` and `function f<T>(x: T)`
ast/resolved: fail before AST construction at the `<` after `function`
visible symbols before failure: binding x
```

TypeScript oracle:

```text
diagnostics=[]
AST path: VariableDeclaration -> FunctionExpression `function <T>(x: T) { ... }`
parameter x has type T
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
