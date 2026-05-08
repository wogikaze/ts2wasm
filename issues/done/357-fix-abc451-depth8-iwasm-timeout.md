---
id: 357
title: "Fix ABC451 depth-8 iwasm timeout"
type: bug
area: runtime/memory
class: blocked
priority: P1
depends_on: [385, 386]
blocks: [309]
created: 2026-04-30
updated: 2026-05-01
---

## Summary

Fix the current repo-wide full-suite blocker: `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` times out under `iwasm`.

This is a smaller child slice split from issue 309 after multiple workers narrowed the remaining blocker to runtime budget/performance rather than compile correctness.

## Problem

Full `cargo nextest run` and issue-specific broad gates now reach the ABC451 depth-8 fixture and fail only because the `iwasm` execution times out around 30 seconds.

Problem: `fixtures/core-semantics/abc451-depth8-live-set.ts` no longer finishes within the iwasm test timeout, blocking otherwise validated issue closes.

## Current failure

Observed during the parent cycle after integrating issues 347 and 355:

```sh
cargo nextest run
```

Result:

```text
619 passed, 1 failed, 4 skipped
FAIL ts2wasm-cli::m2_node_diff_fixture_tests::abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
iwasm timed out for fixtures/core-semantics/abc451-depth8-live-set.ts
```

The filtered spread/node_diff retry also reproduced the same failure:

```text
166 passed, 1 failed, 457 skipped
FAIL abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
iwasm timed out
```

## Desired final state

The depth-8 ABC451 live-set fixture completes under `iwasm` within the test timeout and prints Node-matching output, without weakening memory policy or hiding the test.

## Scope

In scope:

- [ ] Reduce runtime cost for the depth-8 fixture without increasing `MEMORY_MAX_PAGES`.
- [ ] Preserve the committed 185-page memory policy and explicit OOM regression behavior.
- [ ] Prefer representation/copy/root-liveness improvements that are semantics-preserving for general programs, not ABC451 source rewrites.
- [ ] Record exact before/after timeout or runtime evidence.

Out of scope:

- Raising `MEMORY_MAX_PAGES` without official sample completion evidence.
- Skipping, ignoring, or weakening the ABC451 test.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Problem-specific generated tables or source rewrites.

Split from this issue:
- Issue 385: Instrument ABC451 depth-8 copy vs GC time (focused on instrumentation to identify bottleneck)
- Issue 386: Reduce ABC451 depth-8 array copy pressure (focused on copy optimization)

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/abc451-depth8-live-set.ts` only if adding instrumentation-safe comments is unavoidable
- `issues/open/357-fix-abc451-depth8-iwasm-timeout.md`
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures

## Acceptance criteria

- [ ] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` passes.
- [ ] `cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm` passes.
- [ ] If runtime policy or representation changes, `cargo test -p ts2wasm-backend-wasm --lib -- --nocapture` passes.
- [ ] `cargo fmt --all --check` passes.
- [ ] `mise run update-issue-index -- --check` and `mise run check issues` pass.
- [ ] Issue 309 remains open unless depth-9 acceptance is separately met.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
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

- [x] not affected unless runtime memory policy changes

Current state:

- [x] updated only if runtime facts change

Follow-up issues:

- [x] none

## Notes

Issue 309 evidence already rejected array-growth-only approaches: exact-fit keeps the allocation shape smaller but remains too slow, while slack/geometric growth reduces copy pressure but trips the 185-page cap. Start from that evidence and avoid repeating the same rejected probes without a new hypothesis.

2026-04-30 child-357 blocker evidence:

- Baseline reproduced the assigned blocker:

```text
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.327s; stdout/stderr empty
```

- Rejected `memory.copy` plus small post-threshold array slack. The new
  hypothesis was not array-growth-only: combine bulk memory copy with bounded
  slack lower than the previously rejected geometric policies. It did not meet
  the required gate:

```text
candidate: $copy implemented with memory.copy; ArrayPushGrow exact-fit after 3072 plus 16 spare slots
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.231s

command: cargo run -q -- build fixtures/core-semantics/abc451-depth8-live-set.ts -o /tmp/abc451-depth8-live-set-357.wasm --host-deny && /usr/bin/time -f 'elapsed:%e' timeout 40s iwasm /tmp/abc451-depth8-live-set-357.wasm
result: fail; Exception: unreachable; elapsed:39.70

candidate: $copy implemented with memory.copy; ArrayPushGrow exact-fit after 3072 plus 8 spare slots; allocator max-cap GC preserved
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.254s
```

- Rejected allocator free-list-first GC suppression as implemented in this
  slice. The broad form skipped max-cap last-chance GC and trapped quickly; the
  narrowed allocation-pressure-only form preserved max-cap GC but did not
  improve the required gate enough:

```text
candidate: skip all pre-scan GC when gc_free_list_max_body_size can satisfy payload_size
result: fail; iwasm Exception: unreachable after 0.288s

candidate: skip only allocation-pressure GC when gc_free_list_max_body_size can satisfy payload_size, but still run max-cap last-chance GC
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.235s
```

- Rejected top-of-heap in-place `ArrayPushGrow` as implemented in this slice.
  The hypothesis was semantics-preserving for aliases when the array block was
  exactly at `$heap`, but the candidate trapped before producing output:

```text
candidate: grow top heap array block in place when current memory already covers the larger aligned payload
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm Exception: unreachable after 16.217s
```

- No runtime implementation from these probes was left in the tree. The next
  likely executable slice should instrument the remaining timeout by separating
  copy time from GC sweep/free-list time in the depth-8 fixture, then target a
  representation change that reduces post-3072 copying without increasing live
  capacity under the 185-page policy.

2026-05-01 child-385 instrumentation evidence:

- The existing default-off ABC451 diagnostic runner was used to instrument a
  temporary WAT copy. Normal WAT/WASM output is unchanged. Wall-clock timing
  inside wasm was not used; the evidence is deterministic operation counters,
  so the conclusion is about dominant operation volume rather than exact
  elapsed milliseconds.

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; diagnostic_stop=true; timed_out=false; runtime_exit.code=1 from intentional diagnostic stop

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
```

- Dominant operation at the 100000-event budget: GC sweep traversal
  (`sweep_visits=58859`). Free-list scan is not active on this run
  (`free_list_scan_visits=0`). Copy work is still material, especially
  `array_growth` (`181008` bytes), but it is not the largest deterministic
  counter by operation volume in this diagnostic sample.

```text
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail as expected for issue 385 instrumentation-only slice; iwasm timed out after 30.330s
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
