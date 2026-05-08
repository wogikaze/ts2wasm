---
id: 367
title: "Extract ArrayPushGrow into a runtime helper"
type: refactor
area: backend/runtime
class: done
priority: P1
depends_on: [365, 366]
blocks: [365, 357, 309]
created: 2026-04-30
updated: 2026-05-01
status: done
completed: 2026-05-01
---

## Summary

Extract the inline `ArrayPushGrow` expression-template logic into a dedicated runtime helper with focused WAT/backend coverage, without changing runtime behavior.

## Problem

Issue 365 v3 showed that implementing non-top array-growth behavior directly inside the current inline expression template is high risk. The attempted adjacent swept free-block expansion failed before measurement because the generated instrumented WAT was malformed near the `ArrayPushGrow` expression.

Problem: the current `ArrayPushGrow` behavior is embedded in a large inline WAT template, making the next non-top growth optimization difficult to implement and validate safely.

## Current evidence

Issue 366 identified the dominant miss reason:

```text
100000 events: top_miss_reason=non_top_heap; non_top_heap=2648; committed_memory=0
300000 events: top_miss_reason=non_top_heap; non_top_heap=3771; committed_memory=0
```

Issue 365 v3 rejected direct inline-WAT behavior surgery:

```text
candidate: mark swept free-list blocks and expand a non-top ArrayPushGrow into an immediately adjacent free block before falling back to allocation/copy
result: rejected before measurement; generated instrumented WAT failed wat2wasm with "unexpected token (, expected )" near the ArrayPushGrow expression
```

## Desired final state

`ArrayPushGrow` is emitted via a dedicated helper boundary that preserves current behavior and is easier to test before future non-top growth optimizations.

## Scope

In scope:

- [x] Extract current array push growth behavior into a helper function or similarly isolated backend/runtime emission unit.
- [x] Preserve existing array push semantics and diagnostics.
- [x] Keep ABC451 diagnostic counters compatible after extraction.
- [x] Add focused backend/WAT contract coverage or equivalent regression coverage proving the helper path compiles and preserves behavior.
- [x] Record before/after diagnostics showing no behavior regression.

Out of scope:

- Changing non-top array-growth allocation policy in this slice.
- Raising memory caps or test timeouts.
- Skipping, ignoring, or weakening the ABC451 test.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Source rewriting the ABC451 fixture.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `scripts/run/abc451-runtime-costs.py` if attribution matching must adapt to the helper boundary
- `issues/open/367-extract-arraypushgrow-runtime-helper.md`
- `issues/open/365-reduce-abc451-array-growth-allocation-copy-pressure.md`
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures
- issues 359-364 or 366 unless parent explicitly reopens them

## Acceptance criteria

- [x] Existing ABC451 diagnostic still runs: `mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30` passes.
- [x] `array_push_grow` attribution remains present after extraction.
- [x] `cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm` passes.
- [x] `cargo test -p ts2wasm-backend-wasm --lib -- --nocapture` passes.
- [x] `cargo fmt --all --check` passes.
- [x] `mise run update-issue-index -- --check` and `mise run check issues` pass.
- [x] Issue 365 is updated with the new helper boundary and remains the follow-up implementation target.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected unless runtime helper ABI is externally documented

Current state:

- [x] update only if runtime/backend facts change

Follow-up issues:

- [x] issue 365 remains the implementation target after helper extraction

## Notes

This is intentionally a refactor slice. Do not combine helper extraction with a non-top growth behavior change unless the helper extraction is already validated and the behavior change independently satisfies issue 365 constraints.

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

## Child completion evidence: 2026-05-01

Status: `DONE`.

Implementation:

- Extracted `ArrayPushGrow` from inline expression-template WAT into dedicated runtime helper `$array_push_grow` selected by `RuntimeFn::ArrayPushGrow`.
- Caller emission now evaluates array/value, mirrors both GC roots, and calls `$array_push_grow`.
- Preserved current behavior: no non-top growth policy change, no memory cap change, no timeout change.
- Updated `scripts/run/abc451-runtime-costs.py` attribution matching for the named helper boundary.
- Added backend WAT contract coverage: `array_push_grow_emits_dedicated_helper_boundary` asserts helper/call emission and compiles emitted WAT with `wat2wasm`.

Diagnostic no-regression evidence:

```text
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30: pass
array_push_grow.top_miss_reason=non_top_heap
alloc_array_growth_bytes=362976; alloc_array_growth_calls=2648
copy_array_growth_bytes=181008; copy_array_growth_calls=2648
allocation_requested_bytes=521193; free_list_scan_visits=0; gc_collections=5; sweep_visits=58859
```

```text
mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30: pass
array_push_grow.top_miss_reason=non_top_heap
alloc_array_growth_bytes=1158708; alloc_array_growth_calls=3771
copy_array_growth_bytes=856928; copy_array_growth_calls=3770
allocation_requested_bytes=1376350; free_list_scan_visits=0; gc_collections=13; sweep_visits=241504
```

Validation:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-backend-wasm --lib -- --nocapture: pass; 28 passed
mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30: pass
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm: pass; 1 passed
mise run abc451-runtime-costs -- --event-budget 300000 --timeout 30: pass
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm: known timeout after 30.199s; issue 357/365 remains open
```

Remaining risks:

- This is a refactor slice only. Issue 365 remains the behavior-changing target for reducing non-top array growth allocation/copy pressure.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/367-extract-arraypushgrow-runtime-helper.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
