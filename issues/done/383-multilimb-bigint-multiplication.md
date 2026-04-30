---
id: 383
title: "Multi-limb BigInt multiplication"
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

Closed for the validated cached-decimal BigInt multiplication slice, including known BigInt operands and branch/loop/switch/try-assigned locals that remain BigInt.

Problem: Dynamic BigInt `*` with operands or results outside the signed-i64-backed helper slice previously reported diagnostics or lacked control-flow-assigned local coverage instead of using canonical BigInt arithmetic.

## Problem

The previous dynamic helpers reconstructed operands through signed i64 and the issue-259 first-limb/cached-decimal constructor. That was safe only when the resolver proved operands and results fit the signed-i64 helper slice.

## Current failure

Representative unsupported cases before closure:

```sh
cargo test -p ts2wasm-cli bigint_runtime_large_mul_reports_issue_369
```

These cases built-failed with issue-linked diagnostics because the operands or results were outside the signed-i64-backed dynamic helper slice.

## Desired final state

Dynamic BigInt `*` operates on the current cached-decimal heap BigInt representation for supported arbitrary magnitudes and matches Node output for known and control-flow-assigned BigInt locals.

## Scope

In scope:

- [x] Implement canonical multiplication for dynamic BigInt operands and results in the current cached-decimal representation.
- [x] Preserve canonical zero and sign behavior for multiplication.
- [x] Keep source-backed diagnostics only for genuinely unsupported runtime representation or memory limits.
- [x] Add Node/iwasm differential fixtures for values larger than signed i64, including large multiplication results.
- [x] Update runtime linker structure tests if new helpers/deps are added; no new RuntimeFn/deps were required for this close slice.
- [x] Update `docs/14-runtime-abi.md`, `docs/language-reference/javascript-features.md`, and `current-state.md`.

Out of scope:

- Parser BigInt literal syntax.
- Literal-only arithmetic folding already closed by issue 260.
- Addition, subtraction, division, remainder; issues 382, 384, and 398.
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

- [x] Node/iwasm differential fixtures cover dynamic BigInt mul with operands or results outside signed i64.
- [x] Branch/loop/switch/try-assigned BigInt locals no longer lose correctness when the assigned value is a valid multi-limb BigInt.
- [x] Existing signed-i64 slice fixtures from issue 260 continue to match Node under the required `bigint_large_mul` validation filter's retained baseline fixture.
- [x] Runtime linker structure tests cover any new multi-limb helper deps; no new runtime helper/deps were added for the final control-flow coverage slice.
- [x] Docs/current-state/issues state the new multiplication boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_mul
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -E 'test(bigint) or test(node_diff)'
```

Not run:

- `cargo nextest run -E 'test(bigint) or test(node_diff)'`; not required by the parent assignment for this child slice.

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/14-runtime-abi.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none for multiplication; control-flow-assigned division/remainder remains tracked separately by issue 398.

## Notes

This is a focused split from issue 369, covering multiplication. It does not widen the signed-i64 conversion path; multiplication uses the existing cached-decimal BigInt representation.

## Progress evidence

2026-05-01 progress slice:

- Implemented cached-decimal schoolbook multiplication in `$bigint_mul` for known BigInt local/literal operands outside signed i64.
- Converted `fixtures/core-semantics/bigint-runtime-large-mul.ts` to Node/iwasm differential coverage for large positive, negative, and zero multiplication.
- Remaining open acceptance at that point was branch/loop/switch/try-assigned BigInt locals whose static value is not tracked.
- Validation target: `cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_mul`.

2026-05-01 close slice:

- Added `fixtures/core-semantics/bigint-runtime-large-mul-local-flow.ts` for branch, loop, switch, and try/finally-assigned BigInt locals multiplied outside signed i64.
- Added `bigint_large_mul_local_flow_fixture_matches_node_output_under_iwasm` to the Node/iwasm differential suite.
- Updated runtime ABI docs, language reference, and current-state boundary text to remove issue 383 from remaining multiplication work.

## Completion evidence

Commits:

- `99717e7f` issue-383: add cached-decimal bigint multiplication slice
- `e8b272af` issue-383: cover control-flow bigint multiplication

Validation result:

```text
PASS: cargo fmt --all --check
PASS: cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_mul
PASS: mise run update-issue-index -- --check
PASS: mise run check issues
```

Close note (2026-05-01): issue 383 is closed for the validated multiplication boundary. The remaining branch/control-flow-assigned div/rem boundary is issue 398, not hidden under this close.
