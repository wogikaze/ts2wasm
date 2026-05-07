---
id: 5237
title: "Parse this-property computed object literal keys"
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

Parse object literal computed property keys whose key expression is a property
access rooted at `this`, such as `{ [this.a]: "" }`.

## Problem

`checkingObjectWithThisInNamePositionNoCrash.ts` tokenizes successfully, but AST
construction rejects the `this` token inside the computed object literal key.

Problem: `{ [this.a]: "" }` reports `UnsupportedSyntax: expected identifier,
got Some(This)` at the dot after `this`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts
```

Source context:

```ts
export const thing = {
    doit() {
        return {
            [this.a]: "",
        }
    }
}
```

Compiler evidence:

```text
tokens: ok; LeftBracket This Dot Ident("a") RightBracket Colon String("")
ast: UnsupportedSyntax expected identifier, got Some(This) at 155..156
TypeScript AST: ObjectLiteralExpression -> PropertyAssignment -> ComputedPropertyName -> PropertyAccessExpression `this.a`
TypeScript oracle: TS2339 Property 'a' does not exist on type '{ doit(): { [x: number]: string; }; }'
```

## Desired final state

The parser accepts `this.a` as a computed object literal property-name
expression and preserves enough AST shape for later resolution/lowering or
TypeScript-style diagnostics. The representative case no longer fails while
parsing the computed key.

## Scope

In scope:

- [ ] Parse `{ [this.a]: value }` object literal property assignments.
- [ ] Add one focused parser/compiler fixture for a `this` property-access computed key.

Out of scope:

- Binary computed keys, covered by issue 5209.
- Simple identifier computed keys, covered by issue 5228.
- Computed properties after object spread, covered by issue 5223.
- Runtime semantics for evaluating computed property names.
- TypeScript checker diagnostics such as TS2339 for missing `this.a`.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser fixture

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `checkingObjectWithThisInNamePositionNoCrash.ts` no longer reports `expected identifier, got Some(This)` for `[this.a]`.
- [ ] A focused parser fixture proves object literals accept `[this.a]` computed property keys.
- [ ] Existing computed-member access parsing still handles `this.a` as a property access expression.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(object) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts --detail --no-dashboard-data
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
`issues/done/1148-implement-checkingObjectWithThisInNamePositionNoCrash.md`.

Related but not exact:

- `issues/open/5209-parse-computed-object-literal-property-expressions.md`
- `issues/open/5228-parse-simple-computed-object-literal-keys.md`
- `issues/open/5223-parse-computed-properties-after-object-spread.md`

## Completion evidence

Fill when implemented.
