---
id: 307
title: "Investigate ABC451 depth-9 allocation performance"
type: spike
area: runtime/performance
class: done
priority: P1
depends_on: []
blocks: [300]
created: 2026-04-29
updated: 2026-04-29
status: done
completed: 2026-04-29
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

- [x] Add bounded allocator/GC/performance telemetry for the depth-9 reducer,
      either as temporary diagnostic output behind a test-only switch or as a
      focused local reducer with measurable counters.
- [x] Identify whether 1024/2048-page timeouts are dominated by GC mark/sweep
      scans, recursive array copy growth, string allocation count, or another
      runtime path.
- [x] Implement a small runtime fix only if telemetry proves the cause and the
      focused reducer passes Node/iwasm differential within a practical timeout.
      No fix was committed because the telemetry isolated a broader GC cadence
      and sweep-scan cost, not a safe small runtime change.

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
- `issues/done/307-investigate-abc451-depth9-allocation-performance.md`
- `issues/index.md`

Do not touch:

- BigInt runtime/ABI files
- issue 302/306 eval files
- problem-specific source rewrite hooks
- unrelated artifacts

## Acceptance criteria

- [x] Bounded commands classify the 1024/2048-page depth-9 timeout with
      concrete allocation/GC/runtime evidence.
- [x] If implementation changes are made, a focused Node/iwasm regression test
      is added for the fixed reducer or runtime path.
- [x] If memory policy changes, `oom_alloc_check_must_fail_iwasm` still passes
      and `docs/14-runtime-abi.md` is updated.
- [x] Issue 300 remains open with updated evidence until official sample
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

- [x] not affected
- [x] updated: `docs/14-runtime-abi.md` if memory policy changes

Current state:

- [x] not affected
- [x] updated: `current-state.md` (repo root) if supported runtime facts change

Follow-up issues:

- [x] none
- [x] created/updated if telemetry proves a smaller implementation blocker

## Notes

Issue 305 evidence does not justify a committed implementation slice yet:
512 pages still traps, 1024/2048 pages do not complete in 90 seconds, and a
1MiB GC threshold WAT-only trial still times out.

2026-04-29 child `307-abc451-telemetry-20260429T193520Z` bounded telemetry:

- Recreated the depth-9 search-only reducer from
  `fixtures/core-semantics/abc451-depth8-live-set.ts` by changing
  `remainDigits` from `8 - before.length` to `9 - before.length`. Node prints
  the expected reducer count:

```sh
perl -pe 's/8 - before\.length/9 - before.length/' \
  fixtures/core-semantics/abc451-depth8-live-set.ts \
  > /tmp/abc451-search-depth-9-307.ts
node /tmp/abc451-search-depth-9-307.ts
```

Result:

```text
1404832
```

- Added a WAT-only diagnostic patch for the reducer that does not change the
  committed runtime policy. The patch raises the memory maximum to 1024 or
  2048 pages only for the temporary binary, counts allocation calls, total
  requested block bytes, GC collections, sweep block visits, freed sweep
  blocks, and heap high-water, then aborts after exactly 1,000,000 allocation
  calls and prints those counters through the existing runtime `console.log`
  path.
- The 1024-page and 2048-page diagnostic runs produced identical counters:

```text
alloc_count: 1000000
allocated_block_bytes: 62700952
gc_collect_count: 834
gc_sweep_block_visits: 196941253
gc_sweep_freed_blocks: 19816743
heap_high_water_bytes: 20258192
```

1024-page command:

```sh
/usr/bin/time -f 'elapsed:%e' timeout 60s \
  iwasm /tmp/abc451-search-depth-9-307-telemetry-cap1024.wasm
```

Result:

```text
1000000
62700952
834
196941253
19816743
20258192
Exception: unreachable
elapsed:16.36
```

2048-page command:

```sh
/usr/bin/time -f 'elapsed:%e' timeout 60s \
  iwasm /tmp/abc451-search-depth-9-307-telemetry-cap2048.wasm
```

Result:

```text
1000000
62700952
834
196941253
19816743
20258192
Exception: unreachable
elapsed:15.66
```

- A control run with only the WAT-only 2048-page memory maximum and no
  diagnostic abort still produced no output before a 20 second timeout:

```sh
/usr/bin/time -f 'elapsed:%e' timeout 20s \
  iwasm /tmp/abc451-search-depth-9-307-cap2048.wasm
```

Result:

```text
exit 124, no output, elapsed:20.01
```

Classification: the 1024/2048-page timeout is dominated by GC sweep cadence and
linear heap scan cost under recursive array/string allocation pressure. By the
first 1,000,000 allocation calls, the reducer has requested only about 62.7MB
of allocation blocks and reaches about 20.3MB heap high-water, but it has
already performed 834 collections and about 196.9M sweep block visits. Since
the 1024-page and 2048-page counters are identical at the bounded abort point,
raising `MEMORY_MAX_PAGES` alone is not justified. No runtime fix was committed
because the telemetry points to a broader GC cadence/free-list scan policy
change that needs its own implementation slice and completion-time proof.

## Completion evidence

Commits:

- `00b1871`

Validation result:

```text
command: node /tmp/abc451-search-depth-9-307.ts
result: pass; stdout 1404832
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 60s iwasm /tmp/abc451-search-depth-9-307-telemetry-cap1024.wasm
result: diagnostic abort after 1,000,000 allocations; alloc bytes 62,700,952; GC collections 834; sweep block visits 196,941,253; freed sweep blocks 19,816,743; heap high-water 20,258,192 bytes; elapsed 16.36s
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 60s iwasm /tmp/abc451-search-depth-9-307-telemetry-cap2048.wasm
result: same diagnostic counters as 1024-page run; elapsed 15.66s
date: 2026-04-29

command: /usr/bin/time -f 'elapsed:%e' timeout 20s iwasm /tmp/abc451-search-depth-9-307-cap2048.wasm
result: timeout 124 with no output; elapsed 20.01s
date: 2026-04-29
```

Remaining risks:

- No committed runtime fix was made. The next implementation slice should
  change GC cadence/free-list scan policy or array/string allocation behavior
  with completion-time evidence; it should not raise `MEMORY_MAX_PAGES` alone.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/307-investigate-abc451-depth9-allocation-performance.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
