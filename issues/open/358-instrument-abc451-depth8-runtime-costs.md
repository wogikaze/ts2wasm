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

- [ ] Add a debug/instrumented build path, test helper, or backend-only diagnostic mode for ABC451 runtime cost attribution.
- [ ] Count or time array copy operations, copied bytes/elements, GC collections, sweep visits, free-list scan visits, and allocation attempts relevant to depth-8.
- [ ] Record baseline telemetry for the current committed runtime.
- [ ] Keep instrumentation off by default for normal builds/tests.

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
- `issues/open/358-instrument-abc451-depth8-runtime-costs.md`
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures

## Acceptance criteria

- [ ] A documented command emits counters separating array copy work from GC/sweep/free-list work for `fixtures/core-semantics/abc451-depth8-live-set.ts`.
- [ ] The diagnostic path is disabled by default and does not affect normal WAT/WASM output.
- [ ] The issue records baseline counter output from the current runtime.
- [ ] `cargo fmt --all --check`, `cargo test -p ts2wasm-backend-wasm --lib -- --nocapture`, `mise run update-issue-index -- --check`, and `mise run check issues` pass.

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

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
