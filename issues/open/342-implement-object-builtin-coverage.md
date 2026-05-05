---
id: 342
title: "Implement Object builtin method coverage (1,721 test262 cases)"
type: feature
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-05-02
---

## Summary

Object builtin methods (Object.keys, Object.values, Object.defineProperty, Object.create, Object.assign, etc.) account for 1,721 unsupported test262 cases. This issue tracks closing that gap.

## Problem

test262 coverage shows 1,721 cases blocked by missing Object builtin implementations (feature label: `object-builtin`). Basic Object.keys support exists (issue 316), but many common Object.* methods are unimplemented.

Problem: 1,721 test262 cases fail due to missing Object builtin method implementations.

## Current failure

```
mise run reference-coverage -- test262 --limit 53445
# Coverage matrix shows 1,721 object-builtin failures
```

## Desired final state

The `object-builtin` unsupported count is reduced to 0 for the implemented subset. All commonly used Object.* methods are implemented and pass the relevant test262 cases.

## Scope

In scope:

- Generated child fixture-bucket issues (3420-3429) have been consolidated back into this parent and archived (now in `issues/open/`).
- [ ] Inventory test262 cases tagged `object-builtin` to identify which methods are most impactful
- [ ] Implement Object.defineProperty
- [ ] Implement Object.getOwnPropertyDescriptor
- [ ] Implement Object.create
- [ ] Implement Object.assign
- [ ] Implement Object.values / Object.entries
- [ ] Implement Object.freeze / Object.seal
- [ ] Implement Object.prototype.hasOwnProperty
- [ ] Implement Object.prototype.toString (minimal)
- [ ] Fix Object.keys backend-io error (issue 316)
- [ ] Ensure new methods are exposed to test262 harness

Out of scope:

- Array builtins (tracked by issue 313)
- String builtins (tracked by issue 314)
- Core builtin APIs (tracked by issue 341)
- Date (tracked by issue 050)
- RegExp (tracked by issue 066)
- JSON (tracked by issue 052)

## Affected paths

Expected:

- `crates/ir/src/builtin.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/frontend/src/parser/`
- `crates/backend-wasm/src/runtime_arrays.rs`
- `crates/backend-wasm/src/runtime_objects.rs`
- `fixtures/`

Do not touch:

- Issues already handled by other tracking issues

## Acceptance criteria

- [ ] Object builtin unsupported count in coverage matrix decreases from 1,721
- [ ] Each newly implemented method has a fixture test
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
- [x] created/updated: individual Object method slice issues as needed

## Notes

This is a triage-need umbrella. The first step is to inventory specific missing Object methods and create child issues for each major method group. Issue 316 (Object.keys backend-io) is a known sub-problem.
