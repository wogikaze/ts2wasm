---
id: 056
title: "Implement name resolution for variables and identifiers"
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

Implement name resolution to handle UnresolvedName diagnostics in reference tests.

## Problem

Reference test results show 14 test262 cases fail with UnresolvedName diagnostic. The compiler cannot resolve variable and identifier names, preventing compilation of basic JavaScript code.

## Desired final state

Name resolution correctly resolves variable declarations, function declarations, and identifier references within their appropriate scopes. UnresolvedName diagnostic is only emitted for genuinely unresolved names.

## Scope

In scope:

- [x] Implement lexical scope tracking
- [x] Resolve variable declarations (var, let, const)
- [x] Resolve function declarations
- [x] Resolve identifier references
- [x] Handle shadowing rules
- [x] Update diagnostic to emit UnresolvedName only when appropriate

Out of scope:

- [x] Hoisting semantics (separate issue)
- [x] TDZ (Temporal Dead Zone) for let/const (separate issue)
- [x] Global object property access (separate issue)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [x] Name resolution passes for basic variable declarations and references
- [x] UnresolvedName diagnostic reduced from 14 to 0 in test262 sample
- [x] Regression test added for name resolution
- [x] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/manager reference-coverage test262 --limit 100
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

Start with simple lexical scope before adding closure support.

## Completion evidence

Commits:

- `8d7c2d7` wip: start issue 056 - implement name resolution
- (pending) implement name resolution

Validation result:

```text
command: cargo nextest run
result: 202 passed, 4 skipped
date: 2026-04-26
```

Remaining risks:

- none
