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

Problem: the focused ABC451 depth-8 gate still times out, and array growth is now the highest measured implementation target.

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
