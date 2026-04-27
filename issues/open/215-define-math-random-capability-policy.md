---
id: 215
title: "Define Math.random capability policy"
type: feature
area: runtime/builtins
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Define and implement the policy for `Math.random` so it no longer silently returns a deterministic placeholder as semantic behavior.

## Problem

Issue 053 records that `Math.random()` currently returns `0.5`. True random behavior requires a capability decision for WASI random support or an explicit deterministic/testing mode.

## Desired final state

`Math.random()` has an auditable policy: either it imports a declared random capability for production semantics or emits a clear unsupported/diagnostic mode when randomness is unavailable.

## Scope

In scope:

- [ ] Decide the default policy for randomness on standalone WASI output.
- [ ] Update capability manifest behavior if a random host/WASI capability is used.
- [ ] Replace the silent fixed-value placeholder with policy-compliant behavior.
- [ ] Add validation for manifest/capability consistency and runtime behavior.

Out of scope:

- Cryptographic randomness APIs.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/shared/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `docs/05-compatibility-and-semantics.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [ ] `Math.random()` no longer silently reports semantic success via a fixed placeholder.
- [ ] Capability manifest output records random capability requirements when random support is enabled.
- [ ] Host-deny or manifest validation covers the chosen policy.
- [ ] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
scripts/manager check-manifest-imports
```

Impacted commands:

```sh
cargo nextest run -E 'test(math|manifest|host)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update `docs/language-reference/javascript-features.md`
- [ ] update `docs/05-compatibility-and-semantics.md`

Current state:

- [ ] update `current-state.md`

Follow-up issues:

- [ ] none

## Notes

Created from issue 203 audit of `issues/done/053-implement-math.md`.

## Completion evidence

Fill only when moving to `done/`.
