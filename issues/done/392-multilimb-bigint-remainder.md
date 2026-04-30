---
id: 392
title: "Multi-limb BigInt remainder"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [259, 260, 391]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Closed as superseded by issue 384 for the validated known-BigInt remainder slice, with branch/control-flow-assigned local coverage completed by issue 398.

Problem: Dynamic BigInt `%` with operands or results outside the signed-i64-backed helper slice used to report diagnostics instead of using canonical BigInt limb arithmetic. Issue 384 implemented the cached-decimal division/remainder runtime path for known BigInt operands outside signed i64. Issue 398 then added the supported if/else branch-assigned local tracking slice for `/` and `%`, including outside-signed-i64 operands.

## Problem

The previous dynamic helpers reconstructed operands through signed i64 and the issue-259 first-limb/cached-decimal constructor. That was safe only when the resolver proved operands and results fit the signed-i64 helper slice. Dynamic BigInt remainder outside that slice was rejected with issue-369 diagnostics instead of matching Node for arbitrary BigInt magnitudes.

## Current status

Issue 384 closed the known-BigInt operand division/remainder slice with Node/iwasm differential evidence:

```text
PASS: cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_div_rem
```

The existing `bigint_large_div_rem` filter covers these remainder fixtures:

```text
fixtures/core-semantics/bigint-runtime-large-div-rem.ts
fixtures/core-semantics/bigint-runtime-large-div-rem-local-flow.ts
fixtures/core-semantics/bigint-runtime-branch-large-div-rem.ts
fixtures/core-semantics/bigint-runtime-branch-mixed-div-rem-unsupported.ts
```

Those fixtures include `%` with operands outside signed i64, negative remainder sign behavior, canonical small remainders from outside-signed-i64 dividends, known local/literal operands, and supported if/else branch-assigned BigInt locals. Mixed BigInt/Number branch assignment still reports issue-370 rather than silently lowering.

## Desired final state

Dynamic BigInt `%` operates on the canonical heap BigInt representation for the supported known BigInt and supported if/else branch-assigned local slices outside signed i64 and matches Node output.

## Scope

In scope:

- [x] Confirm issue 384 covers known dynamic BigInt remainder with operands/results outside signed i64.
- [x] Confirm issue 398 covers supported if/else branch-assigned BigInt remainder locals outside signed i64.
- [x] Close this duplicate/superseded split with evidence rather than leaving overlapping open work.

Out of scope:

- Parser BigInt literal syntax.
- Literal-only arithmetic folding already closed by issue 260.
- Addition, subtraction, multiplication, division; issues 393, 394, 383, 391.
- BigInt bitwise/exponentiation; issue 371.
- BigInt equality/comparison/builtins except where tests need arithmetic setup.
- Broader nested control-flow tracking outside the supported if/else div/rem slice.
- Compatible JavaScript `RangeError` object throwing for `% 0n`; issues 370 and 380.

## Affected paths

Touched for this close:

- `issues/done/392-multilimb-bigint-remainder.md`
- `issues/index.md`

Implementation already landed through issues 384 and 398:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

## Acceptance criteria

- [x] Node/iwasm differential fixtures cover dynamic BigInt rem with operands or results outside signed i64 for known locals/literal operands through issue 384.
- [x] Node/iwasm differential fixtures cover supported if/else branch-assigned BigInt locals using `%` outside signed i64 through issue 398.
- [x] Existing signed-i64 slice fixtures from issue 260 continue to match Node through the required `bigint_large_div_rem` gate.
- [x] Runtime linker structure did not require a new public helper variant for this superseded issue; issue 384 records existing BigIntRem catalog coverage, and `BigIntRem` depends on `BigIntDiv` for the shared helper path.
- [x] Docs/current-state/issues state the new remainder boundary through issues 384 and 398.

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

- [x] already updated by issues 384 and 398: `current-state.md` (repo root)

Follow-up issues:

- [x] none for the supported remainder slice; broader nested control-flow tracking remains outside this issue's supported if/else branch-assigned scope.
- [x] BigInt `% 0n` compatible `RangeError` object throwing remains tracked by issues 370 and 380.

## Notes

This issue is not closed by hiding remaining work. The known-BigInt remainder implementation is superseded by issue 384, and the assigned branch/control-flow local concern is covered by issue 398 for the supported if/else div/rem slice.

## Completion evidence

Commits:

- `4e9b2fc6` issue-392: close superseded bigint remainder split

Validation result:

```text
PASS: cargo fmt --all --check
PASS: cargo test -p ts2wasm-cli --test m2_node_diff bigint_large_div_rem (4 passed)
PASS: mise run update-issue-index -- --check
PASS: mise run check issues
```

Close note (2026-05-01): issue 392 is superseded by issue 384 for the validated known-BigInt dynamic remainder slice and covered by issue 398 for supported if/else branch-assigned BigInt div/rem locals outside signed i64.
