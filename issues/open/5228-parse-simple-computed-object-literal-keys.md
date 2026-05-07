---
id: 5228
title: "Parse simple computed object literal keys"
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

Parse object literal computed property keys whose key expression is a simple
identifier, such as `{ [n]: 1 }`.

## Problem

`checkJsObjectLiteralIndexSignatures.ts` tokenizes successfully, but AST
construction rejects the closing bracket in the first computed object literal
key.

Problem: `{ [n]: 1 }` reports `UnsupportedSyntax: expected Dot, got Some(RightBracket)` at `190..191`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts
```

Source shape:

```ts
let n = Math.random();
const numericIndex = { [n]: 1 };
numericIndex[n].toFixed();
```

Compiler evidence:

```text
tokens: ok; LeftBracket Ident("n") RightBracket Colon Number(1)
ast: expected Dot, got Some(RightBracket) at 190..191
TypeScript oracle: no diagnostics; numericIndex has type { [x: number]: number }
```

## Desired final state

The parser accepts a simple identifier as an object literal computed property
key and preserves it for later resolver/lowering work.

## Scope

In scope:

- [ ] Parse `{ [n]: 1 }` object literal property assignments.
- [ ] Add one focused parser/compiler fixture for a simple computed key.

Out of scope:

- Binary computed keys, covered by issue 5209.
- Computed properties after object spread, covered by issue 5223.
- Index-signature type inference or `.toFixed()` runtime semantics.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `checkJsObjectLiteralIndexSignatures.ts` no longer reports `expected Dot, got Some(RightBracket)` for `{ [n]: 1 }`.
- [ ] `commaOperatorInConditionalExpression.ts` no longer reports `expected Dot, got Some(RightBracket)` for `{ [m]: i }` inside ternary object-literal branches.
- [ ] A focused fixture proves object literals accept `[identifier]` computed property keys.
- [ ] Existing computed-member access parsing still handles `numericIndex[n].toFixed()`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(object) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1136-implement-checkJsObjectLiteralIndexSignatures.md`.

2026-05-07 additional evidence:
`commaOperatorInConditionalExpression.ts` stops at the same parser boundary for
`{ [m]: i }` in `return true ? { [m]: i } : { [m]: i + 1 }`. Tokens show the
computed key as `LeftBracket Ident("m") RightBracket Colon`, then AST
construction fails with `UnsupportedSyntax: expected Dot, got
Some(RightBracket) at 97..98`. TypeScript accepts the source with no
diagnostics and its AST path is `ConditionalExpression -> ObjectLiteralExpression
-> PropertyAssignment -> ComputedPropertyName`.

## Completion evidence

Fill when implemented.
