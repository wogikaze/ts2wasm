---
id: 309
title: "Reduce ABC451 depth-9 live allocation shape"
type: feature
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: []
blocks: [308, 300]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement the next smaller ABC451 depth-9 runtime slice after issue 308's GC
cadence and free-list policy work: reduce the live allocation shape or
allocation size that still reaches the committed 185-page cap.

Problem: The depth-9 search-only reducer now fails at the explicit
remaining-page guard with `size=6140`, `block_size=6160`, `new_heap=12126520`,
`memory_pages=185`, `needed_pages=1`, `remaining_pages=0`, and
`gc_free_list_max_body_size=3584`.

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
