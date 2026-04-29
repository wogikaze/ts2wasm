---
id: 300
title: "Support ABC451 large integer number boundary"
type: feature
area: runtime
class: blocked
priority: P1
depends_on: [308]
blocks: [294]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement the smallest architecture-preserving number representation or lowering
path needed for the ABC451 D fixture after issue 299.

Problem: `fixtures/atcoder/abc451-d-concat-power2.ts` now reaches a large
ordinary number literal, but the current tagged small-int wire representation
rejects `1000000000` before wasm generation.

## Current failure

Reproduction:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-large-number.wasm --host-deny
```

Current result:

```text
error: [NumberOutOfRange] number literal 1000000000 is out of small-int tagged range (-268435456..=268435455)
```

The failure occurs at the loop condition in the ABC451 fixture:

```ts
for (let i = 0; 2 ** i <= 1000000000; i++) {
```

## Desired final state

The ABC451 fixture advances beyond the `1000000000` literal without weakening
range validation or silently wrapping tagged small-int values.

For this slice, it is acceptable to implement either:

- a real runtime/ABI representation path for large integer-valued `number`
  values used by the fixture; or
- a proven, narrowly documented lowering path that preserves Node-compatible
  observable output for the fixture's integer-only arithmetic and comparisons.

## Scope

In scope:

- [ ] Represent or lower ordinary `number` values needed by ABC451 up to at
      least `1_000_000_000`.
- [ ] Preserve correct behavior for `2 ** i <= 1000000000`, `String(n)`,
      numeric sort comparator values, `Set<number>`, and the official sample
      output path through `819264512`.
- [ ] Keep existing small-int behavior unchanged for values already
      representable by `ValueTag`.
- [ ] Add focused regression coverage for the new large-integer number path.

Out of scope:

- Full IEEE-754 `number` semantics for fractional values, `NaN`, `Infinity`,
  and signed zero.
- BigInt runtime value support.
- Reclassifying BigInt/Number mixed comparison work owned by issue 281.
- Source-text recognition or replacement of the ABC451 program.

## Affected paths

Expected:

- `crates/runtime-abi/src/value.rs`
- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `fixtures/atcoder/`
- `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`
- `issues/open/300-support-abc451-large-integer-number-boundary.md`

Do not touch:

- problem-specific source rewrite hooks
- BigInt runtime representation, unless only adding explicit non-overlap notes

## Acceptance criteria

- [ ] `fixtures/atcoder/abc451-d-concat-power2.ts` builds past the current
      `NumberOutOfRange` diagnostic for `1000000000`.
- [ ] A focused regression fixture proves the supported large integer number
      path matches Node under `iwasm`.
- [ ] Existing small-int tests still pass.
- [ ] Unsupported number forms outside this slice still produce explicit
      diagnostics or traps instead of silent miscompilation.
- [ ] Issue 294 is updated with the new next blocker or closed if the official
      sample outputs are fully verified.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-large-number.wasm --host-deny
printf '10\n' | iwasm /tmp/abc451-d-large-number.wasm
printf '69\n' | iwasm /tmp/abc451-d-large-number.wasm
printf '1099898\n' | iwasm /tmp/abc451-d-large-number.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` if the runtime ABI representation changes
- [ ] updated: `docs/05-compatibility-and-semantics.md` if supported number
      semantics change

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root) if the supported subset changes

Follow-up issues:

- [ ] none
- [ ] created/updated if the slice proves a broader number-model design is
      required before implementation.

## Notes

Do not remove the `NumberOutOfRange` validator without replacing it with an
equivalent guard for unsupported values. The close condition is observable
Node-compatible behavior for the large integer subset, not merely accepting the
literal.

## Progress evidence

2026-04-29 child `019dda13-74bf-7ec2-9146-e75ae64c098c`:

- Implemented a narrow integer-only heap-number path for ordinary `number`
  values outside the tagged small-int payload range.
- Added `fixtures/core-semantics/large-integer-number-boundary.ts` covering
  `2 ** i <= 1000000000`, `String`/unary-plus round trip through large integer
  strings, `Set<number>` duplicate handling, and numeric sort values including
  `819264512`.
- Verified the reduced regression manually with Node/iwasm matching output:

```text
536870912
536870912
819264512
```

- Verified `fixtures/atcoder/abc451-d-concat-power2.ts` now builds past the
  previous `NumberOutOfRange` diagnostic:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-large-number-child.wasm --host-deny
```

- The ABC451 sample execution is not yet done: all three issue validation
  inputs currently trap under `iwasm` with `Exception: out of bounds memory
  access`. The issue remains open until the official sample path is safe and
  Node-compatible.

2026-04-29 child `019dda13-74bf-7ec2-9146-e75ae64c098c`:

- Isolated the first post-build runtime trap to statement-form
  `array.push(...)` on arrays allocated from `[]`: array literals allocated
  only the length header while the existing push runtime wrote element payload
  past that allocation.
- Added a narrow lowering/backend path for unused local-array `push`
  statements. It reassigns the local to an array value that mutates in place
  while the existing GC allocation body size has spare capacity, and reallocates
  with doubled capacity otherwise. This preserves the existing array payload
  header and does not expand fractional, `NaN`, `Infinity`, or `-0` semantics.
- Added `fixtures/core-semantics/array-push-recursive-growth.ts`, a depth-3
  ABC451 reducer that previously produced the wrong length or trapped. It now
  matches Node under `iwasm`:

```text
114
```

- The full ABC451 fixture still builds, but the smallest official sample
  command remains blocked by a later allocation trap:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-large-number-child.wasm --host-deny
printf '10\n' | iwasm /tmp/abc451-d-large-number-child.wasm
```

Result:

```text
Exception: unreachable
```

`wasmtime` backtrace for the same wasm/input places the trap in the runtime
allocator (`wasm function 26`, `$alloc_heap`) called from recursive search
(`wasm function 49`). A temporary experiment with a larger memory maximum still
ended in `$alloc_heap`, so the remaining blocker is narrower than the original
out-of-bounds write but not yet safe to close.

2026-04-29 child `019dda13-74bf-7ec2-9146-e75ae64c098c` follow-up:

- Narrowed the allocator trap to free-list reuse under recursive
  string/array-allocation pressure. `$alloc_heap` previously reused a swept
  block wholesale for any smaller request, so a small string allocation could
  consume an entire large temporary array block until the next collection.
- Added allocator free-block splitting when the swept block can hold the
  requested payload plus a remainder header and one aligned payload slot. This
  keeps the existing GC header layout and does not change number semantics.
- Validated the allocator change against the focused array-growth regression
  and OOM smoke. The committed memory ceiling remains unchanged at 16 pages.
- This does not close the ABC451 sample path: with the committed 16-page
  memory ceiling, the official `10` sample still reaches `$alloc_heap`
  `Exception: unreachable`. A temporary 512-page ceiling plus block splitting
  delayed the failure to a long recursive-search run but still ended in
  `$alloc_heap`; a 2048-page experiment was stopped after exceeding one minute
  without producing sample output. The remaining decision is whether ABC451's
  live result set requires an accepted memory-limit policy change or a further
  GC/representation fix.

2026-04-29 child `019dda13-74bf-7ec2-9146-e75ae64c098c` reducer follow-up:

- Created implementation-ready issue 303 for the memory-limit policy decision.
- Reduced the remaining `$alloc_heap` trap to an ABC451 depth-7 live-set
  reducer. Under committed `MEMORY_MAX_PAGES=16`, depth 6 prints `12711` and
  depth 7 traps with `Exception: unreachable`. With only a temporary
  `MEMORY_MAX_PAGES=512` change, the same depth-7 reducer prints Node-matching
  `61002`.
- This points to a runtime memory-cap policy blocker rather than a remaining
  allocator free-list corruption bug. Issue 300 remains open until issue 303 is
  resolved and the three ABC451 official samples are verified.

2026-04-29 child `019dda50-d705-7782-bce9-c7e3e8dbf72d` memory-policy follow-up:

- Implemented issue 303's bounded memory policy by raising
  `Layout::MEMORY_MAX_PAGES` from 16 to 42 pages. The depth-7 ABC451 live-set
  reducer now prints Node-matching `61002` under `iwasm`; 40 pages still traps
  with `Exception: unreachable`, so 42 pages is the smallest confirmed cap for
  that reducer.
- Re-ran the ABC451 fixture build and all three official sample commands under
  the 42-page cap:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-memory-policy-child.wasm --host-deny
printf '10\n' | iwasm /tmp/abc451-d-memory-policy-child.wasm
printf '69\n' | iwasm /tmp/abc451-d-memory-policy-child.wasm
printf '1099898\n' | iwasm /tmp/abc451-d-memory-policy-child.wasm
```

Result:

```text
Exception: unreachable
Exception: unreachable
Exception: unreachable
```

- A 64-page trial also trapped on the smallest official sample input, so the
  remaining ABC451 sample blocker is still in the runtime allocation/GC path
  rather than the issue 303 depth-7 reducer cap itself. Issue 300 remains open.

2026-04-29 child `019dda5d-b728-7c33-8729-2b0edbbb94e9` post-memory triage:

- Reproduced the current official sample blocker after issue 303:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-post-memory-child.wasm --host-deny
printf '10\n' | iwasm /tmp/abc451-d-post-memory-child.wasm
```

Result:

```text
Exception: unreachable
```

`wasmtime run` places the trap in wasm function 26 (`$alloc_heap`) called from
recursive `search` (wasm function 49), before the later Set/spread/sort path.

- Split the next smaller implementation-ready reducer into issue 304. The
  depth-8 ABC451 search reducer prints Node `292743`; committed iwasm with
  `MEMORY_MAX_PAGES=42` traps in `$alloc_heap`, cap 128 also traps, and a
  temporary 256-page cap prints Node-matching `292743`.
- Depth 9, matching the official fixture search depth, prints Node `1404832`.
  Temporary 512/1024-page iwasm trials did not finish within 90 seconds during
  this triage, so issue 304 intentionally owns only depth 8. Issue 300 is
  blocked on issue 304 before official sample compatibility can be rechecked.

2026-04-29 child `304-depth8-live-set-20260429T182250Z` depth-8 follow-up:

- Closed issue 304 by raising `Layout::MEMORY_MAX_PAGES` from 42 to 185 pages.
  The depth-8 ABC451 live-set reducer now prints Node-matching `292743` under
  default emitted wasm memory policy. Cap trials classify the previous
  committed policy as too small for the depth-8 live result set: 184 pages
  traps in `$alloc_heap`, while 185 pages prints `292743`.
- Added `fixtures/core-semantics/abc451-depth8-live-set.ts` as regression
  coverage. The intentional OOM fixture still traps, preserving the explicit
  bounded-memory failure boundary.
- Issue 300 remains open for the next ABC451 blocker: official depth-9 sample
  compatibility is still unclaimed, and previous 512/1024-page depth-9 trials
  did not finish within 90 seconds.

2026-04-29 parent queue sync:

- Issue 304 is now closed, so issue 300 is unblocked for the next official
  ABC451 sample compatibility investigation and implementation slice.

2026-04-29 child `300-abc451-post-depth8-20260429T184750Z` official sample
recheck:

- Rebuilt `fixtures/atcoder/abc451-d-concat-power2.ts` under the committed
  post-304 runtime policy (`MEMORY_MAX_PAGES=185`):

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-post-depth8-300.wasm --host-deny
```

- Confirmed the official Node sample outputs for the rewritten fixture:

```text
10       -> 21
69       -> 328
1099898  -> 819264512
```

- Rechecked all three official samples under `iwasm` with 90 second command
  timeouts. All still trap before producing output:

```sh
timeout 90s sh -c "printf '10\n' | iwasm /tmp/abc451-d-post-depth8-300.wasm"
timeout 90s sh -c "printf '69\n' | iwasm /tmp/abc451-d-post-depth8-300.wasm"
timeout 90s sh -c "printf '1099898\n' | iwasm /tmp/abc451-d-post-depth8-300.wasm"
```

Result for each command:

```text
Exception: unreachable
```

- `wasmtime` backtrace for the smallest sample still places the trap in
  wasm function 26 (`$alloc_heap`) called from recursive search function 49,
  before the later Set/spread/sort output path.
- A temporary WAT-only memory-cap trial changed the symptom but did not prove
  compatibility:

```sh
tail -n +2 /tmp/abc451-d-post-depth8-300.raw.wat | perl -pe 's/\(memory \(export "memory"\) 2 185\)/(memory (export "memory") 2 512)/' > /tmp/abc451-d-post-depth8-300-cap512.wat
wat2wasm /tmp/abc451-d-post-depth8-300-cap512.wat -o /tmp/abc451-d-post-depth8-300-cap512.wasm
timeout 60s sh -c "printf '10\n' | iwasm /tmp/abc451-d-post-depth8-300-cap512.wasm"
```

Result:

```text
exit 124, no output
```

- This child did not raise the committed memory cap again because there is no
  bounded evidence that the official depth-9 sample path completes under the
  larger cap. Issue 305 now owns the next concrete blocker: depth-9 recursive
  search memory/performance isolation.

2026-04-29 child `305-abc451-depth9-budget-20260429T190850Z` depth-9
search-only isolation:

- Recreated the depth-9 search-only reducer from issue 305. Node prints
  `1404832`.
- Under committed `MEMORY_MAX_PAGES=185`, the reducer traps with
  `Exception: unreachable`.
- WAT-only memory-cap trials do not justify a committed memory-policy change:
  512 pages still traps after 1:11.00, 1024 pages times out after 90 seconds
  with no output, and 2048 pages also times out after 90 seconds with no
  output.
- A WAT-only trial that changed only the GC allocation-pressure threshold from
  64KiB to 1MiB while using 2048 pages also timed out after 90 seconds with no
  output.
- Issue 307 now owns the smaller next blocker: bounded allocation/GC
  performance telemetry for the depth-9 live-set path. Issue 300 remains open
  and blocked until that path produces completion-time evidence and the
  official samples match Node.

2026-04-29 child `307-abc451-telemetry-20260429T193520Z` depth-9 telemetry:

- Issue 307 classified the WAT-only 1024/2048-page depth-9 search-only timeout
  with bounded allocator/GC counters. Both caps reached the same diagnostic
  abort after 1,000,000 allocation calls:

```text
alloc_count: 1000000
allocated_block_bytes: 62700952
gc_collect_count: 834
gc_sweep_block_visits: 196941253
gc_sweep_freed_blocks: 19816743
heap_high_water_bytes: 20258192
```

- The matching 1024/2048-page counters show that this path is dominated by GC
  sweep cadence and linear heap scan cost under recursive array/string
  allocation pressure, not by the committed memory maximum alone. A no-telemetry
  WAT-only 2048-page control still produced no output before a 20 second
  timeout.
- Issue 300 remains open. No official ABC451 sample compatibility is claimed
  until the samples `10 -> 21`, `69 -> 328`, and `1099898 -> 819264512` match
  Node under committed runtime policy.
- Issue 308 owns the next implementation slice for GC cadence / sweep scan
  policy based on this telemetry.

2026-04-29 child `308-gc-cadence-20260429T195300Z` GC cadence attempt:

- Tested three small GC cadence policy candidates against issue 307 telemetry:
  free-list-first collection with post-GC retry, free-list reuse not counting
  as fresh allocation pressure, and a 128KiB GC threshold. None was committed
  because each candidate failed the required depth-8 regression with
  `Exception: unreachable`.
- The unchanged committed `MEMORY_MAX_PAGES=185` depth-9 search-only reducer
  still traps in `$alloc_heap`, so issue 300 compatibility is not claimed:

```sh
/usr/bin/time -f 'elapsed:%e' timeout 120s iwasm /tmp/abc451-search-depth-9-308.wasm
```

Result:

```text
Exception: unreachable
elapsed: 5.52
```

- WAT-only telemetry with memory max 1024 and 2048 pages produced identical
  counters at the 1,000,000 allocation diagnostic abort for the free-list-first
  experiment, improving the issue 307 baseline but failing depth-8 validation:

```text
alloc_count: 1000000
allocated_block_bytes: 62642176
gc_collect_count: 456
gc_sweep_block_visits: 96942634
gc_sweep_freed_blocks: 11591576
heap_high_water_bytes: 20487448
```

- Issue 308 remains open. Issue 300 remains open until the official samples
  `10 -> 21`, `69 -> 328`, and `1099898 -> 819264512` match Node under
  committed runtime policy.

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
