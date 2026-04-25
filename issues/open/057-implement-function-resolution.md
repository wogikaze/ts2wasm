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

- [ ] Resolve function declarations
- [ ] Resolve function expressions
- [ ] Resolve method calls
- [ ] Handle function hoisting
- [ ] Update diagnostic to emit UnresolvedFunction only when appropriate

Out of scope:

- [ ] Arrow functions (covered by issue 036)
- [ ] Built-in functions (separate issue)
- [ ] Object methods (covered by prototype chain issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] Function resolution passes for basic function calls
- [ ] UnresolvedFunction diagnostic reduced from 5 to 0 in test262 sample
- [ ] Regression test added for function resolution
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/run/reference-coverage.sh test262 --limit 100
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Start with named function declarations before adding function expressions.

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
