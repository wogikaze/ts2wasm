---
id: 242
title: "Implement Date live time with WASI realtime clock"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: [239]
blocks: ["050"]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement `Date.now()` and no-argument `new Date()` using the accepted live-time policy from issue 239.

Problem: Live Date entry points still emit unsupported diagnostics even after the policy decision; implementation now needs to consume `wasi.clock.realtime` and prove the manifest/import boundary.

## Current failure

`Date.now()` and no-argument `new Date()` intentionally report issue-linked unsupported diagnostics instead of reading host time.

## Desired final state

`Date.now()` returns epoch milliseconds from WASI realtime clock access, and no-argument `new Date()` constructs a Date value from the same live epoch-millisecond source. Generated manifests explicitly declare `wasi.clock.realtime` and host-deny checks distinguish live-time programs from deterministic Date programs.

## Scope

In scope:

- [ ] Add manifest schema/runtime support for `wasi.clock.realtime`.
- [ ] Lower `Date.now()` to a WASI realtime clock read.
- [ ] Lower no-argument `new Date()` to the same live epoch-millisecond source used by Date values.
- [ ] Emit `wasi_snapshot_preview1.clock_time_get` only for live Date entry points.
- [ ] Add manifest/import and host-deny regression coverage.
- [ ] Preserve deterministic `new Date(<epoch-ms integer>)` behavior without adding live-time imports.

Out of scope:

- Timezone formatting and `Date.prototype.toString()`.
- Annex B legacy Date methods.
- Event-loop timers such as `setTimeout`.
- Node-host-only time fallback for `wasm32-wasi`.

## Affected paths

Expected:

- `crates/shared/src/capability.rs`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/builtins-and-io/`
- `docs/`
- `issues/open/050-implement-date.md`

Do not touch:

- unrelated reference corpus files

## Acceptance criteria

- [ ] `Date.now()` succeeds and returns a number within the host before/after execution clock window.
- [ ] no-argument `new Date().getTime()` succeeds and returns a number within the host before/after execution clock window.
- [ ] `--emit-manifest` records `wasi.clock.realtime: true` and `capability_reasons["wasi.clock.realtime"]` with `Date.now` or `new Date()`.
- [ ] wasm imports include `wasi_snapshot_preview1.clock_time_get` exactly when live Date entry points require it.
- [ ] deterministic Date fixtures still emit no `wasi.clock.realtime` capability and no `clock_time_get` import.
- [ ] issue 050 remains open until broader Date epic scope is complete.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli date
mise run check manifest
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

- [ ] updated if implementation changes the issue-239 policy

Current state:

- [ ] updated: `current-state.md` when live time becomes implemented

Follow-up issues:

- [ ] none unless validation exposes a smaller blocked sub-slice

## Notes

Use the issue-239 policy in `docs/03-api-and-host-capability.md` and `docs/11-shared-definitions.md`. Do not replace the WASI realtime-clock policy with a Node host import for the default `wasm32-wasi` target.

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
