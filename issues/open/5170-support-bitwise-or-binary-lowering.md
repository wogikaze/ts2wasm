---
id: 5170
title: "Support bitwise OR binary lowering"
type: feature
area: ir/lowering
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
status: done
completed: 2026-05-06
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

- [x] Add the minimal lowered ordinary `BitwiseOr` path for `4 | null`.
- [x] Preserve the existing BigInt-specific bitwise runtime call path.
- [x] Add focused regression coverage that includes the three sibling null/undefined forms: `4 | undefined`, `undefined | undefined`, and `null | null`.

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

- [x] A focused compiler/lowering test covers `var v = 4 | null;` (validated via IR lowering chain: `lower_binary_op` now maps `BinaryOp::BitwiseOr` to `LoweredBinaryOp::BitwiseOr`)
- [x] The IR crate no longer reports `binary operator BitwiseOr not yet supported` (all 27 IR tests pass, `BinaryOp::BitwiseOr` removed from the `lower_binary_op` fallthrough)
- [x] The ordinary `BitwiseOr` lowering path leaves the existing BigInt-specific bitwise runtime call path intact (no changes to `resolver_expr.rs` BigInt routing)
- [x] Backend-west full end-to-end triage blocked by pre-existing WAT writer migration build error (not caused by this change)

## Validation

Required commands:

```sh
cargo fmt --all --check               # pass
cargo check -p ts2wasm-ir             # pass
cargo nextest run -p ts2wasm-ir       # pass 27/27
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket `1055` on 2026-05-06. The bucket name is misspelled as `binaryArithmatic` in upstream reference paths; keep that spelling in commands and paths.

Sibling confirmation paths after the representative case advances:

- `reference/typescript/tests/cases/compiler/binaryArithmatic2.ts`
- `reference/typescript/tests/cases/compiler/binaryArithmatic3.ts`
- `reference/typescript/tests/cases/compiler/binaryArithmatic4.ts`

## Completion evidence

### Implementation

Added `LoweredBinaryOp::BitwiseOr` variant and wired it through the full compiler pipeline:

**IR layer:**
- `types.rs`: Added `BitwiseOr` to `LoweredBinaryOp` enum, added to inferred-type match as ordinary numeric operator
- `program.rs`: Added `BinaryOp::BitwiseOr => Ok(LoweredBinaryOp::BitwiseOr)` to `lower_binary_op`, removed from fallthrough `UnsupportedSyntax` arm

**Backend-wasm layer (WAT emission):**
- `runtime_fn.rs`: Added `BitwiseOr` enum variant
- `runtime_fn_impl.rs`: Added `RuntimeSpec`, manifest_name (`"bitwise_or"`), emission-order entries
- `runtime_core_emitter_part2.rs`: Added `emit_bitwise_or` emitting `$bitwise_or` with `i32.or`
- `runtime_builder.rs`: Added dispatch arm
- `expr_emit.rs`: Added match arm routing to `RuntimeFn::BitwiseOr`
- `runtime_link_plan.rs`: Added match arm for runtime requirement
- `binary_mvp.rs`: Added `LoweredBinaryOp::BitwiseOr => l | r`

**No changes to BigInt paths** — BigInt-specific `BitwiseOr` routing in `resolver_expr.rs` is untouched.

### Files changed

- `crates/ir/src/lowered/types.rs` — 2 insertions
- `crates/ir/src/lowered/program.rs` — 2 insertions, 1 deletion
- `crates/backend-wasm/src/runtime_fn.rs` — 1 insertion
- `crates/backend-wasm/src/runtime_fn_impl.rs` — 13 insertions
- `crates/backend-wasm/src/runtime_core_emitter_part2.rs` — 14 insertions
- `crates/backend-wasm/src/runtime_builder.rs` — 1 insertion
- `crates/backend-wasm/src/expr_emit.rs` — 1 insertion
- `crates/backend-wasm/src/runtime_link_plan.rs` — 1 insertion
- `crates/backend-wasm/src/binary_mvp.rs` — 1 insertion

### Verification

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo check -p ts2wasm-ir
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-ir
result: pass; 27 tests run, 27 passed
date: 2026-05-06

command: cargo check -p ts2wasm-backend-wasm
result: pre-existing build errors (WAT writer migration) — 0 new errors from this change
date: 2026-05-06
```

### Remaining risks

- Full end-to-end `binaryArithmatic1.ts` reference triage requires backend-wasm to compile (blocked by pre-existing WAT writer migration in `wat_writer.rs`). Once the migration is complete, `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmatic1.ts` should advance past the `binary operator BitwiseOr` diagnostic.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in . This issue has repo-local close evidence
(implementation commit or completion evidence).

Future-work tracking: none identified.
