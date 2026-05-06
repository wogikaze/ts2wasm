---
id: 5208
title: "Parse arrow body destructuring assignments"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: [5000]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse concise arrow bodies whose expression is a destructuring assignment, such
as `() => [i] = [i + 1]`.

## Problem

Problem: the parser treats `[i]` in an arrow body as a complete expression and
then rejects the following assignment operator.

Current diagnostic:

```text
UnsupportedSyntax: expected RightParen, got Some(Equal)
```

TypeScript accepts the representative file with no diagnostics.

## Current failure

Use the validation command below to reproduce the representative failure.

Observed failure:

```text
capturedLetConstInLoop12.ts: expected RightParen, got Some(Equal) at 129..130
```

Source shape:

```text
for (let i = 0; i < 4; i++) {
    (() => [i] = [i + 1])();
}
```

TypeScript AST evidence:

```text
ArrowFunction body:
- BinaryExpression `[i] = [i + 1]`
- operator `=`
```

## Desired final state

The parser accepts assignment expressions in concise arrow bodies when the left
side is an array or object binding/assignment pattern.

## Scope

In scope:

- [ ] Parse the representative arrow-body destructuring assignment expression.

Out of scope:

- General destructuring binding semantics after parsing.
- Other capturedLetConstInLoop parser-syntax subfamilies.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser/compiler fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `capturedLetConstInLoop12.ts` no longer reports `expected RightParen,
  got Some(Equal)` for `() => [i] = [i + 1]`.
- [ ] A focused parser fixture proves concise arrow bodies accept array-pattern
  assignment expressions.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop12.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop12.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
