---
id: 394
title: "Multi-limb BigInt subtraction"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [259, 260, 393]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Closed as superseded and covered by the already-merged issue 382 add/sub runtime slice plus issue 397 branch-assigned local tracking.

Dynamic BigInt `-` now matches Node for the validated known-operand and supported if/else branch-assigned local slice outside signed i64, using the shared cached-decimal multi-limb add/sub runtime path rather than widening the signed-i64 helper.

## Problem

The original issue described dynamic BigInt subtraction outside the signed-i64-backed helper slice reporting issue-369 diagnostics instead of matching Node for arbitrary BigInt magnitudes.

The current implementation no longer exhibits that failure for the scoped known-operand and supported branch-assigned-local cases covered by this issue.

## Scope

In scope:

- [x] Dynamic BigInt subtraction for operands and results outside signed i64 in the validated known-operand slice.
- [x] Canonical zero and sign behavior for the covered add/sub runtime path.
- [x] Node/iwasm differential coverage for values larger than signed i64.
- [x] Supported if/else branch-assigned BigInt locals reused as later add/sub operands.
- [x] Documentation/current-state/issue state updated to describe the implemented subtraction boundary.

Out of scope:

- Parser BigInt literal syntax.
- Literal-only arithmetic folding already closed by issue 260.
- Addition-only cleanup, if any, remains outside this issue.
- Multiplication, division, remainder; issues 383, 384, 391, 398.
- BigInt bitwise/exponentiation; issue 371 and follow-ups.
- BigInt equality/comparison/builtins except where tests need arithmetic setup.
- Broader unknown/nested control-flow BigInt tracking beyond the supported branch-assigned local shape.

## Acceptance criteria

- [x] Node/iwasm differential fixtures cover dynamic BigInt sub with operands or results outside signed i64, and existing signed-i64 slice fixtures from issue 260 continue to match Node.
- [x] Runtime linker structure tests did not need updates; issue 382 reused the existing `bigint_sub` runtime link symbol and `BigIntSub -> BigIntAdd` dependency path.
- [x] Docs/current-state/issues state the new subtraction boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_add_sub
mise run update-issue-index -- --check
mise run check issues
```

Result: passed on 2026-05-01 during this close cycle.

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/14-runtime-abi.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none for this subtraction closure; broader out-of-slice BigInt arithmetic remains tracked by existing umbrella/follow-up issues.

## Notes

This issue was a focused split from issue 382, covering subtraction. Issue 382 added the shared cached-decimal add/sub runtime path and `fixtures/core-semantics/bigint-runtime-large-add-sub.ts`; issue 397 extended static BigInt local tracking so a supported branch-assigned large BigInt local can be reused as a later `+` / `-` operand.

The fixture covers the required subtraction behavior:

- `base - one`, where `base = 18446744073709551616n`, proving a dynamic known-operand result outside signed i64.
- `one - base`, proving negative large subtraction outside signed i64.
- `branch = branch - 2n` after `branch` is assigned a large BigInt result inside a supported `if/else`, proving branch-assigned local reuse for subtraction.

## Completion evidence

Commits:

- issue 382 implementation evidence recorded in `issues/done/382-multilimb-bigint-add-sub.md`
- `7d27d84a` issue-397: preserve branch bigint add sub tracking
- close commit: filled by commit that moves this issue to `done/`

Validation result:

```text
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_add_sub
mise run update-issue-index -- --check
mise run check issues

All required validation passed in this close cycle.
```

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/394-multilimb-bigint-subtraction.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
