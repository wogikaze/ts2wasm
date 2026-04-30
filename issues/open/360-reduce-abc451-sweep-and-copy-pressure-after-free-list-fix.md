---
id: 360
title: "Reduce ABC451 sweep and copy pressure after free-list fix"
type: bug
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: [359]
blocks: [357, 309]
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Reduce the remaining ABC451 depth-8 timeout pressure after issue 359 eliminated allocator free-list scans.

## Problem

Issue 359 changed allocation to avoid early free-list scans and parent verification reproduced:

```json
{
  "free_list_scan_visits": 0,
  "gc_collections": 7,
  "sweep_visits": 68498,
  "array_copy_calls": 2614,
  "array_copy_bytes": 162760,
  "all_copy_calls": 15728,
  "all_copy_bytes": 213487,
  "allocation_attempts": 15767,
  "allocation_requested_bytes": 468361
}
```

Problem: `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` still times out around 30 seconds after free-list scan removal. The remaining measured pressure is sweep frequency/live-set retention and copy work.

## Current failure

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Current result after issue 359:

```text
fail; iwasm timed out around 30s
```

Diagnostic command:

```sh
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
```

Current result after issue 359:

```text
pass; diagnostic stops before timeout; free_list_scan_visits=0; gc_collections=7; sweep_visits=68498; array_copy_calls=2614
```

## Desired final state

The depth-8 ABC451 live-set fixture completes under `iwasm` within the test timeout and prints Node-matching output, without weakening memory policy or hiding the test.

## Scope

In scope:

- [ ] Reduce sweep frequency, sweep visits, live-set retention cost, or array/copy pressure using a general runtime-memory improvement.
- [ ] Preserve the committed 185-page memory policy.
- [ ] Preserve explicit OOM failure behavior.
- [ ] Record before/after diagnostic evidence with `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30`.
- [ ] If the depth-8 fixture now completes, close issue 357 in the same merge or request parent close after verification.

Out of scope:

- Raising `MEMORY_MAX_PAGES`.
- Skipping, ignoring, or weakening the ABC451 test.
- Source rewriting the ABC451 fixture.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Free-list scan optimization already completed by issue 359 unless the issue 359 change is proven incorrect.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/` only if regression coverage needs a small assertion update
- `issues/open/360-reduce-abc451-sweep-and-copy-pressure-after-free-list-fix.md`
- `issues/open/357-fix-abc451-depth8-iwasm-timeout.md` only if closure is verified
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures
- `scripts/run/abc451-runtime-costs.py` unless the diagnostic itself is wrong

## Acceptance criteria

- [ ] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and records reduced remaining pressure versus the issue 359 parent-verified baseline, or records a justified replacement metric if the runtime structure changes.
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

Start from the issue 359 parent-verified baseline. Avoid broad timeout probes without diagnostic attribution.

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
