---
id: 358
title: "Instrument ABC451 depth-8 runtime costs"
type: test
area: runtime/performance
class: implementation-ready
priority: P1
depends_on: [357]
blocks: [357, 309]
created: 2026-04-30
updated: 2026-04-30
status: done
completed: 2026-04-30
---

## Summary

Add focused diagnostics or benchmark-style telemetry that separates ABC451 depth-8 time spent in array copying from GC sweep/free-list work.

This is the next executable slice after issue 357 showed several runtime-policy candidates still time out or trap without enough attribution.

## Problem

Issue 357 proved the depth-8 fixture still times out, but the evidence does not yet isolate whether the dominant cost is array-copying after capacity 3072, GC sweep/free-list traversal, or another runtime path.

Problem: ABC451 depth-8 timeout cannot be safely fixed without cost attribution for copying vs GC/free-list work.

## Current failure

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Current result:

```text
fail; iwasm timed out around 30s
```

Issue 357 rejected uninstrumented `memory.copy`, small slack, GC suppression, and top-of-heap grow probes.

## Desired final state

A reproducible diagnostic command or test artifact reports enough counters to choose the next implementation target, without changing production runtime behavior or weakening gates.

## Scope

In scope:

- [x] Add a debug/instrumented build path, test helper, or backend-only diagnostic mode for ABC451 runtime cost attribution.
- [x] Count or time array copy operations, copied bytes/elements, GC collections, sweep visits, free-list scan visits, and allocation attempts relevant to depth-8.
- [x] Record baseline telemetry for the current committed runtime.
- [x] Keep instrumentation off by default for normal builds/tests.

Out of scope:

- Production runtime policy changes without attribution evidence.
- Raising memory caps or timeouts.
- BigInt/spread/eval/private-class work.
- Source rewriting the ABC451 fixture.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/` if adding a diagnostic test hook
- `scripts/` only if a small diagnostic runner is the cleanest path
- `issues/done/358-instrument-abc451-depth8-runtime-costs.md`
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures

## Acceptance criteria

- [x] A documented command emits counters separating array copy work from GC/sweep/free-list work for `fixtures/core-semantics/abc451-depth8-live-set.ts`.
- [x] The diagnostic path is disabled by default and does not affect normal WAT/WASM output.
- [x] The issue records baseline counter output from the current runtime.
- [x] `cargo fmt --all --check`, `cargo test -p ts2wasm-backend-wasm --lib -- --nocapture`, `mise run update-issue-index -- --check`, and `mise run check issues` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected unless instrumentation reveals a new runtime fact

Follow-up issues:

- [x] created/updated based on telemetry if the next implementation target is clear

## Notes

Keep this as an instrumentation slice. A useful outcome is a mergeable diagnostic hook plus evidence, even if issue 357 remains blocked.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `a689b6ab`

Diagnostic command:

```sh
python scripts/run/abc451-runtime-costs.py --event-budget 100000 --timeout 30
```

Baseline counter output:

```json
{
  "diagnostic": "abc451-runtime-costs",
  "fixture": "fixtures/core-semantics/abc451-depth8-live-set.ts",
  "default_off": true,
  "event_budget": 100000,
  "diagnostic_stop": true,
  "timed_out": false,
  "counters": {
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
  },
  "runtime_exit": {
    "code": 1,
    "stderr_tail": ""
  }
}
```

Interpretation:

- At the 100000-event budget, free-list scanning dominates the observed accounting (`67379` visits) versus array-copy work (`1584` calls, `96408` bytes / `24102` elements) and sweep work (`13562` visits across `2` collections).
- The diagnostic stop is intentional: the instrumented WAT prints counters and then traps when the event budget is reached, avoiding the existing depth-8 timeout while preserving the current runtime policy.

Validation result:

```text
command: python scripts/run/abc451-runtime-costs.py --event-budget 100000 --timeout 30
result: pass; emitted JSON counters above; diagnostic_stop=true; timed_out=false
date: 2026-04-30

command: cargo fmt --all --check
result: pass
date: 2026-04-30

command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: pass
date: 2026-04-30

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-30

command: mise run check issues
result: pass
date: 2026-04-30
```

Remaining risks:

- Counter labels are emitted by the Python diagnostic runner; the instrumented WAT prints numeric values in a fixed order and intentionally traps after reporting when the budget is reached.
