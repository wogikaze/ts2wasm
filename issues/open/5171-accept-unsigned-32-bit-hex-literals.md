---
id: 5171
title: "Accept unsigned 32-bit hex literals"
type: feature
area: frontend/lexer
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Accept the first large hexadecimal mask literal that blocks `binaryArithmeticControlFlowGraphNotTooLarge.ts`.

## Problem

`binaryArithmeticControlFlowGraphNotTooLarge.ts` currently fails before token output because `0xefcdab89` is parsed through `i32::from_str_radix`, even though this unsigned 32-bit mask is valid JavaScript numeric literal syntax.

Problem: non-decimal number literals above `i32::MAX` are rejected during lexing, so parser and lowering triage cannot reach the actual binary arithmetic/control-flow content.

## Current failure

Representative reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: invalid number literal: number too large to fit in target type at 298..308
```

Source context:

```ts
d = ((a & 0xefcdab89) | (~a & 0x98badcfe)) + blocks[1] + 271733878;
```

Compiler evidence:

- Token dump fails before AST construction.
- The failing span covers `0xefcdab89`.
- `crates/frontend/src/lexer_numbers.rs` currently converts ordinary numeric literals with `i32::from_str_radix`.

TypeScript oracle evidence:

```text
TypeScript accepts the file with no diagnostics and treats the failing span as a numeric literal inside a binary expression.
```

## Desired final state

The lexer accepts unsigned 32-bit hexadecimal integer literals used as bit masks, preserving the existing integer-number representation boundary for follow-up semantic work.

## Scope

In scope:

- [ ] Accept hexadecimal literals in the unsigned 32-bit range when they do not fit signed `i32`.
- [ ] Preserve existing diagnostics for invalid digits and values outside the supported unsigned 32-bit mask range.
- [ ] Add a focused lexer/parser regression for `0xefcdab89`.

Out of scope:

- Full JavaScript double-precision number literal support.
- Decimal literals above `i32::MAX`.
- Bitwise operator lowering and shift semantics after the file advances past lexing.

## Affected paths

Expected:

- `crates/frontend/src/lexer_numbers.rs`
- `crates/frontend/src/lexer_tests.rs`
- parser test coverage if the token-level regression is not enough

Do not touch:

- backend lowering for bitwise operators
- control-flow graph limits

## Acceptance criteria

- [ ] Lexer tests accept `0xefcdab89` without `number too large to fit in target type`.
- [ ] Invalid large non-hex or beyond-`u32` literals still produce a source-spanned diagnostic.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts` no longer reports the `0xefcdab89` lexer failure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend hex_literal
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from generated bucket `1056` on 2026-05-06. This only removes the first lexer blocker. Later triage may expose ordinary bitwise operator, shift, compound assignment, or actual control-flow graph work.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
