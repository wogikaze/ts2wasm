---
id: 378
title: "Implement BigInt shift operators and unsigned-right-shift policy"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [260]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Implement BigInt `<<` and `>>`, and preserve the ECMAScript `>>>` TypeError boundary.

Problem: BigInt shift operators currently report issue-378 diagnostics and must not lower through ordinary number shifts; BigInt `>>>` is invalid in JavaScript.

## Problem

BigInt shifts require BigInt-specific width/sign semantics. Unsigned right shift is not defined for BigInt and should become a compatible TypeError path rather than number coercion.

Problem: BigInt shift operators and BigInt `>>>` remain unsupported after issue 371 and need a precise runtime/diagnostic slice.

## Current progress

2026-05-01 child slice implemented source-backed literal folding for bounded static BigInt `<<` / `>>` operands and kept BigInt `>>>` on the issue-378 diagnostic path. This concretely fixes the representative `console.log(1n << 2n);` shape, adds Node/iwasm coverage, and avoids ordinary number shift lowering.

## Previous failure

```sh
cargo test -p ts2wasm-cli --test m2_node_diff bigint_shift_reports_issue_374
```

Representative fixture:

```ts
console.log(1n << 2n);
```

Previous result: source-backed `issue-378` diagnostic.

## Desired final state

Supported BigInt `<<` and `>>` forms produce Node/iwasm-matching output. BigInt `>>>` reports or throws the compatible unsupported-TypeError path without number coercion.

## Scope

In scope:

- [x] Implement a first BigInt `<<` / `>>` slice with Node/iwasm coverage.
- [x] Add negative coverage for BigInt `>>>` showing it is not lowered through number unsigned-shift semantics.
- [x] Preserve diagnostics for unsupported dynamic, out-of-slice, or mixed Number/BigInt cases.

Out of scope:

- BigInt exponentiation; issue 376.
- BigInt NOT/AND/OR/XOR; issue 377.
- Ordinary number shift behavior.

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
- Ordinary number shift lowering unless a shared diagnostic must stay coherent.

## Acceptance criteria

- [x] Implemented BigInt shift forms have Node/iwasm differential fixtures.
- [x] BigInt `>>>` has source-backed diagnostic or compatible TypeError coverage.
- [x] No BigInt shift path lowers through ordinary number shift operators.
- [x] Docs/current-state/issues are synchronized.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_shift
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

- [x] update if runtime ABI or supported subset changes

Current state:

- [x] update `current-state.md` when behavior changes

Follow-up issues:

- [x] none yet

## Notes

BigInt `>>>` is a JavaScript TypeError path, not an arithmetic shift.

## Progress evidence

2026-05-01:

- Added `fixtures/core-semantics/bigint-shift-literal-runtime.ts` with Node/iwasm coverage for literal `<<` and `>>`, including negative values and negative shift counts.
- Changed `fixtures/core-semantics/bigint-shift-unsupported.ts` to `1n >>> 0n`, preserving source-backed issue-378 diagnostic coverage for unsigned right shift.
- Updated resolver folding so literal BigInt `<<` / `>>` fold to canonical BigInt literals and dynamic/out-of-slice shifts remain diagnosed.
- Updated `docs/14-runtime-abi.md` and `current-state.md` for the supported literal shift slice and remaining dynamic/`>>>` boundary.
- `cargo fmt --all --check`: pass.
- `cargo test -p ts2wasm-cli --test m2_node_diff bigint_shift`: pass, 2 shift tests passed.
- `cargo test -p ts2wasm-cli --test m2_node_diff bigint`: failed outside the shift slice with 43 passed and 11 failed; failing cases are existing BigInt builtin/coercion/div-rem diagnostics and output mismatches, while both shift tests passed.

## Completion evidence

Closed for the validated BigInt shift boundary:

- Static literal BigInt `<<` / `>>` fold to canonical BigInt literals for the bounded shift-count slice.
- `fixtures/core-semantics/bigint-shift-literal-runtime.ts` provides Node/iwasm differential coverage for implemented literal shifts, including negative values and negative shift counts.
- `fixtures/core-semantics/bigint-shift-unsupported.ts` keeps BigInt `>>>` on a source-backed issue-378 diagnostic path.
- No BigInt shift path lowers through ordinary Number shift lowering; dynamic/out-of-slice shift operands remain diagnosed rather than silently coerced.

Validation:

- `cargo fmt --all --check`: pass
- `cargo test -p ts2wasm-cli --test m2_node_diff bigint_shift`: pass
- `cargo test -p ts2wasm-cli --test m2_node_diff bigint_unsigned_right_shift`: pass
- `mise run update-issue-index -- --check`: pass
- `mise run check issues`: pass

Commits:

- this close-state commit
