---
id: 362
title: "Drive ABC451 depth-8 under iwasm timeout after copy reductions"
type: bug
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: [361]
blocks: [357, 309]
created: 2026-04-30
updated: 2026-04-30
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

- [ ] Make a general runtime-memory improvement that either gets the focused depth-8 gate passing or produces a smaller measured blocker from the post-361 baseline.
- [ ] Preserve the committed 185-page memory policy.
- [ ] Preserve explicit OOM failure behavior.
- [ ] Record before/after diagnostic evidence with `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30`.
- [ ] If the depth-8 fixture now completes, close issue 357 in the same merge or request parent close after verification.

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
- `issues/open/362-drive-abc451-depth8-under-iwasm-timeout-after-copy-reductions.md`
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
- [ ] `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes and records changed pressure versus the issue 361 parent-verified baseline, or records a justified replacement metric if the runtime structure changes.
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

Start from the issue 361 parent-verified baseline. The next useful result is either a passing depth-8 gate or a clearly smaller measured blocker; do not repeat broad timeout probes without diagnostic attribution.

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
