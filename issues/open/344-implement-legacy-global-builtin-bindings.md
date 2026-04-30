---
id: 344
title: "Implement legacy global builtin bindings (8 test262 cases)"
type: feature
area: runtime/builtins
class: triage-needed
priority: P3
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Legacy global builtin bindings (e.g., escape, unescape, isNaN, parseFloat) account for 8 unsupported test262 cases. These are Annex B or legacy properties that should be available as global bindings.

## Problem

test262 coverage shows 8 cases blocked by missing legacy global builtin bindings (feature label: `legacy-global-builtin`). These are legacy global properties that JavaScript engines are expected to provide for web compatibility.

Problem: 8 test262 cases fail due to missing legacy global builtin bindings.

## Current failure

```
mise run reference-coverage -- test262 --limit 53445
# Coverage matrix shows 8 legacy-global-builtin failures
```

## Desired final state

The `legacy-global-builtin` unsupported count is reduced to 0. All legacy global bindings used by test262 are implemented.

## Scope

In scope:

- [ ] Identify which legacy globals are referenced by failing test262 cases
- [ ] Implement escape/unescape global functions
- [ ] Verify isNaN/parseFloat/parseInt are properly bound
- [ ] Add fixture tests

Out of scope:

- Non-legacy builtin APIs (tracked by issues 341, 342, 313, 314)
- Date legacy methods (tracked by issue 241)

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/backend-wasm/src/runtime_builtins_host.rs`

Do not touch:

- none

## Acceptance criteria

- [ ] Legacy global builtin unsupported count in coverage matrix decreases from 8
- [ ] Each newly implemented binding has a fixture test
- [ ] Docs/current-state/issues are synchronized when status or design changes

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --limit 53445
mise run update-coverage-matrix
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected
- [ ] updated: `docs/...`

Current state:

- [x] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

Low-priority but low-complexity. Many legacy bindings are simple wrappers. First step is to inventory exactly which bindings are failing.
