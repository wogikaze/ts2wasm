---
id: 380
title: "BigInt division/remainder by zero RangeError"
type: feature
area: runtime/semantics
class: blocked
priority: P2
depends_on: [260, 396]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement compatible JavaScript `RangeError` throwing for BigInt division and remainder by zero.

Problem: BigInt `/ 0n` and `% 0n` currently reach a runtime `unreachable` trap in the signed-i64 helper slice, while Node throws `RangeError: Division by zero`.

Current split status: the diagnostic/abort surface is implemented and covered, but catchable JavaScript `RangeError` object propagation depends on issue 396.

## Problem

Issue 260 proved the arithmetic slice without silent coercion, but it did not add JS exception throwing machinery. Node throws `RangeError: Division by zero` for BigInt `/` and `%` by zero, while iwasm currently traps.

Problem: BigInt division/remainder by zero error paths are not compatible JS throws yet. They now report `RangeError: Division by zero` before aborting, but that is not a catchable JS `RangeError` object.

## Current failure

Original representative evidence:

```sh
cargo test -p ts2wasm-cli bigint_runtime_div_zero_traps_after_successful_build
cargo test -p ts2wasm-cli bigint_runtime_rem_zero_traps_after_successful_build
```

Current evidence proves Node reports `RangeError: Division by zero` and iwasm emits the matching diagnostic text before aborting. Catchable exception object propagation remains blocked on issue 396.

## Desired final state

BigInt division/remainder by zero throw a compatible `RangeError` object matching Node behavior.

## Scope

In scope:

- [x] Add the interim runtime diagnostic/abort surface for `RangeError: Division by zero`.
- [ ] After issue 396 lands, use its runtime exception substrate for a catchable `RangeError` object.
- [ ] Convert BigInt division-by-zero trap to compatible `RangeError` throwing for supported helper paths.
- [ ] Convert BigInt remainder-by-zero trap to compatible `RangeError` throwing for supported helper paths.
- [ ] Add Node/iwasm differential or exception-parity fixture for `BigInt / 0n` as `RangeError: Division by zero`.
- [ ] Add Node/iwasm differential or exception-parity fixture for `BigInt % 0n` as `RangeError: Division by zero`.
- [ ] Update docs/current-state/issues with the exception boundary.

Out of scope:

- Mixed Number/BigInt arithmetic TypeError; issue 381.
- Full multi-limb arithmetic; issue 369.
- General catch/finally exception semantics unless required for a minimal throw-compatible test.
- BigInt builtin string exception parity; issue 333.

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
- Multi-limb arithmetic algorithms except as needed to keep tests compiling.
- Unrelated Error constructor behavior.

## Acceptance criteria

- [ ] Node/iwasm differential or explicit exception-parity fixture covers BigInt `/ 0n` as `RangeError: Division by zero`.
- [ ] Node/iwasm differential or explicit exception-parity fixture covers BigInt `% 0n` as `RangeError: Division by zero`.
- [ ] Existing issue-260 signed-i64 arithmetic success fixtures continue to pass.
- [ ] Docs/current-state/issues state the division/remainder by zero exception boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_div_zero_reports_rangeerror_after_successful_build
cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_rem_zero_reports_rangeerror_after_successful_build
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

This is a focused split from issue 370, covering only division/remainder by zero `RangeError` throwing.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

## Progress evidence

2026-05-01 progress:

- Added the minimal runtime exception diagnostic substrate surface for BigInt `/ 0n` and `% 0n`.
- `bigint_div_rem_decimal` now writes `RangeError: Division by zero` before aborting when the divisor BigInt is canonical zero.
- Updated the existing Node/iwasm zero-divisor tests to assert Node's `RangeError: Division by zero` baseline and iwasm's matching RangeError diagnostic surface instead of only a bare `unreachable` trap.
- Updated current-state and BigInt runtime ABI/language-reference docs to state the diagnostic/abort boundary.
- The issue remains open because full catchable JavaScript `RangeError` object propagation is broader runtime exception substrate work.

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_div_zero_reports_rangeerror_after_successful_build: pass
cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_rem_zero_reports_rangeerror_after_successful_build: pass
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

Remaining risks:

- This may need a small cross-cutting runtime exception primitive before BigInt-specific helpers can throw compatibly.

2026-05-01 split/update:

- Reclassified the issue as blocked on issue 396 instead of leaving it as directly implementation-ready.
- Diagnostic/abort behavior is treated as progress only, not DONE: the required catchable `RangeError` object still does not exist.
- The remaining acceptance slice is precise: after issue 396 provides a runtime JS exception object substrate, migrate `/ 0n` and `% 0n` from diagnostic abort to catchable `RangeError: Division by zero` and add the corresponding catch/exception parity test.
