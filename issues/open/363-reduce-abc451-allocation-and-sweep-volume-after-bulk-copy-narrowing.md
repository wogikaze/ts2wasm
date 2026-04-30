---
id: 363
title: "Reduce ABC451 allocation and sweep volume after bulk copy narrowing"
type: bug
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: [362]
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
