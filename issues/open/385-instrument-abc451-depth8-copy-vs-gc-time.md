---
id: 385
title: "Instrument ABC451 depth-8 copy vs GC time"
type: feature
area: runtime/memory
class: implementation-ready
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

- [ ] Add runtime instrumentation to measure copy time for array operations in the depth-8 fixture.
- [ ] Add runtime instrumentation to measure GC sweep time in the depth-8 fixture.
- [ ] Add runtime instrumentation to measure free-list scan time in the depth- fixture.
- [ ] Record the breakdown of time spent in each operation.
- [ ] Identify which operation dominates the timeout.

Out of scope:

- Changing the actual implementation without instrumentation evidence.
- Raising `MEMORY_MAX_PAGES`.
- Skipping, ignoring, or weakening the ABC451 test.
- BigInt, spread, eval, private-class, parser, or reference-harness work.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `fixtures/core-semantics/abc451-depth8-live-set.ts` (only for instrumentation-safe comments)
- `issues/open/385-instrument-abc451-depth8-copy-vs-gc-time.md`
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures

## Acceptance criteria

- [ ] Instrumentation shows the breakdown of time spent in copy vs GC vs free-list.
- [ ] The dominant operation is identified with evidence.
- [ ] Issue 357 is updated with the instrumentation findings.
- [ ] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` still times out (instrumentation-only slice).

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

- [ ] not affected unless runtime memory policy changes

Current state:

- [ ] updated only if runtime facts change

Follow-up issues:

- [ ] Issue 386 (reduce array copy pressure) if copy is the bottleneck
- [ ] Other follow-up depending on instrumentation findings

## Notes

This is a focused instrumentation slice from issue 357. Do not change implementation without evidence.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```

Remaining risks:

- Instrumentation overhead may affect timing; use lightweight counters.
