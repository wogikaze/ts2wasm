---
id: 357
title: "Fix ABC451 depth-8 iwasm timeout"
type: bug
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: []
blocks: [309]
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Fix the current repo-wide full-suite blocker: `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` times out under `iwasm`.

This is a smaller child slice split from issue 309 after multiple workers narrowed the remaining blocker to runtime budget/performance rather than compile correctness.

## Problem

Full `cargo nextest run` and issue-specific broad gates now reach the ABC451 depth-8 fixture and fail only because the `iwasm` execution times out around 30 seconds.

Problem: `fixtures/core-semantics/abc451-depth8-live-set.ts` no longer finishes within the iwasm test timeout, blocking otherwise validated issue closes.

## Current failure

Observed during the parent cycle after integrating issues 347 and 355:

```sh
cargo nextest run
```

Result:

```text
619 passed, 1 failed, 4 skipped
FAIL ts2wasm-cli::m2_node_diff_fixture_tests::abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
iwasm timed out for fixtures/core-semantics/abc451-depth8-live-set.ts
```

The filtered spread/node_diff retry also reproduced the same failure:

```text
166 passed, 1 failed, 457 skipped
FAIL abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
iwasm timed out
```

## Desired final state

The depth-8 ABC451 live-set fixture completes under `iwasm` within the test timeout and prints Node-matching output, without weakening memory policy or hiding the test.

## Scope

In scope:

- [ ] Reduce runtime cost for the depth-8 fixture without increasing `MEMORY_MAX_PAGES`.
- [ ] Preserve the committed 185-page memory policy and explicit OOM regression behavior.
- [ ] Prefer representation/copy/root-liveness improvements that are semantics-preserving for general programs, not ABC451 source rewrites.
- [ ] Record exact before/after timeout or runtime evidence.

Out of scope:

- Raising `MEMORY_MAX_PAGES` without official sample completion evidence.
- Skipping, ignoring, or weakening the ABC451 test.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Problem-specific generated tables or source rewrites.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/abc451-depth8-live-set.ts` only if adding instrumentation-safe comments is unavoidable
- `issues/open/357-fix-abc451-depth8-iwasm-timeout.md`
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures

## Acceptance criteria

- [ ] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` passes.
- [ ] `cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm` passes.
- [ ] If runtime policy or representation changes, `cargo test -p ts2wasm-backend-wasm --lib -- --nocapture` passes.
- [ ] `cargo fmt --all --check` passes.
- [ ] `mise run update-issue-index -- --check` and `mise run check issues` pass.
- [ ] Issue 309 remains open unless depth-9 acceptance is separately met.

## Validation

Required commands:

```sh
cargo fmt --all --check
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

- [x] not affected unless runtime memory policy changes

Current state:

- [x] updated only if runtime facts change

Follow-up issues:

- [x] none

## Notes

Issue 309 evidence already rejected array-growth-only approaches: exact-fit keeps the allocation shape smaller but remains too slow, while slack/geometric growth reduces copy pressure but trips the 185-page cap. Start from that evidence and avoid repeating the same rejected probes without a new hypothesis.

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
