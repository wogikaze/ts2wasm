---
id: 309
title: "Reduce ABC451 depth-9 live allocation shape"
type: feature
area: runtime/memory
class: blocked
priority: P1
depends_on: []
blocks: [308, 300]
created: 2026-04-29
updated: 2026-05-01
---

## Summary

Implement the next smaller ABC451 depth-9 runtime slice after issue 308's GC
cadence and free-list policy work: reduce the live allocation shape or
allocation size that still reaches the committed 185-page cap.

Problem: The depth-9 search-only reducer now fails at the explicit
remaining-page guard with `size=24572`, `block_size=24592`,
`new_heap=12139256`, `memory_pages=185`, `needed_pages=1`,
`remaining_pages=0`, and `gc_free_list_max_body_size=12392` after issue 310's
backend-temp root narrowing.

## Current failure

Reproduction shape from issue 308:

```sh
node /tmp/abc451-depth9-live-set-308.ts
cargo run -q -- build /tmp/abc451-depth9-live-set-308.ts -o /tmp/abc451-depth9-live-set-308-precap.wasm --host-deny
/usr/bin/time -f 'elapsed:%e' timeout 90s iwasm /tmp/abc451-depth9-live-set-308-precap.wasm
WASMTIME_BACKTRACE_DETAILS=1 /usr/bin/time -f 'elapsed:%e' timeout 90s wasmtime run /tmp/abc451-depth9-live-set-308-precap.wasm
```

Current result:

```text
node: pass; stdout 1404832
build: pass
iwasm: Exception: unreachable under committed 185-page policy
wasmtime: trap in $alloc_heap at the explicit remaining-page guard
```

Latest bounded telemetry:

```text
size=6140
block_size=6160
new_heap=12126520
memory_pages=185
needed_pages=1
remaining_pages=0
gc_free_list_max_body_size=3584
```

This means the allocator asks for one more page after GC, free-list scan, and
tail-trim reuse, while the largest known swept free block is too small for the
aligned request.

## Desired final state

The depth-9 search-only reducer either completes under the committed runtime
policy and prints Node-matching `1404832`, or a smaller proven blocker is split
with reducer evidence showing why allocation size, live set, or representation
work must happen first.

## Scope

In scope:

- [ ] Reduce live allocation pressure for recursive array/string search output
      without source-specific ABC451 rewrites.
- [ ] Reduce allocation size for the failing 6,160-byte request when this can
      be proven semantics-preserving.
- [ ] Add a focused reducer fixture or backend runtime contract for the chosen
      policy.
- [ ] Preserve depth-8 ABC451 live-set output and explicit OOM trap behavior.
- [ ] Update issues 308 and 300 with completion or blocker evidence.

Out of scope:

- Raising `MEMORY_MAX_PAGES` without official sample completion evidence.
- BigInt/private-class/spread/eval implementation work.
- Parser/frontend feature work unrelated to the reduced allocation shape.
- Problem-specific ABC451 source rewriting or generated tables.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `fixtures/atcoder/`
- `docs/14-runtime-abi.md` if runtime memory or GC policy changes
- `current-state.md` if runtime facts change
- `issues/open/300-support-abc451-large-integer-number-boundary.md`
- `issues/open/308-implement-abc451-depth9-gc-cadence-policy.md`
- `issues/open/309-reduce-abc451-depth9-live-allocation-shape.md`
- `issues/index.md`

Do not touch:

- BigInt runtime/ABI files
- private-class, spread, or eval files
- parser/frontend files unless a separate parent assignment explicitly expands
  scope
- problem-specific source rewrite hooks

## Acceptance criteria

- [ ] The depth-9 search-only reducer advances beyond the current
      remaining-page guard shape, or a narrower reducer demonstrates the next
      blocker with exact allocation/live-set evidence.
- [ ] If the depth-9 reducer completes, it prints Node-matching `1404832`.
- [ ] `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` remains
      passing.
- [ ] `oom_alloc_check_must_fail_iwasm` remains passing if runtime memory or GC
      policy changes.
- [ ] Issue 300 remains open unless official ABC451 sample inputs `10`, `69`,
      and `1099898` match Node under committed runtime policy.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
node /tmp/abc451-depth9-live-set-308.ts
cargo run -q -- build /tmp/abc451-depth9-live-set-308.ts -o /tmp/abc451-depth9-live-set-309.wasm --host-deny
timeout 90s iwasm /tmp/abc451-depth9-live-set-309.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` if runtime memory or GC policy changes

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` if runtime facts change

Follow-up issues:

- [ ] none
- [ ] created/updated if this slice proves a smaller blocker

## Notes

Issue 308 already committed GC cadence, free-list scan, last-chance GC,
tail-trim, post-GC bump recompute, and remaining-page guard slices. Do not
repeat those policy experiments unless new reducer evidence explains why the
previous depth-8/OOM validation result no longer applies.

2026-04-29 child `309-depth9-live-allocation-20260429T2312Z` blocker evidence:

- Tested a backend root-liveness reduction for function activation frames:
  clearing direct block-scoped `let` locals declared in `for` bodies after each
  iteration body, with a narrower variant that only cleared direct `let` locals
  initialized from user-call results. This was not committed because both
  variants regressed required validation
  `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` with
  `Exception: unreachable`.
- The broad root-clearing experiment did advance the depth-9 search-only
  reducer beyond the previous remaining-page guard shape, but still trapped
  under the committed 185-page cap. WAT-only instrumentation at the explicit
  remaining-page guard reported:

```text
size=3068
block_size=3088
new_heap=12126704
memory_pages=185
needed_pages=1
remaining_pages=0
gc_free_list_max_body_size=1592
```

Previous issue-309 start shape was:

```text
size=6140
block_size=6160
new_heap=12126520
memory_pages=185
needed_pages=1
remaining_pages=0
gc_free_list_max_body_size=3584
```

- Rejected allocation-capacity experiments:
  exact-fit large-array growth advanced past the old guard window but timed out
  at 90s under `iwasm`; a 9/8 large-array growth factor regressed to an earlier
  remaining-page guard with `size=874592`, `block_size=874608`,
  `new_heap=12751744`, `memory_pages=182`, `needed_pages=13`,
  `remaining_pages=3`, and `gc_free_list_max_body_size=269336`.
- No runtime implementation from these experiments was left in the tree.
- Created issue 310 for the next executable slice: make activation-frame root
  liveness narrowing safe for the depth-8 fixture before retrying this
  depth-9 allocation-shape reduction.
- Issue 300 remains open. No official ABC451 sample compatibility is claimed.

2026-04-30 issue 310 safe root-liveness slice:

- Direct block-scoped user-local root clearing remains unsafe for depth-8. A
  reproduction that cleared direct `let` locals after each `for` body trapped
  `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` at the
  explicit remaining-page guard with `size=6140`, `block_size=6160`,
  `new_heap=12129576`, `memory_pages=185`, `needed_pages=1`,
  `remaining_pages=0`, and `gc_free_list_max_body_size=2088`.
- The committed safe narrowing clears backend-owned temporary root slots at
  statement boundaries but leaves user locals rooted until reassignment or
  frame pop. Required depth-8/OOM validations pass.
- The depth-9 reducer still traps under the committed cap, now at `size=24572`,
  `block_size=24592`, `new_heap=12139256`, `memory_pages=185`,
  `needed_pages=1`, `remaining_pages=0`, and
  `gc_free_list_max_body_size=12392`.

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

2026-04-30 child-309 live-shape validation note:

- Found the assigned worktree could not compile `ArrayPushGrow` before runtime
  validation: `expr_emit.rs` used a Rust format placeholder
  `{array_push_grow_linear_growth_threshold}` without a named format argument,
  and the emitted WAT for `ArrayPushGrow` had unbalanced parentheses around the
  generated `local.set`, capacity guard, and fast-path element store.
- Fixed only the backend WAT-shape defects in the local candidate so the
  reducer can build and runtime evidence can be collected.
- The current small-shape policy remains exact-fit growth after capacity 3072.
  With `$copy` changed from the byte loop to `memory.copy`, backend unit tests
  pass, but the required depth-8 iwasm gate still times out:

```text
command: cargo run -q -- build fixtures/core-semantics/abc451-depth8-live-set.ts -o /tmp/abc451-depth8-live-set-309.wasm --host-deny && /usr/bin/time -f 'elapsed:%e' timeout 35s iwasm /tmp/abc451-depth8-live-set-309.wasm
result: fail; iwasm timeout; elapsed:35.01
```

- Growth-policy probes show no safe mergeable policy in this slice:

```text
exact-fit after 3072 + byte copy: depth-8 timeout at 35.01s
exact-fit after 3072 + memory.copy: depth-8 timeout at 35.01s
fixed +16 slots after 3072 + byte copy: depth-8 timeout at 35.01s
fixed +32 slots after 3072 + byte copy: depth-8 traps with Exception: unreachable after 23.25s
fixed +128 slots after 3072 + byte copy: depth-8 traps with Exception: unreachable after 9.69s
fixed +1024 slots after 3072 + byte copy: depth-8 traps with Exception: unreachable after 5.01s
9/8 growth after 3072 + memory.copy: depth-8 traps with Exception: unreachable after 6.45s
17/16 growth after 3072 + memory.copy: depth-8 traps with Exception: unreachable after 6.50s
33/32 growth after 3072 + memory.copy: depth-8 traps with Exception: unreachable after 4.74s
```

- This narrows the next blocker: reducing the depth-9 live allocation shape via
  array growth alone conflicts with the required depth-8 gate. Exact-fit keeps
  allocation shape small enough but needs a stronger copy/representation change
  to meet runtime budget; any tested slack/geometric capacity reduces copy
  pressure but raises live allocation/capacity enough to violate the 185-page
  cap on depth-8 before depth-9 can be claimed.

Required gate result for the final local candidate (`memory.copy` plus exact-fit
large-array growth):

```text
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.235s
```

2026-05-01 child `child/309-abc451-depth9-20260501-061232` blocker evidence:

- Used issue 385's deterministic counter evidence (`sweep_visits=58859`,
  `free_list_scan_visits=0`, `all_copy_calls=20549` at the
  `abc451-runtime-costs --event-budget 100000` diagnostic budget) to test a
  growth-boundary-only allocation-pressure GC policy. The candidate delayed
  allocation-pressure GC until the bump allocation would exceed currently
  committed memory, while preserving max-cap last-chance GC and the explicit
  OOM guard.
- The candidate did reduce the targeted sweep counter, but it was not
  mergeable because the required depth-8 runtime gate still timed out and
  array-growth allocation/copy volume increased:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-01

command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; diagnostic_stop=true; timed_out=false
date: 2026-05-01
counter delta versus issue 385:
  gc_collections: 5 -> 3
  sweep_visits: 58859 -> 40554
  free_list_scan_visits: 0 -> 0
  allocation_attempts: 20587 -> 29741
  allocation_requested_bytes: 521193 -> 1485703
  array_copy_bytes: 182008 -> 1012776
  alloc_array_growth_bytes: 362976 -> 1264808

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass
date: 2026-05-01

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.233s
date: 2026-05-01

command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: fail; allocator/GC contract tests passed, but pre-existing/out-of-scope
  runtime_link_plan::tests::bigint_runtime_arithmetic_selects_helper_deps failed
  in forbidden BigInt area
date: 2026-05-01
```

- Conclusion: issue 385's sweep-dominance signal is real, but simply reducing
  headroom-triggered sweeps shifts pressure into array-growth allocation/copy
  and does not recover the depth-8 timeout. The next smaller blocker is a
  representation or lifetime slice that reduces live/copy volume without
  increasing `allocation_requested_bytes`, not another GC-cadence-only policy.
- No runtime implementation from this rejected candidate was left in the tree.

2026-05-01 child `child/309-depth9-live-allocation-20260501-065708` blocker evidence:

- Tested a heap-tail `ArrayPushGrow` allocation-shape candidate: when a local
  array payload was already at the current heap end, grow the array in place
  across bounded `memory.grow` instead of allocating a second array and copying
  old elements. This directly targeted the issue-309 old/new live-array
  overlap without raising `MEMORY_MAX_PAGES`.
- The first candidate preserved the explicit OOM smoke but was not mergeable
  because the required depth-8 gate timed out:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-01

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass
date: 2026-05-01

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.310s
date: 2026-05-01
```

- A second variant restored allocator-style allocation-pressure GC before the
  in-place page grow so the tail-growth path would not starve collections. It
  still failed the same required depth-8 gate:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-01

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.242s
date: 2026-05-01
```

- The required backend lib command was also rerun while the helper contract
  test covered the candidate WAT shape. The candidate-specific backend tests
  passed, but the command still failed on the existing forbidden/out-of-scope
  BigInt runtime-link assertion already noted in this issue:

```text
command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: fail; 30 passed, 1 failed; failing test was runtime_link_plan::tests::bigint_runtime_arithmetic_selects_helper_deps
date: 2026-05-01
```

- Conclusion: avoiding old/new live-array overlap only at heap-tail page-growth
  boundaries is not sufficient; under the required depth-8 timeout it behaves
  like the earlier array-growth probes and remains non-mergeable. The next
  smaller blocker is still a representation or lifetime reduction that lowers
  live/copy volume without making depth-8 exceed its runtime budget.
- No runtime implementation from these rejected candidates was left in the
  tree. Issue 300 remains open; no official ABC451 sample compatibility is
  claimed.
- After backing out the runtime candidates, the assigned worktree still timed
  out on the same required depth-8 gate at `30.311s`, while
  `cargo fmt --all --check`,
  `cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm`,
  `mise run update-issue-index -- --check`, and `mise run check issues`
  passed. This confirms the child branch has no mergeable runtime change from
  this attempt.
