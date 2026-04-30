---
id: 383
title: "Multi-limb BigInt multiplication"
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

Implement canonical multi-limb BigInt multiplication for dynamic operands and results outside the signed-i64-backed helper slice.

Problem: Dynamic BigInt `*` with operands or results outside the signed-i64-backed helper slice report diagnostics instead of using canonical BigInt limb arithmetic.

## Problem

The current dynamic helpers reconstruct operands through signed i64 and the issue-259 first-limb/cached-decimal constructor. This is safe only when the resolver proves operands and results fit the signed-i64 helper slice.

Problem: Dynamic BigInt multiplication outside the signed-i64-backed helper slice is rejected with issue-369 diagnostics instead of matching Node for arbitrary BigInt magnitudes.

## Current failure

Representative unsupported cases:

```sh
cargo test -p ts2wasm-cli bigint_runtime_large_mul_reports_issue_369
```

These fixtures currently build-fail with an issue-369 diagnostic because the operands or results are outside the signed-i64-backed dynamic helper slice.

## Desired final state

Dynamic BigInt `*` operates on the canonical heap BigInt limb representation for arbitrary supported BigInt magnitudes and matches Node output.

## Scope

In scope:

- [ ] Implement canonical limb mul for dynamic BigInt operands and results.
- [ ] Preserve canonical zero and sign behavior for multiplication.
- [ ] Keep source-backed diagnostics only for genuinely unsupported runtime representation or memory limits.
- [ ] Add Node/iwasm differential fixtures for values larger than signed i64, including large multiplication results.
- [ ] Update runtime linker structure tests if new helpers/deps are added.
- [ ] Update `docs/14-runtime-abi.md`, `docs/language-reference/javascript-features.md`, and `current-state.md`.

Out of scope:

- Parser BigInt literal syntax.
- Literal-only arithmetic folding already closed by issue 260.
- Addition, subtraction, division, remainder; issues 382, 384.
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

- Parser BigInt literal syntax.
- Unrelated number arithmetic behavior.
- Addition, subtraction, division, remainder helpers.

## Acceptance criteria

- [ ] Node/iwasm differential fixtures cover dynamic BigInt mul with operands or results outside signed i64.
- [ ] Branch/loop/switch/try-assigned BigInt locals no longer lose correctness when the assigned value is a valid multi-limb BigInt.
- [ ] Existing signed-i64 slice fixtures from issue 260 continue to match Node.
- [ ] Runtime linker structure tests cover any new multi-limb helper deps.
- [ ] Docs/current-state/issues state the new multiplication boundary.

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

This is a focused split from issue 369, covering only multiplication. Do not implement this by widening the signed-i64 conversion path. The compatibility target is the canonical heap BigInt limb representation.

## Progress evidence

2026-05-01 progress slice:

- Implemented cached-decimal schoolbook multiplication in `$bigint_mul` for known BigInt local/literal operands outside signed i64.
- Converted `fixtures/core-semantics/bigint-runtime-large-mul.ts` to Node/iwasm differential coverage for large positive, negative, and zero multiplication.
- Remaining open acceptance: branch/loop/switch/try-assigned BigInt locals whose static value is not tracked still require resolver/control-flow tracking before this issue can move to done.
- Validation target: `cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_mul`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```

Remaining risks:

- Multiplication may require a deliberately simple but correct limb algorithm before optimization.
