---
id: 381
title: "Mixed Number/BigInt arithmetic TypeError"
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

Implement compatible JavaScript `TypeError` throwing for mixed Number/BigInt arithmetic at runtime.

Problem: Mixed Number/BigInt arithmetic currently reports issue-linked diagnostics instead of throwing compatible `TypeError` objects when the expression reaches runtime.

## Problem

Issue 260 proved the arithmetic slice without silent coercion, but it did not add broad JS exception throwing machinery. Node throws `TypeError` for mixed Number/BigInt arithmetic where the expression reaches runtime.

Problem: Mixed Number/BigInt arithmetic now has runtime helper coverage for the
current literal/known-local slice. It emits a TypeError diagnostic when uncaught
and raises a catchable TypeError-like object with Node-compatible `message`
parity when a supported `try/catch` is active.

## Current failure

Representative current evidence:

```sh
cargo test -p ts2wasm-cli bigint_runtime_branch_large_assignment_reports_issue_370
```

Result:

```text
error: issue-370: mixed Number/BigInt arithmetic not yet supported
```

```sh
cargo test -p ts2wasm-cli bigint_runtime_mixed_add_reports_issue_370_with_span
```

Result:

```text
error: issue-370: mixed Number/BigInt arithmetic not yet supported
```

```sh
cargo test -p ts2wasm-cli bigint_mixed_arithmetic_reports_issue_370
```

Result:

```text
error: issue-370: mixed Number/BigInt arithmetic not yet supported
```

The mixed arithmetic fixtures prove the compiler does not silently compile mixed Number/BigInt arithmetic as number arithmetic, but it reports diagnostics instead of throwing `TypeError` at runtime.

## Desired final state

Mixed Number/BigInt arithmetic throws a compatible `TypeError` object when it reaches runtime, matching Node behavior.

## Scope

In scope:

- [x] Implement runtime TypeError throwing for mixed Number/BigInt arithmetic using an existing compatible throw mechanism or minimal new throw path.
- [x] Convert dynamic mixed Number/BigInt arithmetic diagnostics to compatible runtime TypeError throwing where operands can reach runtime safely.
- [x] Preserve source-backed diagnostics for unsupported shapes until a runtime throw path exists for them.
- [x] Add Node/iwasm differential or exception-parity fixture for mixed Number/BigInt arithmetic as TypeError.
- [x] Update docs/current-state/issues with the exception boundary.

Out of scope:

- BigInt division/remainder by zero RangeError; issue 380.
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

- [x] Node/iwasm differential fixture covers mixed Number/BigInt arithmetic as TypeError with no silent lowering to number arithmetic, and existing issue-260 signed-i64 arithmetic success fixtures continue to pass.
- [x] Docs/current-state/issues state the mixed arithmetic exception boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli bigint_runtime_mixed_add_reports_issue_370_with_span
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

This is a focused split from issue 370, covering only mixed Number/BigInt arithmetic `TypeError` throwing.

## Progress evidence

2026-05-01 child-381 progress:

- Added a dedicated mixed Number/BigInt arithmetic runtime trap helper so current literal/known-local mixed `+` paths no longer silently lower to number arithmetic or fail at build time.
- Added Node `TypeError` baseline + iwasm trap coverage for `1n + 2` and `let a = 1n; a + 2`.
- Historical note: compatible JS `TypeError` object throwing was blocked until
  issue 396 landed. Superseded by the 2026-05-01 completion evidence above.
- Required validation for the progress commit is recorded in the child cycle report.

## Completion evidence

Completed 2026-05-01.

Commits:

- `5f660298` issue-396: add catchable runtime bigint errors
- current close slice: issue-381 close evidence

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test m2_node_diff bigint_mixed -- --nocapture: pass (11 passed)
cargo test -p ts2wasm-cli --test m2_node_diff bigint_mixed_runtime_typeerror_catch -- --nocapture: pass (1 passed)
mise run update-issue-index -- --check && mise run check issues: pass
```

Remaining risks:

- Source-backed unsupported mixed shapes still produce issue-linked diagnostics where runtime execution cannot be proven safe.
- Full ECMAScript completion-record unwinding beyond the issue-396 supported statement-boundary helper path remains out of scope.
