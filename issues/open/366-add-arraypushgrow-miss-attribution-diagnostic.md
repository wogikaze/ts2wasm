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

- [ ] Extend `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` to report `ArrayPushGrow` hit/miss attribution.
- [ ] Count at least top-of-heap hits, non-top misses, committed-memory misses, capacity/threshold path counts, and bytes/calls associated with misses if practical.
- [ ] Keep instrumentation off by default for normal builds/tests.
- [ ] Record the 100000-event baseline miss attribution.
- [ ] Create or update a focused implementation issue if the miss attribution identifies a clear target.

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

- [ ] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and emits `ArrayPushGrow` hit/miss attribution beyond aggregate array-growth counters.
- [ ] Attribution output identifies the dominant miss reason at the diagnostic budget.
- [ ] Normal WAT/WASM output remains unchanged outside the diagnostic script.
- [ ] `mise run check scripts` passes.
- [ ] `mise run update-issue-index -- --check` and `mise run check issues` pass.
- [ ] If a clear implementation target is found, create or update a focused follow-up issue and link it from issue 365.

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
