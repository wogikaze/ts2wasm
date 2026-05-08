---
id: 5381
title: "Parse untyped arrow ternary branches"
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

Parse ternary conditional expressions whose consequent or alternate branch is an
untyped parenthesized arrow function expression.

## Problem

Problem: untyped arrow functions used as ternary conditional branches are rejected during AST construction before contextual typing diagnostics can be reached.

`contextualTypingOfConditionalExpression.ts` tokenizes successfully but fails
while parsing the ternary alternate branch:

```text
contextualTypingOfConditionalExpression.ts: expected Semicolon, got Some(Arrow) at 106..108
```

TypeScript accepts the ternary arrow-function branch shape and then reports
only later class-field definite-assignment diagnostics.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression.ts
```

Representative source shapes:

```ts
var x: (a: number) => void =
    true ? (a) => a.toExponential() : (b) => b.toFixed();
```

Compiler evidence:

```text
tokens: ok
AST: fails on the arrow token for `(b) => b.toFixed()`
TypeScript AST: ConditionalExpression with ArrowFunction branch node
```

## Desired final state

The parser produces an AST for a ternary conditional expression whose alternate
branch is an untyped arrow function. The representative reference should
advance past AST construction to the next semantic/type-system blocker.

## Scope

In scope:

- [x] Parse untyped parenthesized arrow branches in ternaries, such as
  `true ? (a) => a.toExponential() : (b) => b.toFixed()`.
- [x] Preserve the branch expression as an arrow-function AST node under the
  ternary expression.
- [x] Add focused parser coverage for an untyped arrow alternate branch in a
  ternary conditional.

Out of scope:

- Lowering `Expr::Ternary`, tracked by
  `issues/open/5160-lower-plain-ternary-conditional-expressions.md`.
- Typed arrow parameters in ternary branches, tracked by
  `issues/open/5382-parse-typed-arrow-ternary-branches.md`.
- Contextual function type assignability for ternary branch unions.
- Strict property initialization diagnostics for class fields.

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

- [x] `contextualTypingOfConditionalExpression.ts` no longer reports
  `expected Semicolon, got Some(Arrow)` at the alternate arrow branch.
- [x] A focused parser test covers `true ? (a) => a.x : (b) => b.y`.
- [x] Existing arrow-function expression and ternary parser tests still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(arrow) or test(ternary) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket
`issues/open/1525-implement-contextualTypingOfConditionalExpression.md`.

Related but distinct:

- `issues/open/5160-lower-plain-ternary-conditional-expressions.md` owns
  lowering for ternaries that already parse as `Expr::Ternary`.
- `issues/open/5382-parse-typed-arrow-ternary-branches.md` owns the typed
  branch shape `true ? (a: C) => a.foo : (b: number) => { }`.

## Completion evidence

Fill when implemented.

## False-done audit

**truly-done** (5381)

- Implementation commits: verified via `git log --oneline --all --grep=5381`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
