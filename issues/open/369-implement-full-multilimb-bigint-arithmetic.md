---
id: 369
title: "Implement full multi-limb BigInt arithmetic"
type: feature
area: runtime/semantics
class: blocked
priority: P2
depends_on: [259, 260, 393, 394, 383, 391, 392]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Replace the current signed-i64-backed dynamic BigInt arithmetic helper boundary with canonical multi-limb runtime arithmetic.

Problem: issue 260 closed the literal-folding and signed-i64-backed dynamic unary/add/sub/mul/div/rem slice, but dynamic operands or results outside that slice still report diagnostics instead of using canonical BigInt limb arithmetic.

## Problem

The current dynamic helpers reconstruct operands through signed i64 and the issue-259 first-limb/cached-decimal constructor. This is safe only when the resolver proves operands and results fit the signed-i64 helper slice.

Problem: dynamic BigInt arithmetic outside the signed-i64-backed helper slice is rejected with issue-369 diagnostics instead of matching Node for arbitrary BigInt magnitudes.

## Current failure

Representative unsupported cases:

```sh
cargo test -p ts2wasm-cli bigint_runtime_large_add_reports_issue_369
cargo test -p ts2wasm-cli bigint_runtime_large_sub_reports_issue_369
cargo test -p ts2wasm-cli bigint_runtime_large_mul_reports_issue_369
```

These fixtures currently build-fail with an issue-369 diagnostic because the operands or results are outside the signed-i64-backed dynamic helper slice.

## Desired final state

Dynamic BigInt unary minus and binary `+`, `-`, `*`, `/`, and `%` operate on the canonical heap BigInt limb representation for arbitrary supported BigInt magnitudes and match Node output. The signed-i64 pre-lowering arithmetic limit is removed or retained only as an optimization fast path with a correct multi-limb fallback.

## Scope

In scope:

- [ ] Implement canonical limb add/sub for dynamic BigInt operands and results.
- [ ] Implement canonical limb mul for dynamic BigInt operands and results.
- [ ] Implement canonical truncating division and remainder for dynamic BigInt operands and results.
- [ ] Preserve canonical zero and sign behavior for all arithmetic operations.
- [ ] Keep source-backed diagnostics only for genuinely unsupported runtime representation or memory limits, not for ordinary multi-limb arithmetic.
- [ ] Add Node/iwasm differential fixtures for values larger than signed i64, including branch-assigned locals and large multiplication results.
- [ ] Update runtime linker structure tests if new helpers/deps are added.
- [ ] Update `docs/14-runtime-abi.md`, `docs/language-reference/javascript-features.md`, and `current-state.md`.

Out of scope:

- Parser BigInt literal syntax.
- Literal-only arithmetic folding already closed by issue 260.
- Compatible `RangeError` / `TypeError` throwing for arithmetic exceptions; issue 380.
- BigInt bitwise and exponentiation policy; issue 371.
- BigInt equality/comparison/builtins except where tests need arithmetic setup.

Split from this issue:
- Issue 393: Multi-limb BigInt addition (focused on addition)
- Issue 394: Multi-limb BigInt subtraction (focused on subtraction)
- Issue 383: Multi-limb BigInt multiplication (focused on multiplication)
- Issue 391: Multi-limb BigInt division (focused on division)
- Issue 392: Multi-limb BigInt remainder (focused on remainder)

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- BigInt parser syntax.
- Unrelated number arithmetic behavior.
- Broad exception machinery except for preserving existing traps/diagnostics.

## Acceptance criteria

- [ ] Node/iwasm differential fixtures cover dynamic BigInt add/sub/mul/div/rem with operands or results outside signed i64.
- [ ] Branch/loop/switch/try-assigned BigInt locals no longer lose correctness when the assigned value is a valid multi-limb BigInt.
- [ ] Existing signed-i64 slice fixtures from issue 260 continue to match Node.
- [ ] Existing out-of-slice diagnostics are removed or narrowed to true unsupported memory/representation limits.
- [ ] Runtime linker structure tests cover any new multi-limb helper deps.
- [ ] Docs/current-state/issues state the new arithmetic boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -E 'test(bigint) or test(node_diff)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/14-runtime-abi.md`
- [ ] updated: `docs/language-reference/javascript-features.md`

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Do not implement this by widening the signed-i64 conversion path. The compatibility target is the canonical heap BigInt limb representation, with signed-i64 logic allowed only as a proven fast path.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```

Remaining risks:

- Division/remainder may require a deliberately simple but correct limb algorithm before optimization.
