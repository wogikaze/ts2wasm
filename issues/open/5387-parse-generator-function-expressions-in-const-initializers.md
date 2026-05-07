---
id: 5387
title: "Parse generator function expressions in const initializers"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse `function*` expressions when they initialize typed `const` bindings.

## Problem

Problem: `contextuallyTypeGeneratorReturnTypeFromUnion.ts` currently stops at
the `*` in a generator function expression assigned to a typed const:

```text
UnsupportedSyntax: expected LeftParen, got Some(Star) at 185..186
```

TypeScript parses the source with no diagnostics and records the initializer
as a `FunctionExpression` with an `AsteriskToken`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts
```

Representative source:

```ts
type Action = () => (Generator<string, string, string[]> | string)

const test1: Action = function* () {
    const next = yield ''
    return next[0]
}
```

Compiler evidence:

```text
tokens: ok; includes `Function`, `Star`, `(`, `)`, and body tokens
ast: fails with `expected LeftParen, got Some(Star)` at the `function*`
TypeScript AST: VariableDeclaration initializer is FunctionExpression
TypeScript oracle: ok, diagnostics []
```

## Desired final state

The parser accepts generator function expressions in typed const initializers.
The representative reference no longer fails at the `function*` parser
boundary and advances to build-pass or to the next source-spanned generator
runtime/lowering diagnostic.

## Scope

In scope:

- [ ] Parse `const x: T = function* () { ... }` as a generator function
  expression.
- [ ] Preserve enough AST shape or erasure behavior to advance past the raw
  `expected LeftParen, got Some(Star)` parser failure.
- [ ] Add focused parser coverage for a typed const initialized by
  `function* () { yield ""; }`.

Out of scope:

- Generator runtime semantics.
- Async generator expressions such as `async function* () {}`.
- Type inference for `Generator<Yield, Return, Next>`.
- Parameter initializer generator expressions; see
  `issues/open/5213-parse-generator-function-expressions-in-parameter-initializers.md`.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser/compiler fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `contextuallyTypeGeneratorReturnTypeFromUnion.ts` no longer reports
  `expected LeftParen, got Some(Star)` at `const test1: Action = function* ()`.
- [ ] A focused parser fixture proves `const g: T = function* () { yield ""; }`
  parses.
- [ ] Existing generator function declarations still parse.
- [ ] If generator lowering remains unsupported, the next diagnostic is
  source-spanned and specific rather than a raw parser token error.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(generator) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts --detail --no-dashboard-data
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
`issues/done/1537-implement-contextuallyTypeGeneratorReturnTypeFromUnion.md`.

Related but distinct:

- `issues/open/5213-parse-generator-function-expressions-in-parameter-initializers.md`
  owns `function*` expressions inside default parameter initializers.
- `issues/done/1514-implement-contextualTypeOnYield.md` covered the
  `contextualTypeOnYield*.ts` fixed paths; this reference still reports the raw
  parser boundary with the current shared binary.

## Completion evidence

Fill when implemented.
