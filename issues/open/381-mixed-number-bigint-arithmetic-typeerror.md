---
id: 381
title: "Mixed Number/BigInt arithmetic TypeError"
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

Implement compatible JavaScript `TypeError` throwing for mixed Number/BigInt arithmetic at runtime.

Problem: Mixed Number/BigInt arithmetic currently reports issue-linked diagnostics instead of throwing compatible `TypeError` objects when the expression reaches runtime.

## Problem

Issue 260 proved the arithmetic slice without silent coercion, but it did not add broad JS exception throwing machinery. Node throws `TypeError` for mixed Number/BigInt arithmetic where the expression reaches runtime.

Problem: Mixed Number/BigInt arithmetic error paths are not compatible JS throws yet.

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

- [ ] Implement runtime TypeError throwing for mixed Number/BigInt arithmetic using an existing compatible throw mechanism or minimal new throw path.
- [ ] Convert dynamic mixed Number/BigInt arithmetic diagnostics to compatible runtime TypeError throwing where operands can reach runtime safely.
- [ ] Preserve source-backed diagnostics for unsupported shapes until a runtime throw path exists for them.
- [ ] Add Node/iwasm differential or exception-parity fixture for mixed Number/BigInt arithmetic as TypeError.
- [ ] Update docs/current-state/issues with the exception boundary.

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

- [ ] Node/iwasm differential or explicit exception-parity fixture covers mixed Number/BigInt arithmetic as `TypeError` for the supported runtime path.
- [ ] No mixed Number/BigInt arithmetic path silently lowers to ordinary number arithmetic.
- [ ] Existing issue-260 signed-i64 arithmetic success fixtures continue to pass.
- [ ] Docs/current-state/issues state the mixed arithmetic exception boundary.

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

This is a focused split from issue 370, covering only mixed Number/BigInt arithmetic `TypeError` throwing.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```
