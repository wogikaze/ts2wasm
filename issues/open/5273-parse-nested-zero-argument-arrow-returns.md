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
updated: 2026-05-07
---

## Summary

Parse arrow functions whose expression body is another arrow function,
especially nested zero-argument arrows inside object literal property
initializers or contextual callback positions such as
`doStuff: (callback) => () => { ... }` and `f(() => n => n)`.

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
- [ ] `collisionThisExpressionAndLocalVarInAccessors.ts` no longer reports the
  same nested zero-argument arrow parser failure inside class accessor bodies.
- [ ] `collisionThisExpressionAndLocalVarInProperty.ts` no longer reports the
  same nested zero-argument arrow parser failure inside class property
  initializers.
- [ ] `collisionThisExpressionAndParameter.ts` no longer reports the same
  nested zero-argument arrow parser failure inside a constructor object-literal
  property initializer.
- [ ] `contextualTypingFunctionReturningFunction.ts` no longer reports
  `expected Comma, got Some(Arrow)` at the inner arrow in `b: () => n => {}`.
- [ ] `contextualTypingFunctionReturningFunction2.ts` no longer reports
  `expected Comma, got Some(Arrow)` at the inner arrow in `f(() => n => n)`.
- [ ] `contextualTypingWithFixedTypeParameters1.ts` no longer reports
  `expected Comma, got Some(Arrow)` at the inner arrow in
  `f10('', () => a => a.foo, '')`.
- [ ] A focused parser test covers `(callback) => () => { return callback(this); }`.
- [ ] A focused parser test covers `() => n => n` as an expression-bodied
  arrow returning another arrow.
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
`issues/done/1324-implement-collisionThisExpressionAndLocalVarInAccessors.md`,
`issues/done/1329-implement-collisionThisExpressionAndLocalVarInProperty.md`,
`issues/done/1327-implement-collisionThisExpressionAndLocalVarInLambda.md`
and `issues/done/1328-implement-collisionThisExpressionAndLocalVarInMethod.md`,
which stop at the same `(callback) => () => { ... }` parser failure.

2026-05-07 additional evidence: `collisionThisExpressionAndLocalVarInAccessors.ts`
stops at span `141..143` on the second arrow in
`doStuff: (callback) => () => { ... }` inside a getter body. TypeScript accepts
the source and reports no diagnostics; its AST path has nested `ArrowFunction`
nodes under a `PropertyAssignment` inside `GetAccessor`.

2026-05-07 additional evidence: `collisionThisExpressionAndLocalVarInProperty.ts`
stops at span `111..113` on the second arrow in
`doStuff: (callback) => () => { ... }` inside a public class property
initializer. TypeScript accepts the source and reports no diagnostics; its AST
path has nested `ArrowFunction` nodes under a `PropertyAssignment` inside
`PropertyDeclaration`.

2026-05-07 additional evidence: `collisionThisExpressionAndParameter.ts`
stops at span `1073..1075` on the second arrow in
`doStuff: (callback) => () => { ... }` inside the `Foo1` constructor. Tokens
succeed through the nested arrow and callback body, while AST construction
fails with `UnsupportedSyntax: unsupported expression: Some(SpannedToken {
kind: RightParen, span: Span { start: 1071, end: 1072 } })`. TypeScript accepts
that nested arrow shape and later reports TS2683 implicit-`this` diagnostics
plus duplicate global `console` diagnostics.

Related but distinct:

- `issues/open/5240-parse-async-arrow-function-expressions.md` owns `async () =>`.
- `issues/open/5152-support-class-constructor-outer-callback-captures.md` owns
  class-constructor nested callback lowering after parsing succeeds.

2026-05-07 additional evidence: generated bucket
`issues/done/1522-implement-contextualTypingFunctionReturningFunction.md` is
superseded here. Both `contextualTypingFunctionReturningFunction.ts` and
`contextualTypingFunctionReturningFunction2.ts` tokenize successfully but fail
AST construction at the inner arrow in `() => n => ...`:

```text
contextualTypingFunctionReturningFunction.ts: expected Comma, got Some(Arrow) at 164..166
contextualTypingFunctionReturningFunction2.ts: expected Comma, got Some(Arrow) at 144..146
```

TypeScript accepts both sources with no diagnostics and its AST records nested
`ArrowFunction` nodes under the object literal property assignment
`b: () => n => {}` and the call argument `f(() => n => n)`.

2026-05-07 additional evidence: generated bucket
`issues/done/1532-implement-contextualTypingWithFixedTypeParameters.md` is
superseded here. `contextualTypingWithFixedTypeParameters1.ts` tokenizes
successfully but fails AST construction at the inner arrow in
`f10('', () => a => a.foo, '')`:

```text
expected Comma, got Some(Arrow) at 120..122
```

TypeScript parses the call argument as nested `ArrowFunction` nodes:
`() => a => a.foo` and `a => a.foo`. Its oracle then reports TS2339 for the
property access and TS2345 for the later fixed type-parameter argument
diagnostic after parsing succeeds.

2026-05-08 additional evidence: generated bucket
`issues/done/3559-implement-noImplicitThisFunctions.md` is superseded here.
`noImplicitThisFunctions.ts` fails at the inner arrow in
`let f5 = () => () => this;`; TypeScript parses it and later reports
implicit/global-`this` diagnostics.

## Completion evidence

Fill when implemented.
