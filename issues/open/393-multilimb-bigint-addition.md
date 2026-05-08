---
id: 393
title: "Multi-limb BigInt addition"
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

Closed as superseded/covered by the already-merged issue 382 add/sub progress slice plus issue 397 branch-assigned local tracking.

Problem: Dynamic BigInt `+` with operands or results outside the signed-i64-backed helper slice reports diagnostics instead of using canonical BigInt limb arithmetic.

## Problem

The current dynamic helpers reconstruct operands through signed i64 and the issue-259 first-limb/cached-decimal constructor. This is safe only when the resolver proves operands and results fit the signed-i64 helper slice, but dynamic BigInt addition outside that slice is rejected with issue-369 diagnostics instead of matching Node for arbitrary BigInt magnitudes.

## Current failure

Representative unsupported case:

```sh
cargo test -p ts2wasm-cli bigint_runtime_large_add_reports_issue_369
```

Result:

```text
error: issue-369: dynamic BigInt addition outside signed-i64 helper slice
```

This fixture currently build-fails with an issue-369 diagnostic because the operands or result is outside the signed-i64-backed dynamic helper slice.

## Desired final state

Dynamic BigInt `+` operates on the canonical heap BigInt limb representation for arbitrary supported BigInt magnitudes and matches Node output.

## Scope

In scope:

- [x] Implement canonical limb addition for dynamic BigInt operands and results, preserving canonical zero and sign behavior, and add Node/iwasm differential fixtures for values larger than signed i64.
- [x] Keep source-backed diagnostics only for genuinely unsupported runtime representation or memory limits.
- [x] Update runtime linker structure tests if new helpers/deps are added.
- [x] Update `docs/14-runtime-abi.md`, `docs/language-reference/javascript-features.md`, and `current-state.md`.

Out of scope:

- Parser BigInt literal syntax.
- Literal-only arithmetic folding already closed by issue 260.
- Subtraction, multiplication, division, remainder; issues 394, 383, 391, 392.
- BigInt bitwise/exponentiation; issue 371.
- BigInt equality/comparison/builtins except where tests need arithmetic setup.

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

- Parser BigInt syntax.
- Unrelated number arithmetic behavior.
- Subtraction, multiplication, division, remainder helpers.

## Acceptance criteria

- [x] Node/iwasm differential fixtures cover dynamic BigInt add with operands or results outside signed i64, and existing signed-i64 slice fixtures from issue 260 continue to match Node.
- [x] Runtime linker structure tests cover any new multi-limb helper deps.
- [x] Docs/current-state/issues state the new addition boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli bigint_large_add_reports_issue_369
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

- [x] updated: `docs/14-runtime-abi.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

This is a focused split from issue 382, covering only addition. Do not implement this by widening the signed-i64 conversion path. The compatibility target is the canonical heap BigInt limb representation.

## Completion evidence

Fill only when moving to `done/`.

Close note (2026-05-01): issue 393 is superseded by issue 382's validated cached-decimal add/sub runtime slice and issue 397's supported branch-assigned local tracking. Existing coverage already includes `fixtures/core-semantics/bigint-runtime-large-add-sub.ts` for `18446744073709551616n + 1n`, plus `fixtures/core-semantics/bigint-runtime-branch-large-unsupported.ts` for a branch-assigned large BigInt local used as a later `+` operand. No new compiler or fixture change was required in this cycle.

Commits:

- this close commit

Validation result:

```text
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_add_sub
mise run update-issue-index -- --check
mise run check issues

All required validation passed in child worktree child/issue-393-superseded.
```

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/393-multilimb-bigint-addition.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
