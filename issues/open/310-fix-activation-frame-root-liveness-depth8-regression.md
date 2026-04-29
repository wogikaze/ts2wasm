---
id: 310
title: "Fix activation-frame root liveness depth-8 regression"
type: feature
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: []
blocks: [309, 308, 300]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Make the next activation-frame root-liveness narrowing slice safe before
retrying issue 309's depth-9 allocation-shape reduction.

Problem: a rejected issue-309 experiment proved stale function activation-frame
roots are relevant to the depth-9 memory cap, but both broad and narrower
root-clearing variants regressed the required depth-8 fixture with
`Exception: unreachable`.

## Current failure

Issue 309 evidence:

```text
root-liveness experiment: reduced depth-9 guard from size=6140/block_size=6160
to size=3068/block_size=3088
required validation: abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: fail; Exception: unreachable
```

The committed tree keeps the previous safe behavior: depth-8 passes, while
depth-9 still fails at the committed 185-page guard.

## Desired final state

Activation-frame root liveness is narrowed only when the values are no longer
semantically live, and the depth-8 fixture remains passing. If that cannot be
implemented safely in one slice, a smaller blocker is recorded with exact
root/heap evidence.

## Scope

In scope:

- [ ] Reproduce the depth-8 regression caused by activation-frame root clearing.
- [ ] Identify why clearing stale user-call or block-scoped locals exposes an
      invalid GC/free-list state or loses a still-live value.
- [ ] Implement the smallest safe root-liveness narrowing, or record a smaller
      blocker with exact evidence.
- [ ] Preserve current depth-8 output and OOM trap behavior.

Out of scope:

- BigInt/private-class/spread/eval work.
- Raising `MEMORY_MAX_PAGES`.
- Problem-specific ABC451 source rewriting or generated tables.
- Closing issue 300 without official sample compatibility evidence.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `fixtures/atcoder/`
- `issues/open/300-support-abc451-large-integer-number-boundary.md`
- `issues/open/308-implement-abc451-depth9-gc-cadence-policy.md`
- `issues/open/309-reduce-abc451-depth9-live-allocation-shape.md`
- `issues/open/310-fix-activation-frame-root-liveness-depth8-regression.md`
- `issues/index.md`

Do not touch:

- BigInt runtime/ABI files
- parser/frontend files unrelated to runtime memory
- private-class, spread, or eval files
- problem-specific source rewrite hooks

## Acceptance criteria

- [ ] A focused reproduction or regression test covers the unsafe
      activation-frame root-clearing behavior.
- [ ] If root-liveness narrowing is implemented, the depth-8 fixture still
      matches Node under iwasm.
- [ ] If no safe implementation is committed, the next blocker is documented
      with exact allocation/root evidence and issue 309 remains blocked.
- [ ] `oom_alloc_check_must_fail_iwasm` remains passing.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-backend-wasm --lib -- --nocapture
cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
node /tmp/abc451-depth9-live-set-309.ts
cargo run -q -- build /tmp/abc451-depth9-live-set-309.ts -o /tmp/abc451-depth9-live-set-310.wasm --host-deny
timeout 90s iwasm /tmp/abc451-depth9-live-set-310.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` if runtime memory or GC policy changes

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` if runtime facts change

Follow-up issues:

- [ ] none
- [ ] created/updated if this slice proves a smaller blocker

## Notes

Do not re-commit the rejected issue-309 root-clearing variants unless the
depth-8 regression is explained and fixed.

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
