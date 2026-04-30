---
id: 387
title: "Implement BigInt bitwise outside signed-i64 helper slice"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [260, 377]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement BigInt `~`, `&`, `|`, and `^` beyond the signed-i64-safe helper slice closed by issue 377.

Problem: issue 377 added BigInt-specific helpers for known operands/results that fit the signed-i64-backed first-limb constructor boundary, but larger operands or results still report issue-387 diagnostics.

## Problem

BigInt bitwise operators use arbitrary-width two's-complement semantics. The current helper slice converts through signed i64 and is intentionally rejected when the resolver cannot prove the operand/result stays inside that boundary. Problem: out-of-slice BigInt bitwise NOT/AND/OR/XOR remain unsupported and need canonical multi-limb/two's-complement semantics.

## Current failure

Representative fixtures:

```sh
cargo test -p ts2wasm-cli --test m2_node_diff bigint_bitwise_unary_out_of_slice_reports_issue_387
```

Result:

```text
error: issue-387: BigInt bitwise NOT outside signed-i64 helper slice
```

```sh
cargo test -p ts2wasm-cli --test m2_node_diff bigint_bitwise_mixed_reports_issue_387
```

Result:

```text
error: issue-387: BigInt bitwise AND/OR/XOR outside signed-i64 helper slice
```

Current result: source-backed `issue-387` diagnostics for unsupported bitwise forms.

## Desired final state

BigInt bitwise NOT/AND/OR/XOR operate on the canonical heap BigInt representation for all supported magnitudes and match Node output without lowering through ordinary number bitwise operators.

## Scope

In scope:

- [ ] Implement multi-limb/two's-complement semantics for BigInt bitwise NOT/AND/OR/XOR, preserving TypeError ownership for mixed Number/BigInt behavior.
- [ ] Add Node/iwasm differential fixtures outside the signed-i64 helper slice.
- [ ] Update runtime ABI docs and current-state.

Out of scope:

- Signed-i64-safe BigInt bitwise helper slice; closed by issue 377.
- BigInt shift operators and unsigned-right-shift policy; issue 378.
- General multi-limb arithmetic; issues 382, 383, 384.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md`
- `current-state.md`

Do not touch:

- Parser BigInt literal syntax.
- Ordinary number bitwise lowering.

## Acceptance criteria

- [ ] BigInt bitwise NOT/AND/OR/XOR fixtures outside signed i64 match Node/iwasm with no lowering through ordinary number bitwise helpers, and unsupported mixed or exception forms have source-backed diagnostics or compatible TypeError behavior.
- [ ] Docs/current-state/issues are synchronized.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli bigint_bitwise_unary_out_of_slice_reports_issue_387
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -E 'test(bigint) or test(node_diff)'
```

## Completion evidence

Fill only when moving to `done/`.

## Progress evidence

- 2026-05-01: Implemented the first issue-387 progress slice for static BigInt literal `~` outside the signed-i64 helper boundary. The fold now uses the arbitrary-decimal identity `~x == -x - 1`, preserving issue-387 diagnostics for dynamic/out-of-slice runtime bitwise and mixed Number/BigInt ownership.
- 2026-05-01: Implemented a second issue-387 progress slice for static binary BigInt literal `&`, `|`, and `^` outside the signed-i64 helper boundary using arbitrary-width two's-complement fold logic. Added Node/iwasm differential fixture coverage and kept dynamic/mixed issue-387 diagnostics in place.
- 2026-05-01: Implemented a third issue-387 progress slice for local-known BigInt `~`, `&`, `|`, and `^` expressions outside signed i64. The existing static folder now folds tracked BigInt locals and local reassignments to canonical BigInt literals before lowering, so this slice avoids ordinary number bitwise lowering and the signed-i64 runtime helper. Broader untracked dynamic values remain issue-387 diagnostics.
- 2026-05-01: Close audit found issue 387 cannot close yet. Static literal and local-known out-of-signed-i64 fixtures now match Node/iwasm, but control-flow-assigned dynamic BigInt bitwise values outside signed i64 still report source-backed `issue-387` diagnostics. Added `bigint_bitwise_dynamic_out_of_signed_i64_reports_issue_387` as residual evidence for the missing dynamic slice.
- 2026-05-01: Implemented a fourth progress slice for constant-condition control-flow assignments. The static BigInt folder now preserves the executed branch's tracked BigInt assignment for `if (true)` / `if (false)`, allowing the residual out-of-signed-i64 branch-assigned `~`, `&`, `|`, and `^` fixture to fold to canonical BigInt literals and match Node/iwasm without using ordinary number bitwise helpers. Broader non-constant control-flow and untracked dynamic bitwise values remain open.
