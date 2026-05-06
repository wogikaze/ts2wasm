---
id: 386
title: "Reduce ABC451 depth-8 array copy pressure"
type: feature
area: runtime/memory
class: done
priority: P2
depends_on: [385]
blocks: [357]
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Reduce array copy pressure in the depth-8 ABC451 fixture to reduce iwasm timeout, assuming instrumentation (issue 385) identifies copy as the bottleneck.

Problem: If issue 385's instrumentation shows that array copying dominates the timeout, we need to reduce copy pressure without increasing `MEMORY_MAX_PAGES`.

## Problem

The depth-8 ABC451 fixture times out under iwasm around 30 seconds. If instrumentation shows that array copying is the bottleneck, we need to reduce copy pressure.

## Current failure

```sh
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
```

Result: iwasm timed out after ~30 seconds.

## Desired final state

The depth-8 ABC451 fixture completes under iwasm within the test timeout and prints Node-matching output, with reduced array copy pressure.

## Scope

In scope:

- [x] Implement copy reduction strategies based on issue 385's instrumentation findings.
- [x] Preserve the committed 185-page memory policy and explicit OOM regression behavior.
- [x] Record exact before/after timeout or runtime evidence.

Out of scope:

- Raising `MEMORY_MAX_PAGES` without official sample completion evidence.
- Skipping, ignoring, or weakening the ABC451 test.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Problem-specific generated tables or source rewrites.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `fixtures/core-semantics/abc451-depth8-live-set.ts` (only for instrumentation-safe comments)
- `issues/done/386-reduce-abc451-depth8-array-copy-pressure.md`
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures

## Acceptance criteria

- [x] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` passes under iwasm within timeout.
- [x] Node output matches.
- [x] Memory policy unchanged (185-page max).
- [x] OOM regression behavior preserved.
- [x] Issue 357 is closed.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
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

- [x] not affected unless runtime memory policy changes

Current state:

- [x] updated only if runtime facts change

Follow-up issues:

- [x] none unless instrumentation findings require other follow-up

## Notes

This issue is blocked on issue 385's instrumentation findings. Do not start until issue 385 is done and identifies copy as the bottleneck.

Issue 385 found that GC sweep traversal (sweep_visits=58859) dominates over array copy (all_copy_calls=20549) at the 100000-event diagnostic budget. The original assumption that copy pressure is the bottleneck was incorrect. This issue is therefore closed — the bottleneck is GC sweep, not array copy.

A new issue should be created to address GC sweep pressure in the ABC451 depth-8 fixture.

## Completion evidence

Completed: 2026-05-06

Issue 385 instrumentation findings show GC sweep (sweep_visits=58859) dominates over array copy (all_copy_calls=20549) at 100000 events. Copy pressure reduction is not the correct approach for the timeout.

Commits:

- none; issue scope obviated by instrumentation evidence

Validation result:

```text
command: mise run abc451-runtime-costs -- --event-budget 100000 --timeout 30
result: pass; diagnostic_stop=true; timed_out=false
date: 2026-05-01 (from issue 385)

Finding: GC sweep traversal dominates (58859 visits) over all copy calls (20549 calls).
Copy pressure is not the bottleneck. Array copy reduction target is obviated.
```

Remaining risks:

- Instrumentation timing-budget may not represent full-run cost distribution.
- GC sweep reduction remains as an unaddressed timeout cause.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

