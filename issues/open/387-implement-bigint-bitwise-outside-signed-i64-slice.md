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
