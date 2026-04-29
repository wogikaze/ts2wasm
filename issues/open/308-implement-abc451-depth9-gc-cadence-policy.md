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

- [x] Adjust GC trigger cadence, sweep/free-list scanning, or allocation policy
      based on the issue 307 counters.
- [x] Add focused regression or telemetry coverage that proves the chosen
      policy reduces repeated sweep scans without hiding OOM boundaries.
- [x] Preserve the explicit OOM trap behavior for committed code.
- [x] Update issue 300 with attempted-policy evidence.

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

- [x] Bounded evidence shows lower GC sweep pressure than issue 307's
      baseline (`gc_collect_count=834`, `gc_sweep_block_visits=196941253` at
      1,000,000 allocations).
- [ ] If the depth-9 reducer completes, it prints Node-matching `1404832`.
- [x] `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` remains
      passing.
- [x] `oom_alloc_check_must_fail_iwasm` remains passing if runtime memory or GC
      policy changes.
- [x] Issue 300 remains open unless all official ABC451 sample outputs match
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

- [x] updated: `docs/14-runtime-abi.md` to state the current
      `alloc_bytes_since_last_gc + requested_block_size` threshold contract
- [x] updated: `docs/14-runtime-abi.md` to state the committed 12-page GC
      headroom and 16-page minimum memory growth policy
- [x] updated: `docs/14-runtime-abi.md` to state max-cap last-chance GC before
      free-list scan and OOM trap

Current state:

- [x] updated: `current-state.md` records the committed headroom-aware GC
      cadence slice and residual depth-9 blocker
- [x] updated: `current-state.md` records the max-cap last-chance GC slice and
      residual depth-9 / official sample blocker

Follow-up issues:

- [x] none; issue 308 remains open with narrowed negative evidence

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

2026-04-29 child `308-gc-cadence-20260429T195300Z` progress:

- Tested three candidate GC cadence changes and did not commit them because
  each failed the required depth-8 regression
  `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` with
  `Exception: unreachable`:
  free-list-first collection with post-GC retry, free-list reuse not counting
  as fresh allocation pressure, and raising `GC_THRESHOLD` to 128KiB.
- Kept the committed runtime policy unchanged. Under that policy, the depth-9
  search-only reducer still traps in `$alloc_heap`:

```text
command: /usr/bin/time -f 'elapsed:%e' timeout 120s iwasm /tmp/abc451-search-depth-9-308.wasm
result: Exception: unreachable; elapsed 5.52
date: 2026-04-29
```

- WAT-only telemetry for the uncommitted free-list-first experiment with memory
  max 1024 and 2048 pages produced identical counters at the 1,000,000
  allocation diagnostic abort:

```text
alloc_count: 1000000
allocated_block_bytes: 62642176
gc_collect_count: 456
gc_sweep_block_visits: 96942634
gc_sweep_freed_blocks: 11591576
heap_high_water_bytes: 20487448
```

This would lower the issue 307 baseline from `gc_collect_count=834` to `456`
and from `gc_sweep_block_visits=196941253` to `96942634`, but it is not a safe
runtime change because it regresses the committed depth-8 reducer. Issue 308
remains open. Issue 300 remains open.

2026-04-29 child `308-gc-scan-slice-20260429T195405Z` progress:

- Committed a memory-headroom-aware GC cadence policy: allocation-pressure GC
  now runs only when `alloc_bytes_since_last_gc + requested_block_size >=
  GC_THRESHOLD` and the bump allocation result is within
  `GC_HEADROOM_PAGES=12` pages of the currently reserved memory. Small
  `memory.grow` requests are rounded up to `HEAP_GROW_MIN_PAGES=16` when that
  remains below `MEMORY_MAX_PAGES=185`.
- The committed `MEMORY_MAX_PAGES=185` depth-9 search-only reducer still traps
  in `$alloc_heap`, so issue 308 remains open and issue 300 compatibility is
  not claimed:

```text
command: /usr/bin/time -f 'elapsed:%e' timeout 90s iwasm /tmp/abc451-search-depth-9-308-slice.wasm
result: Exception: unreachable; elapsed 6.44
date: 2026-04-29
```

- WAT-only telemetry with memory max 1024 and the committed 12-page headroom
  policy produced lower 1,000,000-allocation sweep pressure than issue 307:

```text
alloc_count: 1000000
allocated_block_bytes: 62908504
gc_collect_count: 790
gc_sweep_block_visits: 192697486
gc_sweep_freed_blocks: 20567471
heap_high_water_bytes: 20524256
elapsed: 16.29
```

- Nearby lower-headroom candidates were rejected because they regressed the
  required depth-8 fixture with `Exception: unreachable`:
  `GC_HEADROOM_PAGES=1`, `8`, and `10`. `GC_HEADROOM_PAGES=16` passed depth-8
  but did not reduce issue 307 telemetry (`gc_collect_count=837`,
  `gc_sweep_block_visits=197777965`).

2026-04-29 child `308-free-list-scan-20260429T201523Z` progress:

- Committed a bounded free-list scan guard in `$alloc_heap`: `$gc_sweep`
  now records `$gc_free_list_max_body_size`, and allocation skips the linear
  free-list scan when the aligned requested payload is larger than every free
  block found by the last sweep. The guard is conservative: stale values can
  only overestimate and cause an extra scan, not skip a reusable block.
- Added backend WAT contract coverage for the new runtime global, sweep reset,
  max-body update, and allocation-side guard.
- The depth-9 search-only reducer still does not complete under the committed
  185-page memory cap:

```text
command: node /tmp/abc451-search-depth-9-308-slice.ts
result: pass; stdout 1404832
date: 2026-04-29

command: cargo run -q -- build /tmp/abc451-search-depth-9-308-slice.ts -o /tmp/abc451-search-depth-9-308-free-list-scan.wasm --host-deny
result: pass
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 90s iwasm /tmp/abc451-search-depth-9-308-free-list-scan.wasm
result: trapped with Exception: unreachable after 7.60s under committed 185-page policy
date: 2026-04-29
```

- Issue 300 remains open. No official ABC451 sample compatibility is claimed.

2026-04-29 child `308-alloc-pattern-20260429T203035Z` progress:

- Committed a max-cap last-chance GC policy in `$alloc_heap`: when
  `memory.size == MEMORY_MAX_PAGES` and the bump allocation would exceed the
  currently reserved memory, allocation now runs GC before the free-list scan
  and before the existing OOM trap. This is separate from the rejected
  free-list-first candidates; it only applies once the committed memory cap is
  already reached, and the explicit OOM trap remains if no reusable swept block
  can satisfy the request.
- Added backend WAT contract coverage for the max-page condition and
  bump-exceeds-memory guard.
- Required depth-8 and OOM regressions still pass, but this does not complete
  issue 308:

```text
command: node /tmp/abc451-search-depth-9-308-lastchance.ts
result: pass; stdout 1404832
date: 2026-04-29

command: cargo run -q -- build /tmp/abc451-search-depth-9-308-lastchance.ts -o /tmp/abc451-search-depth-9-308-lastchance.wasm --host-deny
result: pass
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 90s iwasm /tmp/abc451-search-depth-9-308-lastchance.wasm
result: trapped with Exception: unreachable after 8.41s under committed 185-page policy
date: 2026-04-29
```

- The official smallest ABC451 sample also remains blocked:

```text
command: /usr/bin/time -f 'elapsed:%e' timeout 90s sh -c "printf '10\n' | iwasm /tmp/abc451-d-308-lastchance.wasm"
result: trapped with Exception: unreachable after 10.09s under committed 185-page policy
date: 2026-04-29
```

- Issue 300 remains open. No official ABC451 sample compatibility is claimed.

2026-04-29 child `308-next-blocker-20260429T204403Z` progress:

- Committed sweep tail trimming: when `$gc_sweep` coalesces an unmarked range
  that reaches the current `$heap` end, it lowers `$heap` to the range start
  instead of linking the tail into the free list. Future bump allocation can
  reuse that top-of-heap garbage directly, and the free-list scan does not have
  to visit the tail block.
- Added backend WAT contract coverage for the tail-trim path.
- Required depth-8 and OOM regressions still pass, but the depth-9 reducer and
  official smallest sample remain blocked under the committed 185-page cap:

```text
command: node /tmp/abc451-search-depth-9-308-lastchance.ts
result: pass; stdout 1404832
date: 2026-04-29

command: cargo run -q -- build /tmp/abc451-search-depth-9-308-lastchance.ts -o /tmp/abc451-search-depth-9-tailtrim.wasm --host-deny
result: pass
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 90s iwasm /tmp/abc451-search-depth-9-tailtrim.wasm
result: trapped with Exception: unreachable after 9.84s under committed 185-page policy
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 90s sh -c "printf '10\n' | iwasm /tmp/abc451-d-tailtrim.wasm"
result: trapped with Exception: unreachable after 10.85s under committed 185-page policy
date: 2026-04-29
```

- WAT-only 1024-page telemetry with the committed tail-trim policy lowered
  bounded sweep pressure further than the previous committed issue 308 slice:

```text
alloc_count: 1000000
allocated_block_bytes: 62648256
gc_collect_count: 729
gc_sweep_block_visits: 171324221
gc_sweep_freed_blocks: 18531756
heap_high_water_bytes: 20430896
gc_tail_trim_count: 9
gc_tail_trim_bytes: 49712
elapsed: 16.47
```

This improves the previous committed telemetry from `gc_collect_count=790` and
`gc_sweep_block_visits=192697486`, but it does not complete the depth-9
reducer. Issue 308 remains open. Issue 300 remains open.

2026-04-29 child `308-next-gc-20260429T210037Z` progress:

- Committed a tighter free-list max summary: `$gc_sweep` now records both the
  largest and second-largest swept free block body sizes, and `$alloc_heap`
  lowers `$gc_free_list_max_body_size` after consuming or splitting a block
  whose original size matched the previous maximum.
- This preserves the conservative skip-scan invariant: stale summary values may
  still cause an extra scan, but must not skip a reusable free block.
- Required depth-8 and OOM regressions still pass, but the depth-9 reducer
  remains blocked under the committed 185-page cap:

```text
command: /usr/bin/time -f 'elapsed:%e' timeout 90s iwasm /tmp/abc451-search-depth-9-secondmax.wasm
result: trapped with Exception: unreachable after 10.06s under committed 185-page policy
date: 2026-04-29
```

Issue 308 remains open. Issue 300 remains open.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: cargo test -p ts2wasm-backend-wasm -- --nocapture
result: pass; 27 tests passed
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: pass after reverting unsafe runtime candidates; 1 test passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass; 1 test passed
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK
date: 2026-04-29

command: mise run check issues
result: fail; pre-existing missing test262 coverage result artifact references in unrelated issues 271, 284-286, 288-289, 291-293, and 296
date: 2026-04-29

command: node /tmp/abc451-search-depth-9-308.ts
result: pass; stdout 1404832
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 120s iwasm /tmp/abc451-search-depth-9-308.wasm
result: trapped with Exception: unreachable after 5.52s under committed 185-page policy
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 60s iwasm /tmp/abc451-search-depth-9-308-telemetry-cap1024.wasm
result: diagnostic abort after 1,000,000 allocations; GC collections 456; sweep block visits 96,942,634; elapsed 13.61s
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 60s iwasm /tmp/abc451-search-depth-9-308-telemetry-cap2048.wasm
result: same diagnostic counters as 1024-page run; elapsed 13.77s
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: failed for each uncommitted runtime candidate with iwasm `Exception: unreachable`; runtime candidates were not committed
date: 2026-04-29

command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: pass; 27 tests passed
date: 2026-04-29

command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: pass; 27 tests passed including free-list max-scan guard WAT contract
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: pass; 1 test passed with `GC_HEADROOM_PAGES=12`
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: pass; 1 test passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass; 1 test passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass; 1 test passed
date: 2026-04-29

command: node /tmp/abc451-search-depth-9-308-slice.ts
result: pass; stdout 1404832
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 90s iwasm /tmp/abc451-search-depth-9-308-slice.wasm
result: trapped with Exception: unreachable after 6.44s under committed 185-page policy
date: 2026-04-29

command: node /tmp/abc451-search-depth-9-308-slice.ts
result: pass; stdout 1404832
date: 2026-04-29

command: cargo run -q -- build /tmp/abc451-search-depth-9-308-slice.ts -o /tmp/abc451-search-depth-9-308-free-list-scan.wasm --host-deny
result: pass
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 90s iwasm /tmp/abc451-search-depth-9-308-free-list-scan.wasm
result: trapped with Exception: unreachable after 7.60s under committed 185-page policy
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 60s iwasm /tmp/abc451-search-depth-9-308-slice-telemetry-cap1024.wasm
result: diagnostic abort after 1,000,000 allocations; GC collections 790; sweep block visits 192,697,486; freed sweep blocks 20,567,471; heap high-water 20,524,256 bytes; elapsed 16.29s
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: failed for `GC_HEADROOM_PAGES=1`, `8`, and `10` with iwasm `Exception: unreachable`; those candidates were not committed
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK
date: 2026-04-29

command: mise run check issues
result: pass after copying parent `artifacts/coverage/results/test262-results.jsonl` into the worktree as allowed by the assignment
date: 2026-04-29

command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: pass; 27 tests passed including sweep tail-trim WAT contract
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: pass; 1 test passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass; 1 test passed
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 60s iwasm /tmp/abc451-search-depth-9-tailtrim-telemetry-cap1024.wasm
result: diagnostic abort after 1,000,000 allocations; GC collections 729; sweep block visits 171,324,221; freed sweep blocks 18,531,756; heap high-water 20,430,896 bytes; tail trims 9; tail-trimmed bytes 49,712; elapsed 16.47s
date: 2026-04-29

command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: pass; 27 tests passed including second-max free-list summary WAT contract
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: pass; 1 test passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass; 1 test passed
date: 2026-04-29
```

Remaining risks:

- Depth-9 search-only and official ABC451 sample compatibility remain open in
  issue 308 / issue 300.
