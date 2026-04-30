---
id: 361
title: "Reduce ABC451 array copy pressure after GC cadence fix"
type: bug
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: [360]
blocks: [357, 309]
created: 2026-04-30
updated: 2026-05-01
---

## Summary

Reduce the remaining ABC451 depth-8 timeout pressure after issue 359 removed free-list scans and issue 360 reduced sweep cadence.

## Problem

Parent verification after issue 360 reproduced this diagnostic baseline:

```json
{
  "free_list_scan_visits": 0,
  "gc_collections": 6,
  "sweep_visits": 62313,
  "array_copy_calls": 3119,
  "array_copy_bytes": 311932,
  "array_copy_elements": 77983,
  "all_copy_calls": 18821,
  "all_copy_bytes": 372673,
  "allocation_attempts": 18860,
  "allocation_requested_bytes": 691283
}
```

Problem: the depth-8 fixture still times out after free-list scans and some sweep pressure were reduced. The remaining measured pressure is copy/allocation work plus residual sweep visits.

## Current failure

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Current result after issue 360:

```text
fail; iwasm timed out around 30s
```

Diagnostic command:

```sh
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
```

Current result after issue 360:

```text
pass; diagnostic stops before timeout; free_list_scan_visits=0; gc_collections=6; sweep_visits=62313; array_copy_calls=3119
```

## Desired final state

The depth-8 ABC451 live-set fixture completes under `iwasm` within the test timeout and prints Node-matching output, without weakening memory policy or hiding the test.

## Scope

In scope:

- [ ] Reduce array/copy pressure, allocation count, or residual sweep pressure using a general runtime-memory improvement.
- [ ] Preserve the committed 185-page memory policy.
- [ ] Preserve explicit OOM failure behavior.
- [ ] Record before/after diagnostic evidence with `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30`.
- [ ] If the depth-8 fixture now completes, close issue 357 in the same merge or request parent close after verification.

Out of scope:

- Raising `MEMORY_MAX_PAGES`.
- Skipping, ignoring, or weakening the ABC451 test.
- Source rewriting the ABC451 fixture.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Reverting issue 359 or 360 without measured proof that their runtime policy is incorrect.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/` only if regression coverage needs a small assertion update
- `issues/open/361-reduce-abc451-array-copy-pressure-after-gc-cadence-fix.md`
- `issues/open/357-fix-abc451-depth8-iwasm-timeout.md` only if closure is verified
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures
- `scripts/run/abc451-runtime-costs.py` unless the diagnostic itself is wrong

## Acceptance criteria

- [ ] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and records reduced remaining pressure versus the issue 360 parent-verified baseline, or records a justified replacement metric if the runtime structure changes.
- [ ] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` passes, or the issue records quantified progress and the remaining blocker.
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

Issue 357 rejected earlier broad array-growth probes that either still timed out or trapped. Start from the issue 360 parent-verified baseline and keep any array/copy change tied to diagnostic evidence plus OOM validation.

2026-05-01 child-361 progress:

- Implemented an aligned `$copy` fast path that copies 4-byte chunks when source, destination, and length are all 4-byte aligned, with the original byte loop retained for unaligned copies.
- Implemented top-of-heap `ArrayPushGrow` extension for arrays whose current block is exactly at `$heap` and whose aligned expanded body already fits committed memory. The fallback remains the existing `alloc_heap` plus `$copy` path.
- Preserved the 185-page memory policy and OOM behavior.

Before diagnostic, from the issue-360 parent baseline:

```json
{
  "free_list_scan_visits": 0,
  "gc_collections": 6,
  "sweep_visits": 62313,
  "array_copy_calls": 3119,
  "array_copy_bytes": 311932,
  "array_copy_elements": 77983,
  "all_copy_calls": 18821,
  "all_copy_bytes": 372673,
  "allocation_attempts": 18860,
  "allocation_requested_bytes": 691283
}
```

After diagnostic:

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

Quantified improvement:

- `array_copy_bytes`: `311932 -> 182008`
- `array_copy_elements`: `77983 -> 45502`
- `allocation_requested_bytes`: `691283 -> 521193`
- `gc_collections`: `6 -> 5`
- `sweep_visits`: `62313 -> 58859`

Remaining blocker:

```text
command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; iwasm timed out after 30.222s
date: 2026-05-01
```

Validation:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-01

command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; counters recorded above
date: 2026-05-01

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass
date: 2026-05-01

command: cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
result: pass; 27 passed
date: 2026-05-01

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK
date: 2026-05-01

command: mise run check issues
result: pass; issues/index.md queue OK; check_issue_health OK
date: 2026-05-01
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
