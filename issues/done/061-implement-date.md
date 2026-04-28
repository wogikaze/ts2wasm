---
id: 061
title: "Implement Date object support"
type: feature
area: frontend
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
completed: 2026-04-29
---

## Summary

Implement date feature to handle 17 failing test cases in reference tests.

Problem: This duplicates the Date epic in issue 050 and mixes frontend classification with runtime/API behavior.

Queue design note:

- Do not select this issue directly.
- Superseded by issue 050. Useful affected-test evidence was copied into issue 050 on 2026-04-29.

## Problem

Reference test results show 17 cases fail with date diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

date feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

Original in-scope implementation work, now superseded by issue 050:

- Add required syntax to lexer/parser
- Implement semantics for date feature
- Add fixtures for date feature behavior
- Update diagnostics appropriately

Out of scope:

- Related features (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Original acceptance criteria

- date feature passes for basic cases
- Related diagnostics reduced in reference tests
- Regression test added for date feature
- Docs updated if semantics change

These implementation criteria were not completed in issue 061 because the issue was
closed as a duplicate of issue 050. Date implementation work remains tracked there.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 34
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- not affected

Current state:

- not updated by this superseded close

Follow-up issues:

- issue 050 remains the canonical Date epic

## Notes

- 2026-04-28 child `061-date-annexb-diagnostics-20260428T051924Z`: added precise `issue-061` unsupported diagnostics and regression fixtures for Annex B `Date.prototype.getYear`, `setYear`, and `toGMTString` when used on deterministic Date receivers. This is progress only; broad Date API support remains open.

## Affected test files

- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/nan.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/return-value.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/this-not-date.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/date-value-read-before-tonumber-when-date-is-invalid.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/date-value-read-before-tonumber-when-date-is-valid.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-not-date.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-time-nan.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-time-valid.js`
- ... and 7 more files

## Completion evidence

Closed as superseded by issue 050. No Date implementation was performed in this cleanup.

Commits:

- closing commit on branch `agent/061a-065a-issue-dedupe-20260428T233550Z` (hash recorded in cycle report)

Validation result:

```text
command: mise run update-issue-index; mise run update-issue-index -- --check; mise run check issues; mise run check issue-index
result: index update/check passed; issue-health commands returned nonzero only for unrelated pre-existing missing reports in issues 052 and 228
date: 2026-04-29
```

Remaining risks:

- Date behavior remains tracked by issue 050 and its child slices.
