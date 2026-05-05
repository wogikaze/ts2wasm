---
id: 385
title: "Instrument ABC451 depth-8 copy vs GC time"
type: feature
area: runtime/memory
class: done
priority: P1
depends_on: []
blocks: [357]
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Instrument the ABC451 depth-8 fixture to separate copy time from GC sweep/free-list time and identify the actual bottleneck.

Problem: Issue 357's previous attempts to reduce the timeout did not identify whether the bottleneck is array copying, GC sweep, free-list scanning, or a combination. Without instrumentation, optimization attempts are blind.

## Problem

The depth-8 ABC451 fixture times out under iwasm around 30 seconds. Previous attempts (memory.copy + array slack, GC suppression, in-place growth) all failed without clear evidence of which operation dominates the runtime.

Problem: Cannot optimize effectively without knowing which operation dominates the timeout.

## Current failure

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Result: iwasm timed out after ~30 seconds.

## Desired final state

The depth-8 ABC451 fixture has instrumentation that separates copy time from GC sweep/free-list time, allowing targeted optimization of the actual bottleneck.

## Scope

In scope:

- [x] Add runtime instrumentation to measure copy work for array operations in the depth-8 fixture.
- [x] Add runtime instrumentation to measure GC sweep work in the depth-8 fixture.
- [x] Add runtime instrumentation to measure free-list scan work in the depth-8 fixture.
- [x] Record the breakdown of work spent in each operation.
- [x] Identify which operation dominates the timeout.

Out of scope:

- Changing the actual implementation without instrumentation evidence.
- Raising `MEMORY_MAX_PAGES`.
- Skipping, ignoring, or weakening the ABC451 test.
- BigInt, spread, eval, private-class, parser, or reference-harness work.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `fixtures/core-semantics/abc451-depth8-live-set.ts` (only for instrumentation-safe comments)
- `issues/done/385-instrument-abc451-depth8-copy-vs-gc-time.md`
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures

## Acceptance criteria

- [x] Instrumentation shows the breakdown of work spent in copy vs GC vs free-list.
- [x] The dominant operation is identified with evidence.
- [x] Issue 357 is updated with the instrumentation findings.
- [x] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` still times out (instrumentation-only slice).

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
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

- [x] not affected; no runtime memory policy changed

Current state:

- [x] not updated; instrumentation records current issue evidence only

Follow-up issues:

- [x] Issue 386 remains open for array copy pressure, but this run identified GC sweep visits as the dominant counter at the 100000-event diagnostic budget.
- [x] Issue 357 is updated with the current counter evidence.

## Notes

This is a focused instrumentation slice from issue 357. Do not change implementation without evidence.

## Completion evidence

Completed: 2026-05-01

Instrumentation path:

- `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30`
- The command uses the existing default-off ABC451 diagnostic runner. It instruments a temporary WAT copy and leaves normal WAT/WASM output unchanged.
- Wall-clock timing inside wasm was not used; evidence is deterministic operation counters. This limits the conclusion to relative operation volume, not exact elapsed milliseconds.

Commits:

- pending child worker commit

Validation result:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; diagnostic_stop=true; timed_out=false; runtime_exit.code=1 from intentional diagnostic stop
date: 2026-05-01

counters:
  sweep_visits=58859
  gc_collections=5
  free_list_scan_visits=0
  all_copy_calls=20549
  all_copy_bytes=250278
  array_copy_calls=2898
  array_copy_bytes=182008
  array_copy_elements=45502
  allocation_attempts=20587
  allocation_requested_bytes=521193

top attribution:
  allocation array_growth: calls=2648 bytes=362976
  copy array_growth: calls=2648 bytes=181008
  allocation concat: calls=8935 bytes=104846
  copy concat_left: calls=8935 bytes=58302
  allocation scratch_array: calls=8936 bytes=35744

dominant operation by deterministic counter volume:
  GC sweep traversal dominates at this budget: sweep_visits=58859, while free_list_scan_visits=0 and all_copy_calls=20549.

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail as expected for instrumentation-only slice; iwasm timed out after 30.330s; 0 passed, 1 failed, 432 skipped
date: 2026-05-01

command: cargo fmt --all --check
result: pass
date: 2026-05-01
```

Remaining risks:

- Counter evidence does not measure exact wall-clock time inside wasm.
- The diagnostic intentionally stops at an event budget, so the evidence identifies the dominant early timeout path rather than full-run completion cost.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/385-instrument-abc451-depth8-copy-vs-gc-time.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
