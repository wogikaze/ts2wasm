---
id: 308
title: "Implement ABC451 depth-9 GC cadence policy"
type: feature
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: []
blocks: [300]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Use the issue 307 depth-9 telemetry to implement the next bounded runtime slice
for ABC451: reduce GC sweep cadence and linear free-list scan cost under
recursive array/string allocation pressure.

Problem: the depth-9 search-only reducer reaches 1,000,000 allocation calls
with only about 62.7MB requested allocation bytes and about 20.3MB heap
high-water, but already performs 834 collections and about 196.9M sweep block
visits. WAT-only 1024/2048-page runs time out without output, so raising
`MEMORY_MAX_PAGES` alone is not justified.

## Desired final state

The depth-9 search-only reducer either completes under a committed runtime
policy with Node-matching `1404832`, or the next smaller GC/free-list blocker is
isolated with bounded evidence and issue 300 remains open.

## Scope

In scope:

- [ ] Adjust GC trigger cadence, sweep/free-list scanning, or allocation policy
      based on the issue 307 counters.
- [ ] Add focused regression or telemetry coverage that proves the chosen
      policy reduces repeated sweep scans without hiding OOM boundaries.
- [ ] Preserve the explicit OOM trap behavior.
- [ ] Update issue 300 with committed runtime evidence.

Out of scope:

- Raising `MEMORY_MAX_PAGES` without completion-time evidence.
- BigInt runtime or ABI changes.
- Problem-specific ABC451 source rewrites or generated tables.
- Claiming full issue 300 compatibility before official sample outputs match
  Node.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/layout.rs` only if policy constants change
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md` if memory or GC policy changes
- `current-state.md` if runtime facts change
- `issues/open/300-support-abc451-large-integer-number-boundary.md`
- `issues/open/308-implement-abc451-depth9-gc-cadence-policy.md`
- `issues/index.md`

Do not touch:

- BigInt runtime/ABI files
- issue 302/306 eval files
- problem-specific source rewrite hooks
- unrelated artifacts

## Acceptance criteria

- [ ] Bounded evidence shows lower GC sweep pressure than issue 307's
      baseline (`gc_collect_count=834`, `gc_sweep_block_visits=196941253` at
      1,000,000 allocations).
- [ ] If the depth-9 reducer completes, it prints Node-matching `1404832`.
- [ ] `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` remains
      passing.
- [ ] `oom_alloc_check_must_fail_iwasm` remains passing if runtime memory or GC
      policy changes.
- [ ] Issue 300 remains open unless all official ABC451 sample outputs match
      Node.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
focused new/affected tests if added
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
node /tmp/abc451-search-depth-9-307.ts
cargo run -q -- build /tmp/abc451-search-depth-9-307.ts -o /tmp/abc451-search-depth-9-307.wasm --host-deny
timeout 90s iwasm /tmp/abc451-search-depth-9-307.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` if memory or GC policy changes

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` if runtime facts change

Follow-up issues:

- [ ] none
- [ ] created/updated if a smaller implementation blocker is isolated

## Notes

Issue 307 baseline at the 1,000,000 allocation diagnostic abort:

```text
alloc_count: 1000000
allocated_block_bytes: 62700952
gc_collect_count: 834
gc_sweep_block_visits: 196941253
gc_sweep_freed_blocks: 19816743
heap_high_water_bytes: 20258192
```

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
