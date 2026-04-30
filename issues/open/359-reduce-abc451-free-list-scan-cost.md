---
id: 359
title: "Reduce ABC451 free-list scan cost"
type: bug
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: [358]
blocks: [357, 309]
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Reduce the allocator free-list scan cost that dominates the ABC451 depth-8 runtime diagnostic budget.

This is the next implementation slice after issue 358 added focused telemetry for the issue 357 `iwasm` timeout.

## Problem

Issue 357 remains blocked because `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` times out under `iwasm`.

Issue 358 added a diagnostic command and recorded this baseline at a 100000-event budget:

```json
{
  "array_copy_calls": 1584,
  "array_copy_bytes": 96408,
  "array_copy_elements": 24102,
  "all_copy_calls": 9509,
  "all_copy_bytes": 127068,
  "allocation_attempts": 9548,
  "allocation_requested_bytes": 285109,
  "gc_collections": 2,
  "sweep_visits": 13562,
  "free_list_scan_visits": 67379
}
```

Problem: the current allocator repeatedly scans the free list enough times to dominate the diagnostic budget, so the ABC451 fixture cannot complete within the `iwasm` timeout.

## Current failure

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Current result:

```text
fail; iwasm timed out around 30s
```

Diagnostic command:

```sh
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
```

Current result:

```text
pass; diagnostic stops before timeout; free_list_scan_visits=67379
```

## Desired final state

The allocator avoids pathological repeated free-list scans for ABC451 depth-8 while preserving the committed memory cap, GC semantics, and OOM behavior.

## Scope

In scope:

- [x] Reduce `free_list_scan_visits` for `fixtures/core-semantics/abc451-depth8-live-set.ts` using a general allocator/runtime improvement.
- [ ] Preserve the committed 185-page memory policy.
- [ ] Preserve explicit OOM failure behavior.
- [ ] Record before/after diagnostic evidence with `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30`.
- [ ] If the depth-8 fixture now completes, close issue 357 in the same merge or request parent close after verification.

Out of scope:

- Raising `MEMORY_MAX_PAGES`.
- Skipping, ignoring, or weakening the ABC451 test.
- Source rewriting the ABC451 fixture.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Repeating issue 357 rejected probes unless paired with a new measured free-list hypothesis.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/` only if regression coverage needs a small assertion update
- `issues/open/359-reduce-abc451-free-list-scan-cost.md`
- `issues/open/357-fix-abc451-depth8-iwasm-timeout.md` only if the parent verifies closure
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures
- `scripts/run/abc451-runtime-costs.py` unless the diagnostic itself is wrong

## Acceptance criteria

- [ ] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and records lower `free_list_scan_visits` than the issue 358 baseline, or records a justified replacement metric if the allocator structure changes.
- [ ] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` passes, or the issue records quantified progress and the remaining blocker.
- [ ] `cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm` passes.
- [ ] `cargo test -p ts2wasm-backend-wasm --lib -- --nocapture` passes if backend runtime code changes.
- [ ] `cargo fmt --all --check` passes.
- [ ] `mise run update-issue-index -- --check` and `mise run check issues` pass.

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

- [x] not affected unless allocator policy changes

Current state:

- [x] update only if runtime facts change

Follow-up issues:

- [x] create only if this slice isolates a smaller remaining blocker

## Notes

Issue 358 baseline interpretation:

- `free_list_scan_visits=67379`
- `sweep_visits=13562`
- `array_copy_calls=1584`
- `gc_collections=2`

Use the diagnostic to avoid repeating broad timeout probes without attribution.

2026-04-30 child progress evidence:

- Implemented a general bump-first allocator policy in `$alloc_heap`: after GC
  and heap cursor recomputation, the allocator now scans swept free blocks only
  when the bump allocation would exceed currently committed memory. This keeps
  the 185-page cap and max-cap/OOM fallback path intact while avoiding early
  repeated free-list scans.
- Before diagnostic from issue 358 baseline:

```json
{
  "array_copy_calls": 1584,
  "array_copy_bytes": 96408,
  "array_copy_elements": 24102,
  "all_copy_calls": 9509,
  "all_copy_bytes": 127068,
  "allocation_attempts": 9548,
  "allocation_requested_bytes": 285109,
  "gc_collections": 2,
  "sweep_visits": 13562,
  "free_list_scan_visits": 67379
}
```

- After diagnostic:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; diagnostic stopped before timeout
free_list_scan_visits: 67379 -> 0
gc_collections: 2 -> 7
sweep_visits: 13562 -> 68498
array_copy_calls: 1584 -> 2614
all_copy_calls: 9509 -> 15728
```

- Remaining blocker: issue 357 is not closed. The focused depth-8 `iwasm`
  fixture still times out at the test limit after free-list scans are removed,
  so the next slice should target sweep frequency/live-set retention or array
  copy pressure rather than free-list scan cost.
- Rejected unsafe probe: a preferred free-block cache reduced some allocation
  and sweep counters but was not kept because the focused depth-8 run failed
  immediately with `Exception: wasm operand stack overflow`.

Validation run for this progress:

```text
command: cargo fmt --all --check
result: pass

command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: pass; 27 passed

command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; free_list_scan_visits=0

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass; 1 passed

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.233s
```
