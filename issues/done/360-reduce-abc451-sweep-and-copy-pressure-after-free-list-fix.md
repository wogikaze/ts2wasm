---
id: 360
title: "Reduce ABC451 sweep and copy pressure after free-list fix"
type: bug
area: runtime/memory
class: done
priority: P1
depends_on: [359]
blocks: [357, 309]
created: 2026-04-30
updated: 2026-04-30
status: done
completed: 2026-04-30
---

## Summary

Reduce the remaining ABC451 depth-8 timeout pressure after issue 359 eliminated allocator free-list scans.

## Problem

Issue 359 changed allocation to avoid early free-list scans and parent verification reproduced:

```json
{
  "free_list_scan_visits": 0,
  "gc_collections": 7,
  "sweep_visits": 68498,
  "array_copy_calls": 2614,
  "array_copy_bytes": 162760,
  "all_copy_calls": 15728,
  "all_copy_bytes": 213487,
  "allocation_attempts": 15767,
  "allocation_requested_bytes": 468361
}
```

Problem: `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` still times out around 30 seconds after free-list scan removal. The remaining measured pressure is sweep frequency/live-set retention and copy work.

## Current failure

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Current result after issue 359:

```text
fail; iwasm timed out around 30s
```

Diagnostic command:

```sh
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
```

Current result after issue 359:

```text
pass; diagnostic stops before timeout; free_list_scan_visits=0; gc_collections=7; sweep_visits=68498; array_copy_calls=2614
```

## Desired final state

The depth-8 ABC451 live-set fixture completes under `iwasm` within the test timeout and prints Node-matching output, without weakening memory policy or hiding the test.

## Scope

In scope:

- [x] Reduce sweep frequency, sweep visits, live-set retention cost, or array/copy pressure using a general runtime-memory improvement.
- [x] Preserve the committed 185-page memory policy.
- [x] Preserve explicit OOM failure behavior.
- [x] Record before/after diagnostic evidence with `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30`.
- [x] Depth-8 did not complete; issue 357 remains open and issue 361 tracks the remaining blocker.

Out of scope:

- Raising `MEMORY_MAX_PAGES`.
- Skipping, ignoring, or weakening the ABC451 test.
- Source rewriting the ABC451 fixture.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Free-list scan optimization already completed by issue 359 unless the issue 359 change is proven incorrect.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/` only if regression coverage needs a small assertion update
- `issues/done/360-reduce-abc451-sweep-and-copy-pressure-after-free-list-fix.md`
- `issues/open/357-fix-abc451-depth8-iwasm-timeout.md` only if closure is verified
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures
- `scripts/run/abc451-runtime-costs.py` unless the diagnostic itself is wrong

## Acceptance criteria

- [x] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and records reduced remaining pressure versus the issue 359 parent-verified baseline, or records a justified replacement metric if the runtime structure changes.
- [x] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` passes, or the issue records quantified progress and the remaining blocker.
- [x] `cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm` passes.
- [x] `cargo test -p ts2wasm-backend-wasm --lib -- --nocapture` passes if backend runtime code changes.
- [x] `cargo fmt --all --check` passes.
- [x] `mise run update-issue-index -- --check` and `mise run check issues` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected unless allocator or GC policy changes

Current state:

- [x] update only if runtime facts change

Follow-up issues:

- [x] created issue 361 for remaining array/copy pressure

## Notes

Start from the issue 359 parent-verified baseline. Avoid broad timeout probes without diagnostic attribution.

2026-05-01 child progress evidence:

- Implemented a general GC cadence slice in `$alloc_heap`: allocation-pressure GC now uses `Layout::GC_THRESHOLD * 2`, while preserving the 185-page `MEMORY_MAX_PAGES` cap, memory-headroom check, max-cap last-chance GC, and explicit OOM guard.
- Before diagnostic from the issue 359 parent-verified baseline:

```json
{
  "free_list_scan_visits": 0,
  "gc_collections": 7,
  "sweep_visits": 68498,
  "array_copy_calls": 2614,
  "array_copy_bytes": 162760,
  "all_copy_calls": 15728,
  "all_copy_bytes": 213487,
  "allocation_attempts": 15767,
  "allocation_requested_bytes": 468361
}
```

- After diagnostic:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; diagnostic stopped before timeout
free_list_scan_visits: 0 -> 0
gc_collections: 7 -> 6
sweep_visits: 68498 -> 62313
array_copy_calls: 2614 -> 3119
all_copy_calls: 15728 -> 18821
```

- Interpretation: the fixed event budget now reaches more allocation/copy events before diagnostic abort, but sweep frequency and total sweep visits drop while free-list scans remain eliminated. Issue 357 remains open because the focused depth-8 `iwasm` fixture still times out at the test limit.
- Rejected probe in this slice: replacing `$copy` with `memory.copy` was not kept because `scripts/run/abc451-runtime-costs.py` intentionally instruments the existing byte-loop `$copy` shape and rejected the changed diagnostic target. A bounded post-threshold array growth chunk was also not kept because it changed the focused failure from timeout to `Exception: unreachable`.

Validation run for this progress:

```text
command: cargo fmt --all --check
result: pass

command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; gc_collections=6; sweep_visits=62313; free_list_scan_visits=0

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.211s

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass; 1 passed

command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: pass; 27 passed
```

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

## Parent close evidence

Parent verified and closed this focused issue after integrating child progress.

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; free_list_scan_visits=0; gc_collections=6; sweep_visits=62313; array_copy_calls=3119; timed_out=false

action: created issue 361 for remaining array/copy pressure because issue 357 still times out.
```
