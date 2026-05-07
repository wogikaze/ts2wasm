---
id: 5223
title: "Parse computed properties after object spread"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse object literal computed property assignments that appear after an object
spread property, including the reference shape `{ x: 1, ...o, [k]: 1 }`.

## Problem

`checkDestructuringShorthandAssigment2.ts` tokenizes successfully but AST
construction fails when an object literal contains a computed property after a
spread entry. The parser expects a dotted property access after `[k` and errors
at the closing bracket.

Current diagnostic:

```text
UnsupportedSyntax: expected Dot, got Some(RightBracket) at 134..135
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```ts
let o: any, k: any;
let { x } = { x: 1, ...o, [k]: 1 };
```

Compiler evidence:

```text
tokens: ok; includes DotDotDot, LeftBracket, Ident k, RightBracket, Colon
ast: fails with expected Dot at the computed property closing bracket
TypeScript oracle: parses ObjectLiteralExpression with PropertyAssignment [k]: 1 and reports TS2353
```

## Desired final state

The parser accepts computed property assignments after object spread entries in
object literals, so the representative path no longer stops with `expected Dot`.

## Scope

In scope:

- [ ] Parse object literal entries of the form `[expr]: value`.
- [ ] Preserve the existing object spread parse path for `...o`.
- [ ] Add a focused fixture for `{ x: 1, ...o, [k]: 1 }` in a destructuring initializer.

Out of scope:

- Full computed property runtime semantics.
- TypeScript excess-property diagnostics such as TS2353.
- Destructuring binding runtime semantics beyond preserving the existing parsed binding pattern.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `crates/cli/tests/`
- focused parser fixtures

Do not touch:

- resolver/lowering semantics unless the AST contract already requires a new node shape there
- unrelated object literal spread runtime behavior

## Acceptance criteria

- [ ] `checkDestructuringShorthandAssigment2.ts` no longer reports `expected Dot, got Some(RightBracket)`.
- [ ] A focused parser fixture covers `{ x: 1, ...o, [k]: 1 }`.
- [ ] Existing object spread fixtures still parse.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend object
cargo nextest run -p ts2wasm-cli -E 'test(object) or test(destructuring)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts --detail
```

Not run:

- none

## Notes

Split from generated bucket `issues/done/1129-implement-checkDestructuringShorthandAssigment-destructuring.md`.

## Completion evidence

Fill when implemented.
