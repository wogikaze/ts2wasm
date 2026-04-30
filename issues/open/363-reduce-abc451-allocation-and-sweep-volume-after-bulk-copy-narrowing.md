---
id: 363
title: "Reduce ABC451 allocation and sweep volume after bulk copy narrowing"
type: bug
area: runtime/memory
class: blocked
priority: P1
depends_on: [362, 364]
blocks: [357, 309]
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Reduce the remaining ABC451 depth-8 allocation and sweep volume after issue 362 showed the timeout is not caused by the internal `$copy` byte/word loop.

## Problem

Parent verification after issue 362 reproduced these diagnostics:

```json
{
  "event_budget": 100000,
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

At a larger diagnostic budget:

```json
{
  "event_budget": 300000,
  "free_list_scan_visits": 0,
  "gc_collections": 13,
  "sweep_visits": 241504,
  "array_copy_calls": 4132,
  "array_copy_bytes": 858376,
  "array_copy_elements": 214594,
  "all_copy_calls": 29222,
  "all_copy_bytes": 955420,
  "allocation_attempts": 29261,
  "allocation_requested_bytes": 1376350
}
```

Problem: the depth-8 fixture still times out, and the remaining measured blocker is high allocation/copy volume plus repeated sweeping, not free-list scanning or the internal copy loop.

## Current failure

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Current result after issue 362:

```text
fail; iwasm timed out around 30s
```

Diagnostic commands:

```sh
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30
```

Current result after issue 362:

```text
pass; diagnostics stop before timeout; free_list_scan_visits=0; remaining pressure is allocation/copy volume and sweep visits
```

## Desired final state

The depth-8 ABC451 live-set fixture completes under `iwasm` within the test timeout and prints Node-matching output, without weakening memory policy or hiding the test.

## Scope

In scope:

- [ ] Reduce allocation attempts, allocation requested bytes, array copy volume, or sweep visits using a general runtime-memory improvement.
- [ ] Preserve the committed 185-page memory policy.
- [ ] Preserve explicit OOM failure behavior.
- [ ] Record before/after diagnostic evidence with `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` and, if useful, `--event-budget 300000`.
- [ ] If the depth-8 fixture now completes, close issue 357 in the same merge or request parent close after verification.

Out of scope:

- Raising `MEMORY_MAX_PAGES`.
- Skipping, ignoring, or weakening the ABC451 test.
- Source rewriting the ABC451 fixture.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Reverting issues 359, 360, 361, or 362 without measured proof that their runtime policy is incorrect.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/` only if regression coverage needs a small assertion update
- `issues/open/363-reduce-abc451-allocation-and-sweep-volume-after-bulk-copy-narrowing.md`
- `issues/open/357-fix-abc451-depth8-iwasm-timeout.md` only if closure is verified
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures
- `scripts/run/abc451-runtime-costs.py` unless the diagnostic itself is wrong

## Acceptance criteria

- [ ] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` passes, or the issue records quantified progress and a smaller remaining blocker.
- [ ] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and records changed pressure versus the issue 362 parent-verified baseline, or records a justified replacement metric if the runtime structure changes.
- [ ] `cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm` passes.
- [ ] `cargo test -p ts2wasm-backend-wasm --lib -- --nocapture` passes if backend runtime code changes.
- [ ] `cargo fmt --all --check` passes.
- [ ] `mise run update-issue-index -- --check` and `mise run check issues` pass.

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

- [x] create only if this slice isolates a smaller remaining blocker

## Notes

Issue 362 proved bulk `memory.copy` preserves the same API-level diagnostic counters at 100000 events, so the next useful change should reduce the amount of allocation/copy/sweep work rather than only replacing the implementation of a copy primitive.

## Child attempt evidence: 2026-05-01

Status: `BLOCKED`.

This child did not keep runtime code changes because each tested candidate either produced no diagnostic improvement or changed the focused gate from timeout to OOM/trap.

Baseline reproduced:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; free_list_scan_visits=0; gc_collections=5; sweep_visits=58859; array_copy_calls=2898; array_copy_bytes=182008; allocation_attempts=20587; allocation_requested_bytes=521193
```

Rejected candidates:

```text
candidate: top-of-heap ArrayPushGrow memory.grow extension under MEMORY_MAX_PAGES=185
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; no counter delta from baseline
command: mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30
result: pass; no counter delta from issue-362 300000-event baseline
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.212s
```

```text
candidate: allocation-pressure GC cadence Layout::GC_THRESHOLD * 3
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; sweep_visits 58859 -> 51888, but allocation/copy volume increased
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; Exception: unreachable after 29.408s
```

```text
candidate: allocation-pressure GC cadence Layout::GC_THRESHOLD * 2 + Layout::GC_THRESHOLD / 2
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; sweep_visits 58859 -> 57445, but allocation/copy volume increased
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; Exception: unreachable after 29.704s
```

```text
candidate: immutable string concat empty-string fast path for "" + s and s + ""
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; counters worsened: allocation_attempts 20587 -> 20590, all_copy_calls 20549 -> 20551
```

```text
candidate: ARRAY_PUSH_GROW_LINEAR_GROWTH_THRESHOLD 3072 -> 4096
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; no counter delta from baseline
command: mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30
result: pass; no counter delta from issue-362 300000-event baseline
```

Remaining blocker:

```text
The existing diagnostic identifies aggregate allocation/copy/sweep pressure, but the safe runtime-policy candidates above do not isolate a mergeable reduction that preserves the focused depth-8 gate. The next slice should add callsite/category attribution for allocation and copy volume, or target a proven high-volume allocation site with a smaller acceptance gate.
```

Validation for evidence-only blocker update:

```text
cargo fmt --all --check: pass
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30: pass; baseline counters reproduced
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm: fail; iwasm timed out after 30.212s
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm: pass
cargo test -p ts2wasm-backend-wasm --lib -- --nocapture: pass; 27 passed
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

## Parent blocker classification

2026-04-30 parent classification:

- Class changed to `blocked` because the child evidence shows broad allocator probes are no longer productive without allocation/copy attribution.
- Follow-up issue 364 tracks the diagnostic attribution needed to identify the next safe implementation target.

## Attribution diagnostic result

Issue 364 closed the diagnostic gap and identified array growth as the next focused target:

```text
100000 events: allocation array_growth=362976 bytes/2648 calls; copy array_growth=181008 bytes/2648 calls
300000 events: allocation array_growth=1158708 bytes/3771 calls; copy array_growth=856928 bytes/3770 calls
```

Follow-up issue 365 tracks the smaller implementation slice for reducing array-growth allocation/copy pressure. Keep this broad issue blocked until issue 365 produces a mergeable runtime-memory change or records a smaller blocker.

## Issue 365 helper-level blocker result

Issue 365 child v4 tested helper-local non-top `ArrayPushGrow` reductions after issue 367 extracted `$array_push_grow`.

Result:

```text
status: BLOCKED
evidence: adjacent free-block expansion and zero-length copy skipping were neutral; bounded growth-factor probes violated either `allocation_requested_bytes` or `sweep_visits` at 100000 events
next: representation-level append strategy or deeper attribution for why non-top result arrays miss heap-top growth
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
