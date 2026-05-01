---
id: 341
title: "Implement core builtin API coverage (3,190 test262 cases)"
type: meta
area: runtime/builtins
class: ready
priority: P1
depends_on: []
blocks: [341a, 341b, 341c, 341d, 341e]
created: 2026-04-30
updated: 2026-05-01
---

## Summary

Core runtime builtin APIs (Math, console, Number, Boolean, globalThis, etc.) are responsible for 3,190 unsupported test262 cases. This meta issue tracks closing that gap by implementing the most commonly referenced builtin APIs that are not yet covered by individual feature issues.

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

This meta issue tracks child issues for completing core builtin API coverage.

Already completed (not tracked by child issues):
- [x] Math.floor, ceil, round, abs, min, max, random, pow
- [x] console.log

Child issues:
- [ ] Issue 341a: Implement isNaN, parseInt, parseFloat, isFinite global functions
- [ ] Issue 341b: Implement Number constructor and static methods
- [ ] Issue 341c: Implement Boolean global
- [ ] Issue 341d: Implement globalThis binding
- [ ] Issue 341e: Implement encodeURI, decodeURI, escape, unescape

Out of scope:

- Array builtins (tracked by issue 313)
- String builtins (tracked by issue 314)
- Object builtins (tracked by issue 342)
- Date (tracked by issue 050)
- RegExp (tracked by issue 066)
- JSON (tracked by issue 052)
- BigInt (tracked by issues 259-263, 280-282)

## Affected paths

Child issues define their own affected paths. This meta issue spans:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`

## Acceptance criteria

This meta issue is complete when all child issues are moved to `done/`.

- [x] Math builtins (floor, ceil, round, abs, min, max, random, pow) are implemented
- [x] console.log is implemented
- [ ] All child issues (341a-341e) are moved to `done/`
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --limit 2000 --no-web-ui
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created/updated: individual builtin slice issues as needed

## Notes

Triage from triage-needed to ready with child issues on 2026-05-01.
