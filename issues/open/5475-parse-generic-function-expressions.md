---
id: 5475
title: "Parse generic function expressions"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Parse TypeScript generic function expressions, both anonymous and named:
`function <T>(x: T) {}` and `function f<T>(x: T) {}`.

This is the current blocker from
`noImplicitAnyFunctionExpressionAssignment.ts`.

## Problem

The parser accepts the function-typed variable annotation, then fails when a
function expression is followed by type parameters:

```ts
var x: (a: any) => void = function <T>(x: T) {
    return null;
};
```

Current compiler diagnostic:

```text
UnsupportedSyntax: expected LeftParen, got Some(Less) at 82..83
```

Problem: function-expression parsing expects `(` immediately after `function`
or the optional function name, so it treats generic type parameters as a syntax
error instead of erasing them and continuing to the parameter list.

## Current failure

Fresh focused coverage for
`reference/typescript/tests/cases/compiler/noImplicitAnyFunctionExpressionAssignment.ts`
shows:

```text
executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
```

Fresh triage shows:

```text
line 4: var x: (a: any) => void = function <T>(x: T) {
diagnostic: UnsupportedSyntax expected LeftParen, got Some(Less) at 82..83
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

## Desired final state

Generic type parameter lists on anonymous and named function expressions are
parsed and erased, and the representative fixture advances past the current
`expected LeftParen` parser boundary.

## Scope

In scope:

- [ ] Parse optional `<T, ...>` type parameter lists on anonymous and named function expressions before the parameter list.

Out of scope:

- Full generic type checking or type argument inference.
- Function declarations; those are already handled by separate parser paths.
- Function-valued local call lowering, tracked by `issues/open/5440-support-initialized-function-expression-local-calls.md`.

## Affected paths

Expected:

- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/ast.rs`

Do not touch:

- backend-wasm lowering
- resolver function-valued local call semantics
- reference dashboard artifacts

## Acceptance criteria

- [ ] A focused parser regression accepts `function <T>(x: T) { return x; }`.
- [ ] A focused parser regression accepts `function f<T>(x: T) { return x; }`.
- [ ] `noImplicitAnyFunctionExpressionAssignment.ts` no longer reports `expected LeftParen, got Some(Less)`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend generic_function_expression
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyFunctionExpressionAssignment.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyFunctionExpressionAssignment.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from
`issues/done/3538-implement-noImplicitAnyFunctionExpressionAssignment.md`.

Related but not duplicates:

- `issues/done/3425-implement-namedFunctionExpressionCall.md` covers behavior
  after named function expressions parse; its child issue 5440 owns
  function-valued local call lowering.
- `issues/done/5148-parse-generic-async-generator-declarations.md` covered
  generic async generator declarations, not generic function expressions.

## Completion evidence

Fill when implemented.
