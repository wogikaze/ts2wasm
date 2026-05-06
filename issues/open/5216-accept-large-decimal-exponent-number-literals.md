---
id: 5216
title: "Accept large decimal exponent number literals"
type: feature
area: frontend/lexer
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Accept valid JavaScript decimal exponent numeric literals that exceed the
current small integer lexer conversion range, such as `12e+34`.

## Problem

Problem: `castExpressionParentheses.ts` fails before token output because
`12e+34` is parsed through a target integer conversion and rejected as too large
even though TypeScript accepts it as a numeric literal.

Current diagnostic:

```text
UnsupportedSyntax: invalid number literal: number too large to fit in target type
```

## Current failure

Use the validation command below to reproduce the representative failure.

Observed failure:

```text
castExpressionParentheses.ts: invalid number literal: number too large to fit in target type at 215..221
```

Source shape:

```text
(<any>12e+34);
(<any>0xff);
```

Compiler evidence:

```text
tokens: fails before token output
ast: same lexer failure
resolved: same lexer failure
```

TypeScript AST evidence:

```text
ExpressionStatement -> ParenthesizedExpression -> TypeAssertionExpression -> FirstLiteralToken "12e+34"
```

## Desired final state

The lexer accepts valid decimal exponent number literals even when their value
does not fit the existing small integer representation, so later parser,
erasure, or lowering stages can classify any unsupported runtime semantics.

## Scope

In scope:

- [ ] Accept `12e+34` as a valid numeric literal token or explicit frontend
  numeric representation.
- [ ] Preserve source-spanned diagnostics for malformed exponent literals.
- [ ] Keep runtime/value range diagnostics separate from lexer tokenization.

Out of scope:

- Full IEEE-754 runtime semantics.
- Unsigned 32-bit hexadecimal masks owned by issue 5171.
- Leading-decimal literals owned by issue 5191.
- Large integer runtime representation owned by issue 300.

## Affected paths

Expected:

- `crates/frontend/src/lexer_numbers.rs`
- `crates/frontend/src/lexer_tests.rs`
- parser tests if token-level coverage is not enough

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] Lexer tests accept `12e+34` without `number too large to fit in target
  type`.
- [ ] `castExpressionParentheses.ts` no longer stops before token output at
  `12e+34`.
- [ ] Existing malformed exponent literal diagnostics remain source-spanned.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend numeric
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castExpressionParentheses.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/castExpressionParentheses.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
