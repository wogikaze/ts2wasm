---
id: 5131
title: "Design ABC451 non-top array growth strategy"
type: design
area: runtime/memory
class: design-ready
priority: P1
depends_on: []
blocks: [365]
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Define a mergeable strategy for reducing ABC451 depth-8 non-top
`$array_push_grow` fallback allocation/copy pressure after helper-level local
policy probes failed.

Problem: issue 365 remains blocked because the dominant measured array-growth
pressure comes from non-top arrays, and prior helper-level growth-factor or
adjacent-free-block probes either produced no improvement or violated the
mergeability constraints.

## Current failure

The current issue-365 evidence shows:

```text
100000 events: alloc_array_growth_bytes=362976; alloc_array_growth_calls=2648;
copy_array_growth_bytes=181008; copy_array_growth_calls=2648;
top_miss_reason=non_top_heap; free_list_scan_visits=0
```

The focused gate still times out:

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Current result recorded in issue 365:

```text
fail; known iwasm timeout
```

## Desired final state

The project has a source-backed runtime-memory design decision for the next
ABC451 array-growth implementation slice: either a representation-level append
strategy with alias semantics, or a deeper attribution target proving a smaller
implementation blocker than non-top array growth.

## Scope

In scope:

- [ ] Review issue 365 rejected candidates and preserve the mergeability constraints.
- [ ] Define aliasing semantics for any representation-level append strategy.
- [ ] Decide whether the next implementation should change array representation or add deeper non-top separation attribution.
- [ ] Create one implementation-ready child issue with exact metrics, paths, and validation commands.

Out of scope:

- Behavior-changing runtime implementation in this design issue.
- Raising memory caps or test timeouts.
- Weakening or skipping the ABC451 depth-8 gate.
- BigInt, spread, eval, private-class, parser, or reference-harness work.

## Affected paths

Expected:

- `issues/open/365-reduce-abc451-array-growth-allocation-copy-pressure.md`
- `issues/open/5131-design-abc451-non-top-array-growth-strategy.md`
- `current-state.md`

Do not touch:

- `crates/backend-wasm/src/` before a child implementation issue exists
- `crates/frontend/src/`
- unrelated fixtures

## Acceptance criteria

- [ ] The next ABC451 array-growth implementation approach is classified as `change array representation`, `add deeper attribution`, or `defer with blocker`.
- [ ] The decision states how array aliases, mutation ordering, and existing OOM policy are preserved.
- [ ] One child issue is created if a safe implementation or attribution slice exists.
- [ ] The child issue names exact before/after metrics from `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30`.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
mise run check issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] created: `issues/open/...`

## Notes

This issue exists to unblock issue 365 without moving the broad parent into the
Ready queue. Keep the output small: one decision and, if possible, one child
implementation or attribution issue.

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
