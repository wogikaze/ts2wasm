---
id: 5170
title: "Support bitwise OR binary lowering"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement the first ordinary JavaScript `|` binary operator lowering slice using `binaryArithmatic1.ts` as the representative case, with the other `binaryArithmatic` null/undefined cases kept as confirmation coverage.

## Problem

The parser accepts all four `binaryArithmatic` reference cases as `Binary { op: BitwiseOr }`, but lowering currently rejects the operator with `UnsupportedSyntax: binary operator BitwiseOr not yet supported`.

Problem: ordinary number/null/undefined bitwise OR expressions parse successfully but cannot be lowered.

## Current failure

Representative reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmatic1.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: binary operator BitwiseOr not yet supported
```

Representative source:

```ts
var v = 4 | null;
var v = 4 | undefined;
var v = undefined | undefined;
var v = null | null;
```

Compiler evidence:

- Tokens are successful for `|`, `null`, and `undefined`.
- AST construction succeeds with `Binary { op: BitwiseOr }`.
- Lowering fails in the ordinary binary operator path because `lower_binary_op` rejects `BinaryOp::BitwiseOr`.
- BigInt bitwise OR has a separate runtime call path and is not the blocker for these four files.

TypeScript oracle evidence:

```text
TypeScript accepts these ES2015 compiler cases; JavaScript bitwise OR applies ToInt32-style coercion to null and undefined operands.
```

## Desired final state

The lowered IR accepts ordinary `BinaryOp::BitwiseOr` for the representative primitive/null expression and routes it through the same backend/runtime layer used for ordinary numeric binary operators.

## Scope

In scope:

- [ ] Add the minimal lowered ordinary `BitwiseOr` path for `4 | null`.
- [ ] Preserve the existing BigInt-specific bitwise runtime call path.
- [ ] Add focused regression coverage that includes the three sibling null/undefined forms: `4 | undefined`, `undefined | undefined`, and `null | null`.

Out of scope:

- `|=` compound assignment.
- Bitwise AND, XOR, shifts, and unsigned right shift unless they are required by shared representation changes.
- TypeScript union-type parser erasure for `as A | B`; issue `5153` owns that parser slice.
- BigInt bitwise OR runtime semantics; existing BigInt-specific issues and code paths own that behavior.

## Affected paths

Expected:

- `crates/ir/src/lowered/types.rs`
- `crates/ir/src/lowered/program.rs`
- `crates/backend-wasm/src/expr_emit.rs`
- focused tests/fixtures for primitive bitwise OR

Do not touch:

- TypeScript assertion parsing for `as A | B`
- BigInt runtime helper implementation unless a focused test proves this ordinary operator slice needs a shared hook

## Acceptance criteria

- [ ] A focused compiler/lowering test covers `var v = 4 | null;`.
- [ ] The focused test output no longer reports `binary operator BitwiseOr not yet supported`.
- [ ] The ordinary `BitwiseOr` lowering path leaves the existing BigInt-specific bitwise runtime call path intact.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmatic1.ts` no longer reports `binary operator BitwiseOr not yet supported`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir bitwise_or
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmatic1.ts
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

Split from generated bucket `1055` on 2026-05-06. The bucket name is misspelled as `binaryArithmatic` in upstream reference paths; keep that spelling in commands and paths.

Sibling confirmation paths after the representative case advances:

- `reference/typescript/tests/cases/compiler/binaryArithmatic2.ts`
- `reference/typescript/tests/cases/compiler/binaryArithmatic3.ts`
- `reference/typescript/tests/cases/compiler/binaryArithmatic4.ts`

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
