---
id: 380
title: "BigInt division/remainder by zero RangeError"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [260, 396]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Implement compatible JavaScript `RangeError` throwing for BigInt division and remainder by zero.

Problem: BigInt `/ 0n` and `% 0n` currently reach a runtime `unreachable` trap in the signed-i64 helper slice, while Node throws `RangeError: Division by zero`.

Current split status: the diagnostic/abort surface and the supported `try/catch`
RangeError-like object path are implemented and covered.

## Problem

Issue 260 proved the arithmetic slice without silent coercion, but it did not add JS exception throwing machinery. Node throws `RangeError: Division by zero` for BigInt `/` and `%` by zero, while iwasm currently traps.

Problem: BigInt division/remainder by zero error paths need compatible JS
throws. They now report `RangeError: Division by zero` when uncaught and raise
a catchable RangeError-like object with Node-compatible `message` parity when a
supported `try/catch` is active.

## Current failure

Original representative evidence:

```sh
cargo test -p ts2wasm-cli bigint_runtime_div_zero_traps_after_successful_build
cargo test -p ts2wasm-cli bigint_runtime_rem_zero_traps_after_successful_build
```

Current evidence proves Node reports `RangeError: Division by zero`; iwasm emits
the matching diagnostic text when uncaught and matches Node output for supported
`try/catch` fixtures.

## Desired final state

BigInt division/remainder by zero throw a compatible `RangeError` object matching Node behavior.

## Scope

In scope:

- [x] Add the interim runtime diagnostic/abort surface for `RangeError: Division by zero`.
- [x] After issue 396 lands, use its runtime exception substrate for a catchable `RangeError` object.
- [x] Convert BigInt division-by-zero trap to compatible `RangeError` throwing for supported helper paths.
- [x] Convert BigInt remainder-by-zero trap to compatible `RangeError` throwing for supported helper paths.
- [x] Add Node/iwasm differential or exception-parity fixture for `BigInt / 0n` as `RangeError: Division by zero`.
- [x] Add Node/iwasm differential or exception-parity fixture for `BigInt % 0n` as `RangeError: Division by zero`.
- [x] Update docs/current-state/issues with the exception boundary.

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

- [x] Node/iwasm differential or explicit exception-parity fixture covers BigInt `/ 0n` as `RangeError: Division by zero`.
- [x] Node/iwasm differential or explicit exception-parity fixture covers BigInt `% 0n` as `RangeError: Division by zero`.
- [x] Existing issue-260 signed-i64 arithmetic success fixtures continue to pass.
- [x] Docs/current-state/issues state the division/remainder by zero exception boundary.

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

- [x] updated: `docs/14-runtime-abi.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- none

## Notes

This is a focused split from issue 370, covering only division/remainder by zero `RangeError` throwing.

## Completion evidence

Completed 2026-05-01.

Commits:

- `5f660298` issue-396: add catchable runtime bigint errors
- current close slice: added `% 0n` catch fixture and issue close evidence

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test m2_node_diff rangeerror -- --nocapture: pass (4 passed)
cargo test -p ts2wasm-cli --test m2_node_diff bigint_mixed_runtime_typeerror_catch -- --nocapture: pass (1 passed; issue-381 sibling smoke)
mise run update-issue-index -- --check && mise run check issues: pass
```

Remaining risks:

- Full ECMAScript completion-record unwinding beyond the issue-396 supported statement-boundary helper path remains out of scope.
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

- Historical note: the issue was temporarily reclassified as blocked on issue
  396 while only diagnostic/abort behavior existed.
- Superseded by the 2026-05-01 completion evidence above: issue 396 now
  provides the runtime exception substrate, and `/ 0n` plus `% 0n` have
  supported `try/catch` RangeError-like message parity coverage.
