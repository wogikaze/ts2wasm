---
id: 5385
title: "Parse arrow body assignment expressions"
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

Parse concise arrow bodies whose expression is an assignment, such as
`y => x = y`, especially when nested inside call arguments.

## Problem

Problem: `contextualTypingTwoInstancesOfSameTypeParameter.ts` currently fails
while parsing the nested arrow expression `y => x = y`.

The lexer recognizes the assignment token, but the parser expects a call
argument separator and reports:

```text
UnsupportedSyntax: expected Comma, got Some(Equal) at 113..114
```

TypeScript parses the same source as an arrow function whose body is a
`BinaryExpression` assignment.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts
```

Representative source:

```ts
function f6<T>(x: (a: T) => T) {
    return null;
}
f6(x => f6(y => x = y));
```

Compiler evidence:

```text
tokens: ok; includes `Ident x`, `Equal`, `Ident y` inside the nested arrow body
ast: fails with `expected Comma, got Some(Equal)` at the assignment operator
TypeScript AST: ExpressionStatement -> CallExpression -> ArrowFunction ->
  CallExpression -> ArrowFunction -> BinaryExpression `x = y`
TypeScript oracle: ok, diagnostics []
```

## Desired final state

The parser accepts assignment expressions as concise arrow bodies. The
representative reference no longer fails at `x = y` and advances to build-pass
or the next more specific unsupported boundary.

## Scope

In scope:

- [ ] Parse identifier-target assignment expressions in concise arrow bodies.
- [ ] Preserve nested arrow AST shape when the assignment body appears inside
  another call argument.
- [ ] Add focused parser coverage for `f(x => g(y => x = y));`.

Out of scope:

- Destructuring assignment arrow bodies; see
  `issues/done/5208-support-regexp-match-fallback-array-map-receiver.md`.
- Compound assignment operators beyond existing supported assignment forms.
- Type inference for contextual generic callback parameters after parsing.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser/compiler fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `contextualTypingTwoInstancesOfSameTypeParameter.ts` no longer reports
  `expected Comma, got Some(Equal)` at `x = y`.
- [ ] A focused fixture proves `f(x => g(y => x = y));` parses with the
  assignment inside the nested arrow body.
- [ ] Existing call-argument comma parsing remains unchanged for invalid
  argument lists.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(parser) or test(arrow) or test(assignment)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts --detail --no-dashboard-data
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
`issues/done/1531-implement-contextualTypingTwoInstancesOfSameTypeParameter.md`.

Related but distinct:

- `issues/done/5208-support-regexp-match-fallback-array-map-receiver.md` owns
  destructuring assignment bodies such as `() => [i] = [i + 1]`; this issue
  owns plain identifier assignment bodies.

## Completion evidence

Fill when implemented.
