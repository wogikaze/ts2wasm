---
id: 341
title: "Implement core builtin API coverage (3,190 test262 cases)"
type: feature
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Core runtime builtin APIs (Math, console, Number, Boolean, globalThis, etc.) are responsible for 3,190 unsupported test262 cases. This issue tracks closing that gap by implementing the most commonly referenced builtin APIs that are not yet covered by individual feature issues.

## Problem

test262 coverage shows 3,190 cases blocked by missing builtin API implementations (feature label: `builtin-api`). These are foundational runtime functions that many other test cases depend on.

Problem: 3,190 test262 cases fail due to missing core builtin API implementations.

## Current failure

```
mise run reference-coverage -- test262 --limit 53445
# Coverage matrix shows 3,190 builtin-api failures
```

## Desired final state

The `builtin-api` unsupported count is reduced to 0 for the implemented subset. Individual builtins (Math, console, Number, Boolean, globalThis) are either implemented or have their own tracking issues.

## Scope

In scope:

- [ ] Inventory all test262 cases tagged `builtin-api` to identify which specific APIs are missing
- [ ] Implement missing Math methods beyond pow/sqrt (e.g., Math.floor, Math.ceil, Math.round, Math.abs, Math.min/max)
- [ ] Implement console.log/bindings for diagnostics
- [ ] Implement Number constructor and static properties (Number.MAX_VALUE, Number.isNaN, etc.)
- [ ] Implement Boolean global
- [ ] Implement globalThis, isNaN, parseInt, parseFloat
- [ ] Ensure new builtins are exposed to test262 harness

Out of scope:

- Array builtins (tracked by issue 313)
- String builtins (tracked by issue 314)
- Object builtins (tracked by issue 342)
- Date (tracked by issue 050)
- RegExp (tracked by issue 066)
- JSON (tracked by issue 052)
- BigInt (tracked by issues 259-263, 280-282)

## Affected paths

Expected:

- `crates/ir/src/builtin.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/frontend/src/parser/`
- `fixtures/`

Do not touch:

- Issues already handled by other tracking issues (see out-of-scope)

## Acceptance criteria

- [ ] Builtin API unsupported count in coverage matrix decreases from 3,190
- [ ] Each newly implemented builtin has a fixture test
- [ ] Existing test262 cases that now pass are updated in the baseline
- [ ] Docs/current-state/issues are synchronized when status or design changes

## Validation

Required commands:

```sh
mise run reference-coverage -- test262 --limit 53445
mise run update-coverage-matrix
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --detail | head -50
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

- [ ] none
- [x] created/updated: individual builtin slice issues as needed

## Notes

This is a triage-need umbrella. The first step is to inventory specific missing APIs and create child issues for each major builtin group.
