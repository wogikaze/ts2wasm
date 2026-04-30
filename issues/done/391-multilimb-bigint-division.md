---
id: 391
title: "Multi-limb BigInt division"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [259, 260]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Closed as superseded by issue 384 for the validated known-BigInt division slice, with the only remaining residual explicitly tracked by issue 398.

Problem: Dynamic BigInt `/` with operands or results outside the signed-i64-backed helper slice used to report diagnostics instead of using canonical BigInt arithmetic. Issue 384 now implements the validated cached-decimal division/remainder path for known BigInt operands outside signed i64. Branch/control-flow-assigned locals that are still conservatively invalidated are not part of this closed issue; they are tracked by issue 398.

## Problem

The current dynamic helpers previously reconstructed operands through signed i64 and the issue-259 first-limb/cached-decimal constructor. That was safe only when the resolver proved operands and results fit the signed-i64 helper slice, while dynamic BigInt division outside that slice was rejected with issue-369 diagnostics instead of matching Node for arbitrary BigInt magnitudes.

## Current status

Issue 384 closed the known-BigInt operand division/remainder slice with Node/iwasm differential evidence:

```text
PASS: cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_div_rem
PASS: cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_mul_div_rem
```

The remaining residual is not a separate division arithmetic implementation gap in this issue. It is resolver tracking for branch/control-flow-assigned BigInt locals, tracked by issue 398.

## Desired final state

Dynamic BigInt `/` operates on the canonical heap BigInt representation for supported known BigInt operands outside signed i64 and matches Node output. Control-flow-assigned local tracking is completed by issue 398.

## Scope

In scope:

- [x] Confirm issue 384 covers known dynamic BigInt division with operands/results outside signed i64.
- [x] Confirm remaining branch/control-flow-assigned local tracking is represented by issue 398.
- [x] Close this duplicate/superseded split with evidence rather than leaving overlapping open work.

Out of scope:

- Parser BigInt literal syntax.
- Literal-only arithmetic folding already closed by issue 260.
- Addition, subtraction, multiplication, remainder; issues 393, 394, 383, 392.
- BigInt bitwise/exponentiation; issue 371.
- Branch/control-flow-assigned BigInt `/` and `%` local tracking; issue 398.
- BigInt equality/comparison/builtins except where tests need arithmetic setup.

## Affected paths

Touched:

- `issues/done/391-multilimb-bigint-division.md`
- `issues/index.md`

Implementation already landed through issue 384:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

## Acceptance criteria

- [x] Node/iwasm differential fixtures cover dynamic BigInt div with operands or results outside signed i64 for the known-BigInt operand slice through issue 384.
- [x] Existing signed-i64 slice fixtures from issue 260 continue to match Node through the required `bigint_large_div_rem` gate.
- [x] Runtime linker structure did not require a new public helper variant for this superseded issue; issue 384 records existing BigIntDiv/BigIntRem catalog coverage.
- [x] Docs/current-state/issues state the new division boundary through issue 384 and the residual issue 398.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_div_rem
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -E 'test(bigint) or test(node_diff)'
```

Not run:

- `cargo nextest run -E 'test(bigint) or test(node_diff)'`; this lifecycle-only close used the assignment-required narrow gate.

## Docs / current-state / issue sync

Final-state docs:

- [x] already updated by issue 384: `docs/14-runtime-abi.md`
- [x] already updated by issue 384: `docs/language-reference/javascript-features.md`

Current state:

- [x] already updated by issue 384: `current-state.md` (repo root)

Follow-up issues:

- [x] issue 398 tracks branch/control-flow-assigned BigInt div/rem locals

## Notes

This issue is not closed by hiding remaining work. The known-BigInt division implementation is superseded by issue 384, and the exact remaining residual is issue 398.

## Completion evidence

Commits:

- `3672eee7` issue-391: close superseded bigint division split

Validation result:

```text
PASS: cargo fmt --all --check
PASS: cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_div_rem
PASS: mise run update-issue-index -- --check
PASS: mise run check issues
```

Close note (2026-05-01): issue 391 is superseded by issue 384 for the validated known-BigInt dynamic division slice. The only residual division-specific gap named in the issue is branch/control-flow-assigned local tracking, which is tracked by issue 398.
