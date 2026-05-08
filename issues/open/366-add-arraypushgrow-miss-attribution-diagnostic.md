---
id: 366
title: "Add ABC451 ArrayPushGrow miss attribution diagnostic"
type: test
area: runtime/performance
class: implementation-ready
priority: P1
depends_on: [365]
blocks: [365, 363, 357, 309]
created: 2026-04-30
updated: 2026-04-30
status: done
completed: 2026-05-01
---

## Summary

Add attribution for why `ArrayPushGrow` cannot use the existing top-of-heap extension path in the ABC451 depth-8 fixture.

## Problem

Issue 365 tried two narrower array-growth probes after issue 364 identified array growth as the dominant allocation/copy target. Both probes were neutral at the 100000-event diagnostic budget:

```text
pre-reserve hidden top-of-heap ArrayPushGrow capacity: no counter delta
ARRAY_PUSH_GROW_LINEAR_GROWTH_THRESHOLD 3072 -> 4096: no counter delta
```

Problem: the current attribution identifies array growth as dominant, but it does not explain why the optimized top-of-heap extension path misses for the dominant non-top result arrays.

## Current evidence

100000-event baseline:

```text
alloc_array_growth_bytes=362976
alloc_array_growth_calls=2648
copy_array_growth_bytes=181008
copy_array_growth_calls=2648
allocation_requested_bytes=521193
sweep_visits=58859
free_list_scan_visits=0
```

Issue 365 child v2 conclusion:

```text
The 100000-event array-growth pressure is below the 3072 linear-growth threshold, and the current top-of-heap extension is not reached for the dominant non-top result arrays.
```

## Desired final state

The diagnostic reports `ArrayPushGrow` fast-path hits and miss reasons, enough to decide whether to implement a safe representation-level append strategy or a smaller runtime-memory optimization.

## Scope

In scope:

- [x] Extend `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` to report `ArrayPushGrow` hit/miss attribution.
- [x] Extend `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` to report `ArrayPushGrow` hit/miss attribution.
- [x] Count at least top-of-heap hits, non-top misses, committed-memory misses, capacity/threshold path counts, and bytes/calls associated with misses if practical.
- [x] Keep instrumentation off by default for normal builds/tests.
- [x] Record the 100000-event baseline miss attribution.
- [x] Create or update a focused implementation issue if the miss attribution identifies a clear target.

Out of scope:

- Production runtime policy changes without miss-attribution evidence.
- Raising memory caps or timeouts.
- Skipping, ignoring, or weakening the ABC451 test.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Source rewriting the ABC451 fixture.

## Affected paths

Expected:

- `scripts/run/abc451-runtime-costs.py`
- `issues/open/366-add-arraypushgrow-miss-attribution-diagnostic.md`
- `issues/open/365-reduce-abc451-array-growth-allocation-copy-pressure.md`
- `issues/index.md`

Do not touch:

- production runtime code unless a tiny off-by-default diagnostic-compatible hook is unavoidable
- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures
- issues 359-364 unless parent explicitly reopens them

## Acceptance criteria

- [x] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and emits `ArrayPushGrow` hit/miss attribution beyond aggregate array-growth counters.
- [x] Attribution output identifies the dominant miss reason at the diagnostic budget.
- [x] Normal WAT/WASM output remains unchanged outside the diagnostic script.
- [x] `mise run check scripts` passes.
- [x] `mise run update-issue-index -- --check` and `mise run check issues` pass.
- [x] If a clear implementation target is found, create or update a focused follow-up issue and link it from issue 365.

## Validation

Required commands:

```sh
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
mise run check scripts
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected unless attribution changes known runtime facts

Follow-up issues:

- [x] create or update based on miss attribution evidence

## Notes

Issue 365 should not receive more broad array-growth implementation probes until this miss attribution exists or another concrete high-volume target is identified.

## Completion evidence

## ArrayPushGrow attribution baseline

100000-event baseline:

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
  "growth_capacity_paths": {
    "double_capacity": 3257,
    "linear_capacity": 903,
    "min_capacity": 1867,
    "required_capacity": 903
  },
  "fallback_allocation": {"calls": 2648, "bytes": 362976},
  "fallback_copy": {"calls": 2648, "bytes": 181008}
}
```

300000-event confirmation:

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
  "growth_capacity_paths": {
    "double_capacity": 4598,
    "linear_capacity": 3282,
    "min_capacity": 2650,
    "required_capacity": 3282
  },
  "fallback_allocation": {"calls": 3771, "bytes": 1158708},
  "fallback_copy": {"calls": 3770, "bytes": 856928}
}
```

Dominant miss reason:

```text
non_top_heap
```

Follow-up:

```text
issue 365 is updated to target non-top array-growth allocation/copy pressure instead of more top-of-heap or committed-memory probes.
```

Commits:

- final child commit

Validation result:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; emitted attribution.array_push_grow with top_miss_reason=non_top_heap
date: 2026-05-01

command: mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30
result: pass; confirmed non_top_heap remains dominant at the larger diagnostic budget
date: 2026-05-01

command: mise run check scripts
result: pass
date: 2026-05-01

command: mise run update-issue-index -- --check
result: pass
date: 2026-05-01

command: mise run check issues
result: pass
date: 2026-05-01
```

Remaining risks:

- The focused ABC451 depth-8 gate is still tracked by issue 357 and is not closed by this diagnostic-only issue.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/366-add-arraypushgrow-miss-attribution-diagnostic.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
