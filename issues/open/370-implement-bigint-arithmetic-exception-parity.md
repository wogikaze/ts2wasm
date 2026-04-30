---
id: 370
title: "Implement BigInt arithmetic RangeError and TypeError parity"
type: feature
area: runtime/semantics
class: design-ready
priority: P2
depends_on: [260]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement compatible JavaScript exception behavior for BigInt arithmetic errors that issue 260 intentionally left as diagnostics or runtime traps.

Problem: division/remainder by zero currently reaches a runtime `unreachable` trap in the signed-i64 helper slice, and mixed Number/BigInt arithmetic reports issue-linked diagnostics instead of throwing compatible `RangeError` / `TypeError` objects on runtime paths.

## Problem

Issue 260 proved the arithmetic slice without silent coercion, but it did not add broad JS exception throwing machinery. Node throws `RangeError: Division by zero` for BigInt `/` and `%` by zero, and throws `TypeError` for mixed Number/BigInt arithmetic where the expression reaches runtime.

Problem: BigInt arithmetic exception paths are not compatible JS throws yet.

## Current failure

Representative current evidence:

```sh
cargo test -p ts2wasm-cli bigint_runtime_div_zero_traps_after_successful_build
cargo test -p ts2wasm-cli bigint_runtime_rem_zero_traps_after_successful_build
cargo test -p ts2wasm-cli bigint_runtime_branch_large_assignment_reports_issue_370
cargo test -p ts2wasm-cli bigint_runtime_mixed_add_reports_issue_370_with_span
cargo test -p ts2wasm-cli bigint_mixed_arithmetic_reports_issue_370
```

The zero-division fixtures prove Node reports `RangeError: Division by zero`, while iwasm currently traps. The mixed arithmetic fixtures prove the compiler does not silently compile mixed Number/BigInt arithmetic as number arithmetic.

## Desired final state

BigInt arithmetic error paths match Node-observable exception behavior for the supported runtime subset:

- `/` and `%` by zero throw a compatible `RangeError` object.
- mixed Number/BigInt arithmetic throws a compatible `TypeError` object when it reaches runtime.
- compile-time diagnostics remain only where this compiler intentionally rejects unsupported source before lowering, with issue ownership kept precise.

## Scope

In scope:

- [ ] Define the minimal runtime throw path needed by BigInt arithmetic, or depend on an existing compatible throw mechanism if available.
- [ ] Convert BigInt division/remainder-by-zero traps to compatible `RangeError` throwing for supported helper paths.
- [ ] Convert dynamic mixed Number/BigInt arithmetic diagnostics to compatible runtime `TypeError` throwing where operands can reach runtime safely.
- [ ] Preserve source-backed diagnostics for unsupported shapes until a runtime throw path exists for them.
- [ ] Add Node/iwasm differential or exception-parity fixtures for `RangeError` and `TypeError` paths.
- [ ] Update docs/current-state/issues with the exception boundary.

Out of scope:

- Full multi-limb arithmetic; issue 369.
- BigInt bitwise/exponentiation policy; issue 371.
- General catch/finally exception semantics unless required for a minimal throw-compatible BigInt arithmetic test.
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

- [ ] Node/iwasm differential or explicit exception-parity fixture covers BigInt `/ 0n` and `% 0n` as `RangeError: Division by zero`.
- [ ] Node/iwasm differential or explicit exception-parity fixture covers mixed Number/BigInt arithmetic as `TypeError` for the supported runtime path.
- [ ] No mixed Number/BigInt arithmetic path silently lowers to ordinary number arithmetic.
- [ ] Existing issue-260 signed-i64 arithmetic success fixtures continue to pass.
- [ ] Docs/current-state/issues state which BigInt arithmetic exception paths are compatible and which remain diagnostics.

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

If generic exception machinery is still insufficient, record the exact missing primitive and keep this issue blocked/progress rather than reintroducing traps as a claimed compatible path.

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
