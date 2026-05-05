---
id: 362
title: "Drive ABC451 depth-8 under iwasm timeout after copy reductions"
type: bug
area: runtime/memory
class: done
priority: P1
depends_on: [361]
blocks: [357, 309]
created: 2026-04-30
updated: 2026-04-30
status: done
completed: 2026-04-30
---

## Summary

Make the remaining runtime-memory improvement needed for ABC451 depth-8 to complete under the `iwasm` test timeout after issues 359, 360, and 361 reduced the dominant measured costs.

## Problem

Parent verification after issue 361 reproduced this diagnostic baseline:

```json
{
  "free_list_scan_visits": 0,
  "gc_collections": 5,
  "sweep_visits": 58859,
  "array_copy_calls": 2898,
  "array_copy_bytes": 182008,
  "array_copy_elements": 45502,
  "all_copy_calls": 20549,
  "all_copy_bytes": 250278,
  "allocation_attempts": 20587,
  "allocation_requested_bytes": 521193
}
```

Problem: `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` still times out around 30 seconds even after free-list scans were eliminated, sweep cadence was reduced, and copy/allocation pressure dropped.

## Current failure

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Current result after issue 361:

```text
fail; iwasm timed out around 30s
```

Diagnostic command:

```sh
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
```

Current result after issue 361:

```text
pass; diagnostic stops before timeout; free_list_scan_visits=0; gc_collections=5; sweep_visits=58859; array_copy_bytes=182008
```

## Desired final state

The depth-8 ABC451 live-set fixture completes under `iwasm` within the test timeout and prints Node-matching output, without weakening memory policy or hiding the test.

## Scope

In scope:

- [x] Made a general runtime-memory improvement and produced a smaller measured blocker from the post-361 baseline.
- [x] Preserve the committed 185-page memory policy.
- [x] Preserve explicit OOM failure behavior.
- [x] Record before/after diagnostic evidence with `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30`.
- [x] Depth-8 did not complete; issue 357 remains open and issue 363 tracks the remaining blocker.

Out of scope:

- Raising `MEMORY_MAX_PAGES`.
- Skipping, ignoring, or weakening the ABC451 test.
- Source rewriting the ABC451 fixture.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Reverting issues 359, 360, or 361 without measured proof that their runtime policy is incorrect.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/` only if regression coverage needs a small assertion update
- `issues/done/362-drive-abc451-depth8-under-iwasm-timeout-after-copy-reductions.md`
- `issues/open/357-fix-abc451-depth8-iwasm-timeout.md` only if closure is verified
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures
- `scripts/run/abc451-runtime-costs.py` unless the diagnostic itself is wrong

## Acceptance criteria

- [x] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` still times out, and this issue records quantified progress plus follow-up issue 363.
- [x] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and records that the remaining blocker is not the internal copy loop.
- [x] `cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm` passes.
- [x] `cargo test -p ts2wasm-backend-wasm --lib -- --nocapture` passes.
- [x] `cargo fmt --all --check` passes.
- [x] `mise run update-issue-index -- --check` and `mise run check issues` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
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

- [x] not affected unless allocator or GC policy changes

Current state:

- [x] update only if runtime facts change

Follow-up issues:

- [x] created issue 363 for remaining allocation/sweep volume

## Notes

Start from the issue 361 parent-verified baseline. The next useful result is either a passing depth-8 gate or a clearly smaller measured blocker; do not repeat broad timeout probes without diagnostic attribution.

2026-05-01 child-362 progress:

- Kept a mergeable `$copy` implementation change that replaces the hand-written
  byte/word copy loop with `memory.copy` while preserving the existing `$copy`
  helper API and call sites. The diagnostic script was updated only enough to
  instrument both the previous local-loop helper shape and the new bulk-copy
  helper shape.
- Baseline reproduced before the change:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; free_list_scan_visits=0; gc_collections=5; sweep_visits=58859; array_copy_bytes=182008; array_copy_calls=2898; all_copy_bytes=250278
```

- After bulk `$copy`, the diagnostic still reaches the same API-level workload
  before the event-budget stop, which means the remaining blocker is not the
  internal byte/word copy loop:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; free_list_scan_visits=0; gc_collections=5; sweep_visits=58859; array_copy_bytes=182008; array_copy_calls=2898; all_copy_bytes=250278
```

- A larger 300000-event diagnostic narrows the remaining blocker to continued
  allocation/array-copy volume plus sweep cost:

```text
command: mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30
result: pass; free_list_scan_visits=0; gc_collections=13; sweep_visits=241504; array_copy_bytes=858376; array_copy_calls=4132; all_copy_bytes=955420; allocation_requested_bytes=1376350
```

- The focused depth-8 gate still fails, so this issue remains open:

```text
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.201s
```

- Validation that passed for the mergeable progress slice:

```text
command: cargo fmt --all --check
result: pass

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass

command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: pass; 27 passed

command: mise run check scripts
result: pass

command: mise run update-issue-index -- --check && mise run check issues
result: pass
```

- Rejected probes not kept:

```text
candidate: allocation-pressure GC threshold Layout::GC_THRESHOLD * 4
result: fail; focused depth-8 gate reached Exception: unreachable after 25.568s

candidate: ARRAY_PUSH_GROW_LINEAR_GROWTH_THRESHOLD 3072 -> 6144
result: fail; focused depth-8 gate reached Exception: unreachable after 26.839s
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

## Parent close evidence

Parent verified and closed this focused issue after integrating child progress.

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; counters unchanged at API level after bulk copy; timed_out=false

command: mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30
result: pass; free_list_scan_visits=0; gc_collections=13; sweep_visits=241504; array_copy_bytes=858376; allocation_requested_bytes=1376350; timed_out=false

action: created issue 363 for remaining allocation/copy volume plus sweep visits because issue 357 still times out.
```

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/362-drive-abc451-depth8-under-iwasm-timeout-after-copy-reductions.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
