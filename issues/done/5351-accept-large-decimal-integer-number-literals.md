---
id: 5351
title: "Accept large decimal integer number literals"
type: feature
area: frontend/lexer
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Accept valid JavaScript decimal integer number literals that exceed the current
`i32` lexer conversion range.

## Problem

`constEnumErrors.ts` reaches `9007199254740992` and fails before token output:

```text
UnsupportedSyntax: invalid number literal: number too large to fit in target type at 524..540
```

TypeScript accepts the literal as a `number` in a const enum member initializer
and reports later enum diagnostics.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumErrors.ts
```

Observed 2026-05-07:

```text
source:
37 | const enum NaNOrInfinity {
38 |     A = 9007199254740992,
39 |     B = A * A,
40 |     C = B * B,
```

## Desired final state

The lexer accepts large decimal integer number literals as valid numeric tokens
or reports a later explicit semantic/runtime unsupported diagnostic rather than
failing during lexing.

## Scope

In scope:

- [ ] Accept `9007199254740992` as a decimal numeric literal token.
- [ ] Preserve source spans for later diagnostics.
- [ ] Add focused lexer coverage for a decimal integer above `i32::MAX`.

Out of scope:

- Hex literals above `i32::MAX`; issue `5171`.
- Decimal exponent literals; issue `5216`.
- Runtime large-number representation; issue `300`.
- Const-enum parser and semantic diagnostics; issue `5184` and later enum issues.

## Affected paths

Expected:

- `crates/frontend/src/lexer_numbers.rs`
- `crates/frontend/src/lexer_tests.rs`

Do not touch:

- backend/runtime number representation
- enum lowering or inlining

## Acceptance criteria

- [ ] Lexer tests accept `9007199254740992` without `number too large to fit in target type`.
- [ ] `constEnumErrors.ts` no longer reports the current lexer failure at `9007199254740992`.
- [ ] Invalid decimal number forms still produce source-spanned diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend number
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumErrors.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Notes

Split from `issues/open/1447-implement-constEnumErrors.md`.

## Completion evidence

Fill only when moving to `done/`.
