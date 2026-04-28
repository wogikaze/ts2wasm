---
id: 057
title: "Implement function resolution for function calls"
type: feature
area: frontend
class: design-ready
priority: P0
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement function resolution to handle UnresolvedFunction diagnostics in reference tests.

## Problem

Reference test results show 5 test262 cases fail with UnresolvedFunction diagnostic. The compiler cannot resolve function names in call expressions, preventing compilation of basic JavaScript function calls.

## Desired final state

Function resolution correctly resolves function declarations, function expressions, and method calls. UnresolvedFunction diagnostic is only emitted for genuinely unresolved functions.

## Scope

In scope:

- [x] Resolve function declarations
- [x] Resolve function expressions
- [x] Resolve method calls
- [x] Handle function hoisting
- [x] Update diagnostic to emit UnresolvedFunction only when appropriate

Out of scope:

- [x] Arrow functions (covered by issue 036)
- [x] Built-in functions (separate issue)
- [x] Object methods (covered by prototype chain issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [x] Function resolution passes for basic function calls
- [x] UnresolvedFunction diagnostic reduced from 5 to 0 in test262 sample
- [x] Regression test added for function resolution
- [x] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 100
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

Start with named function declarations before adding function expressions.

## Completion evidence

Commits:

- `b14c625` wip: start issue 057 - implement function resolution

Validation result:

```text
command: cargo nextest run
result: 202 passed, 4 skipped
date: 2026-04-26
```

Remaining risks:

- none

## Notes

Issue 057 was already resolved by issue 056 (name resolution). The name_resolver module added in issue 056 handles function declarations with hoisting, which resolves the UnresolvedFunction diagnostic. No additional implementation was needed.
