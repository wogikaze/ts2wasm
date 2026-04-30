---
id: 380
title: "BigInt division/remainder by zero RangeError"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [260]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement compatible JavaScript `RangeError` throwing for BigInt division and remainder by zero.

Problem: BigInt `/ 0n` and `% 0n` currently reach a runtime `unreachable` trap in the signed-i64 helper slice, while Node throws `RangeError: Division by zero`.

## Problem

Issue 260 proved the arithmetic slice without silent coercion, but it did not add JS exception throwing machinery. Node throws `RangeError: Division by zero` for BigInt `/` and `%` by zero, while iwasm currently traps.

Problem: BigInt division/remainder by zero error paths are not compatible JS throws yet.

## Current failure

Representative current evidence:

```sh
cargo test -p ts2wasm-cli bigint_runtime_div_zero_traps_after_successful_build
cargo test -p ts2wasm-cli bigint_runtime_rem_zero_traps_after_successful_build
```

The zero-division fixtures prove Node reports `RangeError: Division by zero`, while iwasm currently traps.

## Desired final state

BigInt division/remainder by zero throw a compatible `RangeError` object matching Node behavior.

## Scope

In scope:

- [ ] Define the minimal runtime throw path for `RangeError`, or depend on an existing compatible throw mechanism if available.
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

This is a focused split from issue 370, covering only division/remainder by zero `RangeError` throwing.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```

Remaining risks:

- This may need a small cross-cutting runtime exception primitive before BigInt-specific helpers can throw compatibly.
