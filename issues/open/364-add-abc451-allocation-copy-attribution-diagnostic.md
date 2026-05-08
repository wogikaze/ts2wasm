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
updated: 2026-05-01
status: done
completed: 2026-05-01
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

- [x] Extend `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` to report allocation/copy attribution categories relevant to ABC451.
- [x] Attribute at least array growth copies, generic `$copy` calls, string/concat copies if present, object/array allocation attempts, and GC/sweep counters.
- [x] Keep instrumentation off by default for normal builds/tests.
- [x] Record 100000-event baseline attribution after issue 363.
- [x] Create or update a follow-up implementation issue if the attribution identifies a clear high-volume target.

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

- [x] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and emits attribution categories beyond the existing aggregate counters.
- [x] Attribution output identifies the top remaining allocation/copy categories at the diagnostic budget.
- [x] Normal WAT/WASM output remains unchanged outside the diagnostic script.
- [x] `mise run check scripts` passes.
- [x] `mise run update-issue-index -- --check` and `mise run check issues` pass.
- [x] If a clear implementation target is found, create or update a focused follow-up issue and link it from issue 363.

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

- [x] created issue 365 from attribution evidence

## Notes

Issue 363 should not receive more broad allocator probes until this attribution exists or another concrete high-volume target is identified.

## Attribution baseline

100000-event baseline:

```json
{
  "top_targets": [
    {"kind": "allocation", "category": "array_growth", "calls": 2648, "bytes": 362976},
    {"kind": "copy", "category": "array_growth", "calls": 2648, "bytes": 181008},
    {"kind": "allocation", "category": "concat", "calls": 8935, "bytes": 104846},
    {"kind": "copy", "category": "concat_left", "calls": 8935, "bytes": 58302},
    {"kind": "allocation", "category": "scratch_array", "calls": 8936, "bytes": 35744},
    {"kind": "copy", "category": "concat_right", "calls": 8934, "bytes": 10803}
  ],
  "copy_unattributed": {"calls": 0, "bytes": 0},
  "allocation_unattributed": {"calls": 0, "bytes": 0},
  "gc_collections": 5,
  "sweep_visits": 58859,
  "free_list_scan_visits": 0
}
```

300000-event baseline:

```json
{
  "top_targets": [
    {"kind": "allocation", "category": "array_growth", "calls": 3771, "bytes": 1158708},
    {"kind": "copy", "category": "array_growth", "calls": 3770, "bytes": 856928},
    {"kind": "allocation", "category": "concat", "calls": 12710, "bytes": 149167},
    {"kind": "copy", "category": "concat_left", "calls": 12710, "bytes": 82923},
    {"kind": "allocation", "category": "scratch_array", "calls": 12712, "bytes": 50848},
    {"kind": "copy", "category": "concat_right", "calls": 12710, "bytes": 15404}
  ],
  "copy_unattributed": {"calls": 0, "bytes": 0},
  "allocation_unattributed": {"calls": 0, "bytes": 0},
  "gc_collections": 13,
  "sweep_visits": 241504,
  "free_list_scan_visits": 0
}
```

Top implementation target:

```text
issue 365: reduce ABC451 array-growth allocation/copy pressure.
Reason: array_growth dominates both attributed allocation bytes and attributed copy bytes at 100000 and 300000 diagnostic events.
```

## Completion evidence

Commits:

- final child commit

Validation result:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; emitted allocation/copy attribution and top_targets; array_growth is top allocation/copy target
date: 2026-05-01

command: mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30
result: pass; larger budget confirmed array_growth remains dominant
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

- issue 357 remains open; issue 365 tracks the next runtime-memory implementation target.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/364-add-abc451-allocation-copy-attribution-diagnostic.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
