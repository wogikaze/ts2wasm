---
id: 365
title: "Reduce ABC451 array-growth allocation and copy pressure"
type: bug
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: [364]
blocks: [363, 357, 309]
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Reduce the dominant ABC451 depth-8 allocation/copy category identified by issue 364: array growth.

## Problem

Issue 364 added callsite attribution to `mise run abc451-runtime-costs`. The diagnostic shows array growth dominates the remaining allocation/copy bytes after the free-list, GC cadence, top-of-heap grow, and bulk-copy work.

100000-event evidence:

```json
{
  "alloc_array_growth_calls": 2648,
  "alloc_array_growth_bytes": 362976,
  "copy_array_growth_calls": 2648,
  "copy_array_growth_bytes": 181008,
  "allocation_requested_bytes": 521193,
  "all_copy_bytes": 250278,
  "gc_collections": 5,
  "sweep_visits": 58859
}
```

300000-event evidence:

```json
{
  "alloc_array_growth_calls": 3771,
  "alloc_array_growth_bytes": 1158708,
  "copy_array_growth_calls": 3770,
  "copy_array_growth_bytes": 856928,
  "allocation_requested_bytes": 1376350,
  "all_copy_bytes": 955420,
  "gc_collections": 13,
  "sweep_visits": 241504
}
```

Problem: the focused ABC451 depth-8 gate still times out, and array growth is now the highest measured implementation target. Issue 366 narrowed the dominant `ArrayPushGrow` miss reason to non-top arrays, not committed-memory misses.

## Desired final state

The runtime reduces array-growth allocation/copy pressure enough to either make the focused ABC451 depth-8 `iwasm` gate pass or produce a smaller quantified blocker.

## Scope

In scope:

- [ ] Reduce `alloc_array_growth_*` or `copy_array_growth_*` counters using a general runtime-memory improvement.
- [ ] Preserve the committed `MEMORY_MAX_PAGES=185` policy.
- [ ] Preserve explicit OOM failure behavior.
- [ ] Record before/after diagnostics with `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30`.
- [ ] Run the focused ABC451 depth-8 gate and record pass/fail.

Out of scope:

- Raising memory caps or test timeouts.
- Skipping, ignoring, or weakening the ABC451 test.
- Source rewriting the ABC451 fixture.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Reverting issues 359, 360, 361, or 362 without measured proof that the runtime policy is incorrect.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `scripts/run/abc451-runtime-costs.py` only if attribution categories need a compatible update
- `issues/open/365-reduce-abc451-array-growth-allocation-copy-pressure.md`
- `issues/open/363-reduce-abc451-allocation-and-sweep-volume-after-bulk-copy-narrowing.md`
- `issues/open/357-fix-abc451-depth8-iwasm-timeout.md` only if closure is verified
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures

## Acceptance criteria

- [ ] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and records before/after attribution.
- [ ] `alloc_array_growth_bytes` or `copy_array_growth_bytes` decreases, or the issue records evidence that the next blocker is smaller than array growth.
- [ ] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` passes, or records the remaining timeout/trap with diagnostics.
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
mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30
cargo nextest run
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected unless allocator or array-growth policy changes

Current state:

- [x] update only if runtime facts change

Follow-up issues:

- [x] create only if this slice isolates a smaller remaining blocker

## Notes

Issue 364 reported zero unattributed allocation and copy counts at 100000 and 300000 diagnostic events, so this issue should start from the array-growth callsites before trying broader allocator policy changes.

## Parent blocker classification

2026-04-30 parent classification:

- Class changed to `blocked` after two child attempts failed to find a safe mergeable array-growth reduction.
- Follow-up issue 366 tracks `ArrayPushGrow` hit/miss attribution needed before more implementation probes.

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

## Parent review of child attempt: 2026-04-30

Status: `REJECTED_FOR_MERGE`.

The child commit reduced the targeted `array_growth` counters by giving empty array literals a small spare backing store while committed memory had headroom, but the parent rejected the production runtime change because total allocation pressure worsened and late free-list scans reappeared.

Useful evidence from the rejected attempt:

```text
100000 events before: alloc_array_growth=362976 bytes/2648 calls; copy_array_growth=181008 bytes/2648 calls; allocation_requested_bytes=521193; sweep_visits=58859; free_list_scan_visits=0
100000 events after:  alloc_array_growth=326920 bytes/1096 calls; copy_array_growth=173552 bytes/1096 calls; allocation_requested_bytes=609546; sweep_visits=64303; free_list_scan_visits=0
```

```text
300000 events before: alloc_array_growth=1158708 bytes/3771 calls; copy_array_growth=856928 bytes/3770 calls; allocation_requested_bytes=1376350; sweep_visits=241504; free_list_scan_visits=0
300000 events after:  alloc_array_growth=1082272 bytes/1688 calls; copy_array_growth=808092 bytes/1687 calls; allocation_requested_bytes=1503147; sweep_visits=231032; free_list_scan_visits=14657
```

Parent decision:

```text
Do not merge the empty-array spare backing-store runtime change as-is. A valid follow-up must reduce array-growth pressure without increasing total allocation pressure or reintroducing free-list scans at the diagnostic budgets.
```

## Child v2 blocker evidence: 2026-05-01

Status: `BLOCKED`.

This child did not keep runtime code changes because the tested candidates were neutral at the required 100000-event diagnostic budget and therefore did not satisfy the parent merge constraint.

Baseline reproduced:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; alloc_array_growth_bytes=362976; alloc_array_growth_calls=2648; copy_array_growth_bytes=181008; copy_array_growth_calls=2648; allocation_requested_bytes=521193; sweep_visits=58859; free_list_scan_visits=0
```

Rejected candidates:

```text
candidate: pre-reserve hidden top-of-heap ArrayPushGrow capacity before evaluating the pushed value, while delaying length/element mutation until after value evaluation
command: cargo fmt --all --check && mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; neutral counters versus baseline; alloc_array_growth_bytes=362976; copy_array_growth_bytes=181008; allocation_requested_bytes=521193; sweep_visits=58859; free_list_scan_visits=0
decision: not kept because it produced no measurable pressure reduction
```

```text
candidate: raise ARRAY_PUSH_GROW_LINEAR_GROWTH_THRESHOLD from 3072 to 4096
command: cargo fmt --all --check && mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; neutral counters versus baseline; alloc_array_growth_bytes=362976; copy_array_growth_bytes=181008; allocation_requested_bytes=521193; sweep_visits=58859; free_list_scan_visits=0
decision: not kept because it produced no measurable pressure reduction
```

Remaining blocker:

```text
The 100000-event array-growth pressure is below the 3072 linear-growth threshold, and the current top-of-heap extension is not reached for the dominant non-top result arrays. The next useful slice should either add attribution for ArrayPushGrow top-of-heap miss reasons or implement a representation-level append strategy that reduces non-top array copying without increasing aggregate requested allocation or sweep volume.
```

Validation:

```text
cargo fmt --all --check: pass
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30: pass; baseline counters reproduced
mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30: not run because no 100000-event candidate was promising
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm: fail; iwasm timed out after 30.211s
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm: pass
cargo test -p ts2wasm-backend-wasm --lib -- --nocapture: pass; 27 passed
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

## Issue 366 ArrayPushGrow miss attribution: 2026-05-01

Status: `READY_FOR_IMPLEMENTATION`.

Issue 366 added default-off `ArrayPushGrow` hit/miss attribution to `mise run abc451-runtime-costs`.

100000-event result:

```json
{
  "capacity_hits": 49087,
  "capacity_misses": 4160,
  "top_heap_hits": 1512,
  "top_heap_misses": 2648,
  "top_miss_reason": "non_top_heap",
  "miss_reasons": [
    {"reason": "non_top_heap", "calls": 2648},
    {"reason": "committed_memory", "calls": 0}
  ],
  "fallback_allocation": {"calls": 2648, "bytes": 362976},
  "fallback_copy": {"calls": 2648, "bytes": 181008}
}
```

300000-event result:

```json
{
  "capacity_hits": 67439,
  "capacity_misses": 7880,
  "top_heap_hits": 4109,
  "top_heap_misses": 3771,
  "top_miss_reason": "non_top_heap",
  "miss_reasons": [
    {"reason": "non_top_heap", "calls": 3771},
    {"reason": "committed_memory", "calls": 0}
  ],
  "fallback_allocation": {"calls": 3771, "bytes": 1158708},
  "fallback_copy": {"calls": 3770, "bytes": 856928}
}
```

Implementation target:

```text
Reduce non-top array-growth allocation/copy pressure. Do not spend the next slice on committed-memory misses, because committed-memory miss count is 0 at both diagnostic budgets.
```
