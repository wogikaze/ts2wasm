---
id: 307
title: "Investigate ABC451 depth-9 allocation performance"
type: spike
area: runtime/performance
class: implementation-ready
priority: P1
depends_on: []
blocks: [300]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Add bounded runtime evidence for the ABC451 depth-9 search-only live-set path
after issue 305 showed that raising the memory maximum alone is not a safe
implementation slice.

Problem: The depth-9 reducer prints `1404832` under Node. Under the committed
185-page policy it traps in `$alloc_heap`; WAT-only 512 pages still traps after
71 seconds; WAT-only 1024 and 2048 pages time out after 90 seconds without
output. A WAT-only 1MiB GC threshold trial with 2048 pages also times out.

## Desired final state

The depth-9 path has bounded telemetry or a smaller reducer that identifies
whether the next implementation should change GC scan cadence, root retention,
array/string allocation strategy, or a later runtime path. No memory maximum is
raised without completion-time evidence and OOM boundary proof.

## Scope

In scope:

- [ ] Add bounded allocator/GC/performance telemetry for the depth-9 reducer,
      either as temporary diagnostic output behind a test-only switch or as a
      focused local reducer with measurable counters.
- [ ] Identify whether 1024/2048-page timeouts are dominated by GC mark/sweep
      scans, recursive array copy growth, string allocation count, or another
      runtime path.
- [ ] Implement a small runtime fix only if telemetry proves the cause and the
      focused reducer passes Node/iwasm differential within a practical timeout.

Out of scope:

- Raising `MEMORY_MAX_PAGES` without completion-time evidence.
- BigInt runtime or ABI changes.
- Problem-specific source rewrite hooks or generated ABC451 tables.
- Claiming issue 300 compatibility.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `issues/open/300-support-abc451-large-integer-number-boundary.md`
- `issues/open/307-investigate-abc451-depth9-allocation-performance.md`
- `issues/index.md`

Do not touch:

- BigInt runtime/ABI files
- issue 302/306 eval files
- problem-specific source rewrite hooks
- unrelated artifacts

## Acceptance criteria

- [ ] Bounded commands classify the 1024/2048-page depth-9 timeout with
      concrete allocation/GC/runtime evidence.
- [ ] If implementation changes are made, a focused Node/iwasm regression test
      is added for the fixed reducer or runtime path.
- [ ] If memory policy changes, `oom_alloc_check_must_fail_iwasm` still passes
      and `docs/14-runtime-abi.md` is updated.
- [ ] Issue 300 remains open with updated evidence until official sample
      outputs `10 -> 21`, `69 -> 328`, and `1099898 -> 819264512` match Node.

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
node /tmp/abc451-search-depth-9-305.ts
cargo run -q -- build /tmp/abc451-search-depth-9-305.ts -o /tmp/abc451-search-depth-9-305.wasm --host-deny
timeout 90s iwasm /tmp/abc451-search-depth-9-305.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` if memory policy changes

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root) if supported runtime facts change

Follow-up issues:

- [ ] none
- [ ] created/updated if telemetry proves a smaller implementation blocker

## Notes

Issue 305 evidence does not justify a committed implementation slice yet:
512 pages still traps, 1024/2048 pages do not complete in 90 seconds, and a
1MiB GC threshold WAT-only trial still times out.

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
