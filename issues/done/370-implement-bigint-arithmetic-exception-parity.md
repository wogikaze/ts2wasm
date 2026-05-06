---
id: 370
title: "Implement BigInt arithmetic RangeError and TypeError parity"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [260, 380, 381]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
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

- [x] Define the minimal runtime throw path needed by BigInt arithmetic, or depend on an existing compatible throw mechanism if available.
- [x] Convert BigInt division/remainder-by-zero traps to compatible `RangeError` throwing for supported helper paths.
- [x] Convert dynamic mixed Number/BigInt arithmetic diagnostics to compatible runtime `TypeError` throwing where operands can reach runtime safely.
- [x] Preserve source-backed diagnostics for unsupported shapes until a runtime throw path exists for them.
- [x] Add Node/iwasm differential or exception-parity fixtures for `RangeError` and `TypeError` paths.
- [x] Update docs/current-state/issues with the exception boundary.

Out of scope:

- Full multi-limb arithmetic; issue 369.
- BigInt bitwise/exponentiation policy; issue 371.
- General catch/finally exception semantics unless required for a minimal throw-compatible BigInt arithmetic test.
- BigInt builtin string exception parity; issue 333.

Split from this issue:
- Issue 380: BigInt division/remainder by zero RangeError (focused on `/ 0n` and `% 0n`)
- Issue 381: Mixed Number/BigInt arithmetic TypeError (focused on runtime TypeError throwing)

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

- [x] Node/iwasm differential or explicit exception-parity fixture covers BigInt `/ 0n` and `% 0n` as `RangeError: Division by zero`.
- [x] Node/iwasm differential or explicit exception-parity fixture covers mixed Number/BigInt arithmetic as `TypeError` for the supported runtime path.
- [x] No mixed Number/BigInt arithmetic path silently lowers to ordinary number arithmetic.
- [x] Existing issue-260 signed-i64 arithmetic success fixtures continue to pass.
- [x] Docs/current-state/issues state which BigInt arithmetic exception paths are compatible and which remain diagnostics.

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

- [x] updated: `docs/14-runtime-abi.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

If generic exception machinery is still insufficient, record the exact missing primitive and keep this issue blocked/progress rather than reintroducing traps as a claimed compatible path.

## Completion evidence

Date: 2026-05-06

Commits:

- child issues: `380`, `381`

Validation result:

```text
Parent reclosed from child evidence. `issues/done/380-*` covers BigInt
division/remainder by zero `RangeError`, and `issues/done/381-*` covers mixed
Number/BigInt arithmetic `TypeError` parity.
```

Remaining risks:

- This may need a small cross-cutting runtime exception primitive before BigInt-specific helpers can throw compatibly.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

