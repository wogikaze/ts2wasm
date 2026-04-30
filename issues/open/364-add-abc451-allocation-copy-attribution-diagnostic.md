---
id: 364
title: "Add ABC451 allocation and copy attribution diagnostic"
type: test
area: runtime/performance
class: implementation-ready
priority: P1
depends_on: [363]
blocks: [363, 357, 309]
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Add attribution to the ABC451 runtime-cost diagnostic so the next runtime-memory fix can target the high-volume allocation/copy callsites instead of aggregate counters.

## Problem

Issue 363 reproduced the post-362 baseline but could not find a safe mergeable runtime-policy change. Several candidates either produced no counter delta or changed the focused gate from timeout to `Exception: unreachable`.

Problem: the current diagnostic reports aggregate allocation, copy, sweep, and free-list counters, but it does not identify which allocation or copy categories dominate the remaining depth-8 timeout.

## Current evidence

Issue 363 baseline at 100000 events:

```json
{
  "free_list_scan_visits": 0,
  "gc_collections": 5,
  "sweep_visits": 58859,
  "array_copy_calls": 2898,
  "array_copy_bytes": 182008,
  "array_copy_elements": 45502,
  "all_copy_calls": 20549,
  "all_copy_bytes": 250278,
  "allocation_attempts": 20587,
  "allocation_requested_bytes": 521193
}
```

Issue 363 rejected broad policy probes:

```text
GC_THRESHOLD * 3: lower sweep visits, but focused gate trapped with Exception: unreachable
GC_THRESHOLD * 2.5: lower sweep visits, but focused gate trapped with Exception: unreachable
top-of-heap memory.grow extension: no counter delta
empty-string concat fast path: counters worsened
ARRAY_PUSH_GROW_LINEAR_GROWTH_THRESHOLD 4096: no counter delta
```

## Desired final state

The diagnostic identifies the dominant remaining allocation/copy categories for `fixtures/core-semantics/abc451-depth8-live-set.ts`, with enough evidence to create or execute a focused runtime-memory fix.

## Scope

In scope:

- [ ] Extend `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` to report allocation/copy attribution categories relevant to ABC451.
- [ ] Attribute at least array growth copies, generic `$copy` calls, string/concat copies if present, object/array allocation attempts, and GC/sweep counters.
- [ ] Keep instrumentation off by default for normal builds/tests.
- [ ] Record 100000-event baseline attribution after issue 363.
- [ ] Create or update a follow-up implementation issue if the attribution identifies a clear high-volume target.

Out of scope:

- Production runtime policy changes without attribution evidence.
- Raising memory caps or timeouts.
- Skipping, ignoring, or weakening the ABC451 test.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Source rewriting the ABC451 fixture.

## Affected paths

Expected:

- `scripts/run/abc451-runtime-costs.py`
- `mise.toml` or `scripts/manager.py` only if the task contract changes
- `issues/open/364-add-abc451-allocation-copy-attribution-diagnostic.md`
- `issues/open/363-reduce-abc451-allocation-and-sweep-volume-after-bulk-copy-narrowing.md`
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures
- production runtime code unless a tiny diagnostic-compatible hook is unavoidable and off by default

## Acceptance criteria

- [ ] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and emits attribution categories beyond the existing aggregate counters.
- [ ] Attribution output identifies the top remaining allocation/copy categories at the diagnostic budget.
- [ ] Normal WAT/WASM output remains unchanged outside the diagnostic script.
- [ ] `mise run check scripts` passes.
- [ ] `mise run update-issue-index -- --check` and `mise run check issues` pass.
- [ ] If a clear implementation target is found, create or update a focused follow-up issue and link it from issue 363.

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

- [x] create or update based on attribution evidence

## Notes

Issue 363 should not receive more broad allocator probes until this attribution exists or another concrete high-volume target is identified.

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
