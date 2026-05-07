---
id: 5382
title: "Parse typed arrow ternary branches"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse ternary conditional expressions whose branch is a parenthesized arrow
function with typed parameters.

## Problem

Problem: typed arrow functions used as ternary conditional branches are rejected during AST construction before contextual typing diagnostics can be reached.

`contextualTypingOfConditionalExpression2.ts` tokenizes successfully but fails
while parsing the typed alternate arrow parameter:

```text
UnsupportedSyntax: expected RightParen, got Some(Colon) at 209..210
```

TypeScript accepts the source and represents the initializer as a
`ConditionalExpression` with `ArrowFunction` branch nodes.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression2.ts
```

Representative source:

```ts
var x2: (a: A) => void =
    true ? (a: C) => a.foo : (b: number) => { };
```

Compiler evidence:

```text
tokens: ok
AST: fails on the colon inside `(b: number) => { }`
TypeScript oracle: TS2564 class fields plus TS2322 for the x2 assignment
```

## Desired final state

The parser produces an AST for a ternary branch that is a typed arrow function.
The representative reference should advance past AST construction to the next
semantic/type-system blocker.

## Scope

In scope:

- [ ] Parse typed parenthesized arrow branches in ternaries, such as
  `true ? (a: C) => a.foo : (b: number) => { }`.
- [ ] Preserve typed arrow branch parameters in the parser AST shape already
  used for typed arrow functions.
- [ ] Add focused parser coverage for a typed arrow alternate branch in a
  ternary conditional.

Out of scope:

- Untyped arrow branches in ternaries, tracked by
  `issues/open/5381-parse-arrow-functions-in-ternary-branches.md`.
- Lowering `Expr::Ternary`, tracked by
  `issues/open/5160-lower-plain-ternary-conditional-expressions.md`.
- Contextual function type assignability for branch unions.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- focused parser fixtures or CLI assertions

Do not touch:

- `crates/backend-wasm/`
- Ternary IR/backend lowering paths unless parsing already succeeds and focused
  evidence proves this issue's current blocker has moved.

## Acceptance criteria

- [ ] `contextualTypingOfConditionalExpression2.ts` no longer reports
  `expected RightParen, got Some(Colon)` at the typed alternate arrow branch.
- [ ] A focused parser test covers `true ? (a: C) => a.foo : (b: number) => { }`.
- [ ] Existing typed arrow-function expression and ternary parser tests still
  pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(arrow) or test(ternary) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression2.ts --detail --no-dashboard-data
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

Split from generated bucket
`issues/done/1525-implement-contextualTypingOfConditionalExpression.md`.

## Completion evidence

Fill when implemented.
