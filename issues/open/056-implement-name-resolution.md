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

- [ ] Implement lexical scope tracking
- [ ] Resolve variable declarations (var, let, const)
- [ ] Resolve function declarations
- [ ] Resolve identifier references
- [ ] Handle shadowing rules
- [ ] Update diagnostic to emit UnresolvedName only when appropriate

Out of scope:

- [ ] Hoisting semantics (separate issue)
- [ ] TDZ (Temporal Dead Zone) for let/const (separate issue)
- [ ] Global object property access (separate issue)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] Name resolution passes for basic variable declarations and references
- [ ] UnresolvedName diagnostic reduced from 14 to 0 in test262 sample
- [ ] Regression test added for name resolution
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

Start with simple lexical scope before adding closure support.

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
