---
id: 5349
title: "Parse multiplicative compound assignment operators"
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

Accept identifier-target `*=`, `/=`, and `%=` compound assignment expressions.

## Problem

The lexer emits `StarEqual`, `SlashEqual`, and `PercentEqual`, but the parser
rejects `*=` as statement syntax:

```text
UnsupportedSyntax: expected Semicolon, got Some(StarEqual) at 92..94
```

This blocks both class-binding and const-binding assignment triage before the
compiler can compare TypeScript diagnostics.

## Current failure

Class binding:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/arithAssignTyping.ts
```

```text
source: f *= 1; // error
visible symbol: class f
TypeScript AST: ExpressionStatement -> BinaryExpression -> AsteriskEqualsToken
TypeScript oracle: TS2629 Cannot assign to 'f' because it is a class.
```

Const binding:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-access2.ts
```

```text
source: x *= 4;
visible symbol: const binding x initialized to 0
TypeScript AST: ExpressionStatement -> BinaryExpression -> AsteriskEqualsToken
TypeScript oracle: TS2588 Cannot assign to 'x' because it is a constant.
```

## Desired final state

The frontend represents `target *= expr`, `target /= expr`, and `target %= expr`
as assignment-expression syntax for identifier targets and preserves the target
span for later semantic diagnostics.

## Scope

In scope:

- [x] Parse identifier-target `*=`, `/=`, and `%=`.
- [x] Add focused parser/frontend regression coverage for all three operators.
- [x] Confirm existing identifier-target `+=` and `-=` behavior remains covered.

Out of scope:

- Bitwise compound assignment `^=`, `&=`, `|=`; issue `5178`.
- Exponentiation compound assignment `**=`; issue `5164`.
- Property-access compound assignments; issue `5311`.
- Shift compound assignments `<<=`, `>>=`, `>>>=`.
- Final class-binding or const-binding assignment diagnostics.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/compiler/src/dump.rs`
- `crates/ir/src/name_resolver.rs`

Do not touch:

- backend/runtime code unless triage advances past parsing and proves it is needed

## Acceptance criteria

- [x] `arithAssignTyping.ts` no longer reports `expected Semicolon, got Some(StarEqual)` at `f *= 1`.
- [x] `constDeclarations-access2.ts` no longer reports `expected Semicolon, got Some(StarEqual)` at `x *= 4`.
- [x] `/=` and `%=` use the same compound-assignment representation.
- [x] Focused frontend tests cover `*=`, `/=`, and `%=`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend compound
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/arithAssignTyping.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-access2.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Notes

Split from generated buckets:

- `issues/open/661-implement-arithAssignTyping.md`
- `issues/open/1442-implement-constDeclarations-parser-syntax.md`

## Completion evidence

Fill only when moving to `done/`.
