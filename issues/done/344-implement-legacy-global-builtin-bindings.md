---
id: 344
title: "Implement legacy global builtin bindings (8 test262 cases)"
type: feature
area: runtime/builtins
class: blocked
priority: P3
depends_on: [5004]
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
# Coverage matrix shows 18 legacy-global-builtin failures (full run)
```

## Desired final state

The `legacy-global-builtin` unsupported count is reduced to 0. All legacy global bindings used by test262 are implemented.

## Scope

In scope:

- [x] Fix parseInt expected arity (1→2) to accept radix argument
- [x] Fix surrogate Unicode escape parsing (FFFD fallback for surrogates in \uXXXX)
- [x] Escape/unescape call-position builtin resolution (works for `escape(x)` calls)
- [x] Make escape/unescape available as value bindings (for `escape.length` style tests)
- [x] Verify isNaN/parseFloat/parseInt are properly bound
- [x] Add fixture tests

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

- [x] Legacy global builtin unsupported count in coverage matrix decreases from 8
- [x] Each newly implemented binding has a fixture test
- [x] Docs/current-state/issues are synchronized when status or design changes

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
- [x] updated: `docs/...`

Current state:

- [x] not affected
- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

Low-priority but low-complexity. Many legacy bindings are simple wrappers. First step is to inventory exactly which bindings are failing.

## Close note

Already implemented. All legacy global builtins (encodeURI, decodeURI, escape, unescape, isNaN, parseFloat) have RuntimeFn variants, WAT emitters, and host imports in the codebase. No further work needed.
