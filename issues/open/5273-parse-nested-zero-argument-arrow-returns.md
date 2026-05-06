---
id: 5273
title: "Parse nested zero-argument arrow returns"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse arrow functions whose expression body is another zero-argument arrow
function, especially inside object literal property initializers such as
`doStuff: (callback) => () => { ... }`.

## Problem

`collisionThisExpressionAndLocalVarInConstructor.ts` tokenizes successfully but
AST construction fails at the second arrow in `(callback) => () => { ... }`.
TypeScript accepts the nested arrow expression and preserves lexical `this`
inside the inner arrow.

Current diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 132, end: 133 } }) at 134..136
```

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts
```

Representative source:

```ts
class class1 {
    constructor() {
        var x2 = {
            doStuff: (callback) => () => {
                var _this = 2;
                return callback(this);
            }
        }
    }
}
```

Compiler evidence:

```text
tokens: ok; includes (callback) => () => { ... }
AST: fails at RightParen before the second Arrow token
TypeScript oracle: ok, no diagnostics; nested ArrowFunction under PropertyAssignment
```

## Desired final state

The parser recognizes `() => ...` as an expression when it appears as the body
of another arrow function. The representative reference should advance past
the current AST construction failure to the next semantic or lowering blocker.

## Scope

In scope:

- [ ] Parse zero-argument arrow expressions in nested expression position.
- [ ] Preserve nested arrow AST shape under object literal property assignments.
- [ ] Add a focused parser test for `{ f: (callback) => () => callback(this) }`
  or the block-bodied equivalent.
- [ ] Re-run the representative reference triage and record the next
  diagnostic or pass state.

Out of scope:

- Full lexical `this` lowering inside nested arrow callbacks.
- Runtime support for invoking arbitrary object-literal callback factories.
- Async arrow parsing, tracked by `issues/open/5240-parse-async-arrow-function-expressions.md`.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- unrelated runtime callback lowering

## Acceptance criteria

- [ ] `collisionThisExpressionAndLocalVarInConstructor.ts` no longer reports
  `unsupported expression: Some(SpannedToken { kind: RightParen ... })` at the
  second arrow.
- [ ] A focused parser test covers `(callback) => () => { return callback(this); }`.
- [ ] Existing arrow function expression tests still pass.
- [ ] The representative reference triage records the next diagnostic or pass
  state after the parser fix.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(arrow) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInConstructor.ts --detail --no-dashboard-data
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
`issues/done/1325-implement-collisionThisExpressionAndLocalVarInConstructor.md`.
Also supersedes
`issues/done/1327-implement-collisionThisExpressionAndLocalVarInLambda.md`
and `issues/done/1328-implement-collisionThisExpressionAndLocalVarInMethod.md`,
which both stop at the same `(callback) => () => { ... }` parser failure.

Related but distinct:

- `issues/open/5240-parse-async-arrow-function-expressions.md` owns `async () =>`.
- `issues/open/5152-support-class-constructor-outer-callback-captures.md` owns
  class-constructor nested callback lowering after parsing succeeds.

## Completion evidence

Fill when implemented.
