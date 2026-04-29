---
id: 305
title: "Support ABC451 depth-9 search budget"
type: feature
area: runtime/memory
class: done
priority: P1
depends_on: []
blocks: [307]
created: 2026-04-29
updated: 2026-04-29
status: done
completed: 2026-04-29
---

## Summary

Isolate and fix the ABC451 depth-9 recursive search runtime blocker after the
depth-8 live-set reducer from issue 304.

Problem: The official ABC451 rewritten fixture still traps in `$alloc_heap`
with the committed 185-page memory cap, while a temporary 512-page cap does not
produce even the smallest official sample output within 60 seconds.

## Current failure

Committed policy reproduction:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-post-depth8-300.wasm --host-deny
timeout 90s sh -c "printf '10\n' | iwasm /tmp/abc451-d-post-depth8-300.wasm"
```

Current result:

```text
Exception: unreachable
```

`wasmtime run` places the trap in wasm function 26 (`$alloc_heap`) called from
recursive search function 49.

Temporary cap trial:

```sh
cargo run -q -- dump --wat fixtures/atcoder/abc451-d-concat-power2.ts > /tmp/abc451-d-post-depth8-300.raw.wat
tail -n +2 /tmp/abc451-d-post-depth8-300.raw.wat | perl -pe 's/\(memory \(export "memory"\) 2 185\)/(memory (export "memory") 2 512)/' > /tmp/abc451-d-post-depth8-300-cap512.wat
wat2wasm /tmp/abc451-d-post-depth8-300-cap512.wat -o /tmp/abc451-d-post-depth8-300-cap512.wasm
timeout 60s sh -c "printf '10\n' | iwasm /tmp/abc451-d-post-depth8-300-cap512.wasm"
```

Current result:

```text
exit 124, no output
```

Node prints `1404832` for the depth-9 search-only live-set count, and the
official sample outputs for the rewritten fixture are:

```text
10       -> 21
69       -> 328
1099898  -> 819264512
```

## Desired final state

The runtime either executes the ABC451 depth-9 recursive search path under
`iwasm` with a justified bounded memory policy, or a smaller verified blocker
is split with evidence showing why depth-9 cannot be safely fixed as one slice.

## Scope

In scope:

- [x] Isolate whether the remaining blocker is memory capacity, GC retention,
      allocation strategy, algorithmic overhead from current lowering, or a
      later runtime path after search returns.
- [x] Prefer a committed depth-9 search-only reducer if it can complete within
      a practical focused-test timeout.
- [x] If changing memory policy, prove the smallest justified cap with reducer
      evidence and preserve the intentional OOM boundary.
- [x] If changing allocation/GC behavior, add focused Node/iwasm regression
      coverage.

Out of scope:

- BigInt runtime or ABI changes.
- Problem-specific source rewrite hooks or replacing ABC451 with a generated
  table/DP implementation.
- Claiming full issue 300 or issue 294 compatibility without the official
  sample inputs `10`, `69`, and `1099898`.

## Affected paths

Expected:

- `crates/runtime-abi/src/layout.rs`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md` if memory policy changes
- `current-state.md` if supported runtime facts change
- `issues/open/300-support-abc451-large-integer-number-boundary.md`
- `issues/index.md`

Do not touch:

- BigInt runtime/ABI files
- issue 302 eval files
- problem-specific source rewrite hooks
- unrelated web/report artifacts

## Acceptance criteria

- [x] The next depth-9 blocker is classified with bounded commands that do not
      rely on unbounded sample execution.
- [x] If implementation changes are made, focused Node/iwasm regression
      coverage is added for the fixed reducer or runtime path.
- [x] If memory policy changes, `oom_alloc_check_must_fail_iwasm` still passes
      and `docs/14-runtime-abi.md` is updated.
- [x] Issue 300 is updated with the new evidence and remains open until all
      official sample outputs match Node.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-depth9.wasm --host-deny
timeout 90s sh -c "printf '10\n' | iwasm /tmp/abc451-d-depth9.wasm"
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
- [x] created/updated if a smaller allocator/GC/lowering blocker is isolated

## Notes

Do not increase `MEMORY_MAX_PAGES` solely because the 185-page cap traps. The
depth-9 path also needs completion-time evidence; the 512-page temporary trial
timed out after 60 seconds without output for sample input `10`.

## Completion evidence

Commits:

- `1e77d94b`

Validation result:

```text
command: node /tmp/abc451-search-depth-9-305.ts
result: pass; stdout 1404832
date: 2026-04-29

command: cargo run -q -- build /tmp/abc451-search-depth-9-305.ts -o /tmp/abc451-search-depth-9-305.wasm --host-deny && timeout 45s iwasm /tmp/abc451-search-depth-9-305.wasm
result: fail; iwasm trapped with Exception: unreachable under committed MEMORY_MAX_PAGES=185
date: 2026-04-29

command: WAT-only memory max 512 pages, timeout 90s iwasm /tmp/abc451-search-depth-9-305-cap-512.wasm
result: fail; Exception: unreachable after 1:11.00, maxrss 105248KB
date: 2026-04-29

command: WAT-only memory max 1024 pages, timeout 90s iwasm /tmp/abc451-search-depth-9-305-cap-1024.wasm
result: fail; timeout 124 after 1:30.01, no output, maxrss 110716KB
date: 2026-04-29

command: WAT-only memory max 2048 pages, timeout 90s iwasm /tmp/abc451-search-depth-9-305-cap-2048.wasm
result: fail; timeout 124 after 1:30.01, no output, maxrss 110012KB
date: 2026-04-29

command: WAT-only GC threshold 1MiB with memory max 2048 pages, timeout 90s iwasm /tmp/abc451-search-depth-9-305-th-1048576.wasm
result: fail; timeout 124 after 1:30.01, no output, maxrss 103768KB
date: 2026-04-29
```

Remaining risks:

- The bounded depth-9 search-only reducer isolates the next blocker as
  completion-time/runtime allocation performance under large live-set pressure,
  not a justified memory-cap-only implementation slice. 512 pages still traps,
  while 1024/2048 pages do not complete within 90 seconds. A WAT-only 1MiB GC
  threshold trial also timed out, so issue 307 owns the next smaller
  instrumentation/performance slice before any committed memory or GC policy
  change.
