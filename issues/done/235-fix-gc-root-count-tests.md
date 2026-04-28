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
completed: 2026-04-28
status: done
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

- [x] Inspect the emitted WAT for both failing fixtures/tests.
- [x] Determine whether the implementation root count or the test expectation is wrong.
- [x] Fix implementation or expectations with evidence from the GC-root contract.
- [x] Keep root mirroring coverage meaningful; do not delete or weaken the tests.

Out of scope:

- Broad GC redesign.
- Unrelated runtime or frontend changes.

## Affected paths

Expected:

- `crates/backend-wasm/src/`

Do not touch:

- `crates/frontend/src/`
- `crates/ir/src/`
- `docs/`

## Acceptance criteria

- [x] Both named backend tests pass.
- [x] The fix preserves explicit assertions for top-level and function-local root mirroring.
- [x] Any changed expected counts are justified by emitted WAT evidence.
- [x] No regression in backend-wasm tests.

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

- [x] not affected

Current state:

- [x] update `current-state.md` only if full `cargo nextest run` remains red after this fix

Follow-up issues:

- [x] none

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `0090300` updates the backend GC root tests to derive expected counts and
  byte offsets from `LocalFrame`/`Layout`.

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run -p ts2wasm-backend-wasm function_locals_are_mirrored_into_activation_gc_root_frames top_level_locals_are_mirrored_into_gc_root_table
result: PASS (2 passed)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-backend-wasm
result: PASS (15 passed)
date: 2026-04-28

command: cargo nextest run
result: PASS (339 passed, 4 skipped)
date: 2026-04-28
```

Root-count evidence:

- Top-level fixture: one user local plus 12 backend temporaries yields
  `$gc_root_count = 13` and a 52-byte static root table allocation.
- Function fixture: `_start` has 12 backend static roots, so
  `$gc_root_count = 12`, `$gc_call_frame_base` starts at byte offset 48, and
  the reserved call-frame root stack makes the root allocation 16432 bytes.
- Function activation frame mirrors one function local plus 12 backend
  temporaries after the two-word frame header, so the frame advances by 60
  bytes and the test still asserts concrete mirrored slots.

Remaining risks:

- none
