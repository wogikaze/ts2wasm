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
completed: 2026-04-28
status: done
---

## Summary

Define and implement the policy for `Math.random` so it no longer silently returns a deterministic placeholder as semantic behavior.

## Problem

Issue 053 records that `Math.random()` currently returns `0.5`. True random behavior requires a capability decision for WASI random support or an explicit deterministic/testing mode.

## Desired final state

`Math.random()` has an auditable policy: either it imports a declared random capability for production semantics or emits a clear unsupported/diagnostic mode when randomness is unavailable.

## Scope

In scope:

- [x] Decide the default policy for randomness on standalone WASI output.
- [x] Update capability manifest behavior if a random host/WASI capability is used.
- [x] Replace the silent fixed-value placeholder with policy-compliant behavior.
- [x] Add validation for manifest/capability consistency and runtime behavior.

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

- [x] `Math.random()` no longer silently reports semantic success via a fixed placeholder.
- [x] Capability manifest output records random capability requirements when random support is enabled.
- [x] Host-deny or manifest validation covers the chosen policy.
- [x] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check-manifest-imports
```

Impacted commands:

```sh
cargo nextest run -E 'test(math|manifest|host)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] update `docs/language-reference/javascript-features.md`
- [x] update `docs/05-compatibility-and-semantics.md`

Current state:

- [x] update `current-state.md`

Follow-up issues:

- [x] none

## Notes

Created from issue 203 audit of `issues/done/053-implement-math.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `2cbf0af` (`issue-215: add auditable Math.random random policy`)

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: cargo nextest run -E 'test(math) | test(manifest) | test(host)'
result: pass, 15 passed / 232 skipped
date: 2026-04-28

command: mise run check-manifest-imports
result: pass for fixtures/basics-hello/hello.ts
date: 2026-04-28

command: cargo nextest run
result: pass, 243 passed / 4 skipped
date: 2026-04-28

command: direct build of fixtures/builtins-and-io/math-random.ts with --emit-manifest
result: pass; manifest has standalone=true, wasi.random=true, node_host.required=false, capability_reasons["wasi.random"] includes Math.random, and wasm imports random_get
date: 2026-04-28
```

Remaining risks:

- none for the capability policy. Full ECMAScript fractional double parity remains part of the broader number model and is documented outside this issue.
