---
id: 356
title: "Fix array-push growth WAT format compile blocker"
type: bug
area: backend
class: done
priority: P1
depends_on: []
blocks: [280]
created: 2026-04-30
updated: 2026-04-30
completed: 2026-04-30
---

## Summary

Fix the backend-wasm compile blocker in the array-push growth WAT emission path.

This is a narrow gate-unblocker issue created from child-280 verification evidence.

## Problem

The required close gate for issue 280 cannot compile the workspace because `crates/backend-wasm/src/expr_emit.rs` contains an invalid WAT format string in the array-push growth path.

Problem: `cargo nextest run -E 'test(bigint) or test(node_diff)'` stops before executing target tests due to `expr_emit.rs` array-push growth formatting errors.

## Current failure

From child-280 validation on 2026-04-30:

```sh
cargo nextest run -E 'test(bigint) or test(node_diff)'
```

Current result:

```text
error: argument never used: Layout::ARRAY_ELEM_SHIFT
error: cannot find value `array_push_grow_linear_growth_threshold` in this scope
```

The errors are reported in `crates/backend-wasm/src/expr_emit.rs` around the array-push growth WAT format string.

## Desired final state

The backend-wasm crate compiles, the array-push growth WAT formatter uses valid arguments, and issue-280's broad verification command can reach its actual tests.

## Scope

In scope:

- [x] Fix the invalid format string or missing placeholder in the array-push growth WAT emission path.
- [x] Preserve existing Array.prototype.push growth behavior.
- [x] Add or update a narrow regression test only if the existing array-push coverage does not exercise this path.

Out of scope:

- Changing BigInt semantics.
- Changing Array.prototype.map semantics.
- Changing GC policy or ABC451 memory behavior.
- Broad runtime refactors.

## Affected paths

Expected:

- `crates/backend-wasm/src/expr_emit.rs`
- `crates/backend-wasm/src/` if a helper signature needs a local adjustment
- `crates/cli/tests/` only for a narrow array-push regression if needed
- `fixtures/core-semantics/` only for a narrow array-push regression if needed
- `issues/done/356-fix-array-push-growth-wat-format-compile-blocker.md`
- `issues/index.md`

Do not touch:

- BigInt files
- Array.prototype.map implementation
- parser/frontend files
- GC/ABC451 memory policy files

## Acceptance criteria

- [x] `cargo check -p ts2wasm-backend-wasm` passes.
- [x] `cargo nextest run -E 'test(bigint) or test(node_diff)'` no longer fails on this `expr_emit.rs` compile blocker.
- [x] Existing array-push coverage remains valid; add a regression only if no existing test covers the fixed path.
- [x] Issue 280 remains open unless its own acceptance criteria are separately verified.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo check -p ts2wasm-backend-wasm
cargo nextest run -E 'test(bigint) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli array_push
cargo test -p ts2wasm-backend-wasm --lib
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Keep this issue as a compile-gate fix. Do not broaden into array-push behavior changes unless the existing tests prove behavior changed.

## Completion evidence

Completed 2026-04-30.

Commits:

- `48ed36939df8d230d50e660e11926431c5c2d125` issue-356: fix array push growth wat template

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-30

command: cargo check -p ts2wasm-backend-wasm
result: pass
date: 2026-04-30

command: cargo test -p ts2wasm-backend-wasm --lib
result: pass, 27 passed
date: 2026-04-30

command: cargo nextest run -p ts2wasm-cli array_push
result: pass, 6 passed
date: 2026-04-30

command: cargo nextest run -E 'test(bigint) or test(node_diff)'
result: fail, but no longer on expr_emit.rs compile or WAT parse blocker; reached selected tests and failed on unrelated abc451_depth8_live_set_fixture_matches_node_output_under_iwasm iwasm timeout after 165 passed
date: 2026-04-30

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-30

command: mise run check issues
result: pass
date: 2026-04-30
```

Remaining risks:

- none
