---
id: 382
title: "Multi-limb BigInt addition and subtraction"
type: feature
area: runtime/semantics
class: blocked
priority: P2
depends_on: [259, 260, 393, 394]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

This issue has been split into:
- Issue 393: Multi-limb BigInt addition
- Issue 394: Multi-limb BigInt subtraction

This issue is now blocked on the completion of its split child issues.

## Problem

The current dynamic helpers reconstruct operands through signed i64 and the issue-259 first-limb/cached-decimal constructor. This is safe only when the resolver proves operands and results fit the signed-i64 helper slice, but dynamic BigInt addition/subtraction outside that slice is rejected with issue-369 diagnostics instead of matching Node for arbitrary BigInt magnitudes.

## Current failure

Representative unsupported cases:

```sh
cargo test -p ts2wasm-cli bigint_runtime_large_add_reports_issue_369
```

Result:

```text
error: issue-369: dynamic BigInt addition outside signed-i64 helper slice
```

```sh
cargo test -p ts2wasm-cli bigint_runtime_large_sub_reports_issue_369
```

Result:

```text
error: issue-369: dynamic BigInt subtraction outside signed-i64 helper slice
```

These fixtures currently build-fail with an issue-369 diagnostic because the operands or results are outside the signed-i64-backed dynamic helper slice.

## Desired final state

Dynamic BigInt `+` and `-` operate on the canonical heap BigInt limb representation for arbitrary supported BigInt magnitudes and match Node output.

## Scope

In scope:

- [ ] Implement canonical limb addition and subtraction for dynamic BigInt operands and results.
- [ ] Preserve canonical zero and sign behavior for addition and subtraction.
- [ ] Keep source-backed diagnostics only for genuinely unsupported runtime representation or memory limits.
- [ ] Add Node/iwasm differential fixtures for values larger than signed i64, including branch-assigned locals.
- [ ] Update runtime linker structure tests if new helpers/deps are added.
- [ ] Update `docs/14-runtime-abi.md`, `docs/language-reference/javascript-features.md`, and `current-state.md`.

Out of scope:

- Parser BigInt literal syntax.
- Literal-only arithmetic folding already closed by issue 260.
- Multiplication, division, remainder; issues 383, 384.
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
- Multiplication, division, remainder helpers.

## Acceptance criteria

- [ ] Node/iwasm differential fixtures cover dynamic BigInt add/sub with operands or results outside signed i64, including branch-assigned locals.
- [ ] Existing signed-i64 slice fixtures from issue 260 continue to match Node.
- [ ] Runtime linker structure tests cover any new multi-limb helper deps.
- [ ] Docs/current-state/issues state the new addition/subtraction boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_add_sub
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

This issue has been split into:
- Issue 393: Multi-limb BigInt addition
- Issue 394: Multi-limb BigInt subtraction

This is a focused split from issue 369, covering only addition and subtraction. Do not implement this by widening the signed-i64 conversion path. The compatibility target is the canonical heap BigInt limb representation.

## Progress evidence

2026-05-01 child progress slice:

- Implemented cached-decimal runtime `bigint_add` / `bigint_sub` for selected known BigInt operands and results outside signed-i64.
- Added `fixtures/core-semantics/bigint-runtime-large-add-sub.ts` with Node/iwasm differential coverage for large add/sub and a branch-assigned large result local.
- Validation passed: `cargo fmt --all --check`; `cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_add_sub`; `mise run update-issue-index -- --check`; `mise run check issues`.
- Remaining branch-assigned local reused as a later add/sub operand is split to issue 397.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```
