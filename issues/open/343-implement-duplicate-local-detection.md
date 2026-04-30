---
id: 343
title: "Implement DuplicateLocal diagnostic detection (66 test262 cases)"
type: feature
area: frontend/resolver
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

DuplicateLocal diagnostics (variable shadowing in the same scope) account for 66 unsupported test262 cases. The compiler should detect and report duplicate local variable declarations in the same scope.

## Problem

test262 coverage shows 66 cases blocked by `DuplicateLocal` diagnostics. When two variables with the same name are declared in the same scope, the compiler should produce an appropriate diagnostic instead of silently ignoring or crashing.

Problem: 66 test262 cases fail due to missing DuplicateLocal diagnostic detection.

## Current failure

```
mise run reference-coverage -- test262 --limit 53445
# Coverage matrix shows 66 DuplicateLocal failures
```

## Desired final state

The `DuplicateLocal` unsupported count is reduced to 0. The compiler detects duplicate local variable declarations in the same function/block scope and reports them as diagnostics.

## Scope

In scope:

- [ ] Implement duplicate local variable detection in the resolver/frontend
- [ ] Report appropriate diagnostic for duplicate declarations
- [ ] Support all declaration forms (var, let, const)
- [ ] Differentiate between strict mode and non-strict mode rules
- [ ] Add fixture tests for common duplicate local patterns

Out of scope:

- Cross-scope shadowing (that's allowed in JavaScript)
- TypeScript-specific duplicate identifier rules
- Name resolution infrastructure (tracked by issues 056/064)

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`

## Acceptance criteria

- [ ] DuplicateLocal unsupported count in coverage matrix decreases from 66
- [ ] Fixture tests cover basic duplicate var/let/const in same scope
- [ ] Existing test262 cases that now pass are updated in the baseline
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

This is a relatively contained feature. The resolver already tracks variable names; duplicate detection should be added as a validation pass after scoped resolution.
