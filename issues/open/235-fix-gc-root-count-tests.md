---
id: 235
title: "Fix GC root count backend tests"
type: bug
area: backend/memory
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Restore the backend GC-root unit tests that currently fail on `master`.

## Problem

`cargo nextest run` stops early because two backend GC-root tests assert stale root counts:

- `ts2wasm-backend-wasm tests::function_locals_are_mirrored_into_activation_gc_root_frames`
- `ts2wasm-backend-wasm tests::top_level_locals_are_mirrored_into_gc_root_table`

The failures reproduce on parent `master` after issue 231 was merged:

```text
cargo nextest run -p ts2wasm-backend-wasm function_locals_are_mirrored_into_activation_gc_root_frames top_level_locals_are_mirrored_into_gc_root_table
result: fail
function_locals_are_mirrored_into_activation_gc_root_frames: expected WAT to contain `(global.set $gc_root_count (i32.const 3))`
top_level_locals_are_mirrored_into_gc_root_table: expected WAT to contain `(global.set $gc_root_count (i32.const 4))`
```

## Desired final state

The tests assert the correct GC root contract and the full workspace test suite can progress past these backend failures.

## Scope

In scope:

- [ ] Inspect the emitted WAT for both failing fixtures/tests.
- [ ] Determine whether the implementation root count or the test expectation is wrong.
- [ ] Fix implementation or expectations with evidence from the GC-root contract.
- [ ] Keep root mirroring coverage meaningful; do not delete or weaken the tests.

Out of scope:

- [ ] Broad GC redesign.
- [ ] Unrelated runtime or frontend changes.

## Affected paths

Expected:

- `crates/backend-wasm/src/`

Do not touch:

- `crates/frontend/src/`
- `crates/ir/src/`
- `docs/`

## Acceptance criteria

- [ ] Both named backend tests pass.
- [ ] The fix preserves explicit assertions for top-level and function-local root mirroring.
- [ ] Any changed expected counts are justified by emitted WAT evidence.
- [ ] No regression in backend-wasm tests.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-backend-wasm function_locals_are_mirrored_into_activation_gc_root_frames top_level_locals_are_mirrored_into_gc_root_table
cargo nextest run -p ts2wasm-backend-wasm
```

Impacted commands:

```sh
cargo nextest run
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] update `current-state.md` only if full `cargo nextest run` remains red after this fix

Follow-up issues:

- [ ] none

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
