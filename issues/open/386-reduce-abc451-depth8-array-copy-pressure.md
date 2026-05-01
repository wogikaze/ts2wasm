---
id: 386
title: "Reduce ABC451 depth-8 array copy pressure"
type: feature
area: runtime/memory
class: triage-needed
priority: P2
depends_on: [385]
blocks: [357]
created: 2026-05-01
updated: 2026-05-01
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

- [ ] Implement copy reduction strategies based on issue 385's instrumentation findings.
- [ ] Preserve the committed 185-page memory policy and explicit OOM regression behavior.
- [ ] Record exact before/after timeout or runtime evidence.

Out of scope:

- Raising `MEMORY_MAX_PAGES` without official sample completion evidence.
- Skipping, ignoring, or weakening the ABC451 test.
- BigInt, spread, eval, private-class, parser, or reference-harness work.
- Problem-specific generated tables or source rewrites.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `fixtures/core-semantics/abc451-depth8-live-set.ts` (only for instrumentation-safe comments)
- `issues/open/386-reduce-abc451-depth8-array-copy-pressure.md`
- `issues/index.md`

Do not touch:

- BigInt files
- spread/eval/private-class issue files
- parser/frontend files
- unrelated fixtures

## Acceptance criteria

- [ ] `cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` passes under iwasm within timeout.
- [ ] Node output matches.
- [ ] Memory policy unchanged (185-page max).
- [ ] OOM regression behavior preserved.
- [ ] Issue 357 is closed.

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

- [ ] not affected unless runtime memory policy changes

Current state:

- [ ] updated only if runtime facts change

Follow-up issues:

- [ ] none unless instrumentation findings require other follow-up

## Notes

This issue is blocked on issue 385's instrumentation findings. Do not start until issue 385 is done and identifies copy as the bottleneck.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```

Remaining risks:

- Instrumentation may show that copy is not the bottleneck; this issue may be closed or re-scoped.
