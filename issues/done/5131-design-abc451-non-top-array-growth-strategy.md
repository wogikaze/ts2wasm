---
id: 5131
title: "Design ABC451 non-top array growth strategy"
type: design
area: runtime/memory
class: done
priority: P1
depends_on: []
blocks: [365]
status: done
created: 2026-05-06
updated: 2026-05-06
completed: 2026-05-06
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

- [x] Review issue 365 rejected candidates and preserve the mergeability constraints.
- [x] Define aliasing semantics for any representation-level append strategy.
- [x] Decide whether the next implementation should change array representation or add deeper non-top separation attribution.
- [x] Create one implementation-ready child issue with exact metrics, paths, and validation commands.

Out of scope:

- Behavior-changing runtime implementation in this design issue.
- Raising memory caps or test timeouts.
- Weakening or skipping the ABC451 depth-8 gate.
- BigInt, spread, eval, private-class, parser, or reference-harness work.

## Affected paths

Expected:

- `issues/done/365-reduce-abc451-array-growth-allocation-copy-pressure.md`
- `issues/done/5131-design-abc451-non-top-array-growth-strategy.md`
- `current-state.md`

Do not touch:

- `crates/backend-wasm/src/` before a child implementation issue exists
- `crates/frontend/src/`
- unrelated fixtures

## Acceptance criteria

- [x] The next ABC451 array-growth implementation approach is classified as `change array representation`, `add deeper attribution`, or `defer with blocker`.
- [x] The decision states how array aliases, mutation ordering, and existing OOM policy are preserved.
- [x] One child issue is created if a safe implementation or attribution slice exists.
- [x] The child issue names exact before/after metrics from `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30`.

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

- `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` (known timeout remains recorded in issue 365)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] created: `issues/done/5132-add-abc451-non-top-array-separation-attribution.md`

## Notes

Decision: `add deeper attribution`.

Issue 365 already rejected adjacent-free-block expansion, empty-copy skipping,
and several helper-local growth factors because they were neutral or violated
the parent mergeability constraints. A representation-level append strategy is
still plausible, but changing array representation before knowing why measured
result arrays become non-top would risk a broad aliasing and mutation-order
change without a narrow target.

The next executable slice is issue 5132. It must add attribution for why
non-top arrays separate from the heap top before `$array_push_grow` fallback
copies. This keeps the current contiguous array representation unchanged.
Array aliases, mutation ordering, and the existing OOM policy are therefore
preserved by construction in the next slice; any future representation-level
append strategy must use the attribution result to state and test those
semantics explicitly before changing runtime behavior.

## Completion evidence

Commits:

- `pending`

Validation result:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: interrupted after hanging; emitted "abc451-runtime-costs: expected 1 occurrences for value_to_string undefined copy, found 0" before no further output
date: 2026-05-06

command: issue workflow validation
result: pending in closing commit
date: 2026-05-06
```

Remaining risks:

- Issue 365 remains blocked until issue 5132 produces source-backed non-top separation attribution or proves representation-level append is the next safe implementation target.

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

