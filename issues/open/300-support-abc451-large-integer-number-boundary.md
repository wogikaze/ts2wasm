---
id: 300
title: "Support ABC451 large integer number boundary"
type: feature
area: runtime
class: blocked
priority: P1
depends_on: [304]
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
