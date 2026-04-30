---
id: 382
title: "Multi-limb BigInt addition and subtraction"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [259, 260]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement canonical multi-limb BigInt addition and subtraction for dynamic operands and results outside the signed-i64-backed helper slice.

Problem: Dynamic BigInt `+` and `-` with operands or results outside the signed-i64-backed helper slice report diagnostics instead of using canonical BigInt limb arithmetic.

## Problem

The current dynamic helpers reconstruct operands through signed i64 and the issue-259 first-limb/cached-decimal constructor. This is safe only when the resolver proves operands and results fit the signed-i64 helper slice.

Problem: Dynamic BigInt addition/subtraction outside the signed-i64-backed helper slice is rejected with issue-369 diagnostics instead of matching Node for arbitrary BigInt magnitudes.

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

- [ ] Node/iwasm differential fixtures cover dynamic BigInt add/sub with operands or results outside signed i64.
- [ ] Branch/loop/switch/try-assigned BigInt locals no longer lose correctness when the assigned value is a valid multi-limb BigInt.
- [ ] Existing signed-i64 slice fixtures from issue 260 continue to match Node.
- [ ] Runtime linker structure tests cover any new multi-limb helper deps.
- [ ] Docs/current-state/issues state the new addition/subtraction boundary.

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

This is a focused split from issue 369, covering only addition and subtraction. Do not implement this by widening the signed-i64 conversion path. The compatibility target is the canonical heap BigInt limb representation.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```
