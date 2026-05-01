---
id: 4806
title: "Implement class syntax"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage class feature across 5 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 5 cases fail with class diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: class feature has 5 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/classFieldsAssignmentNamedEvaluation.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/classFieldsAssignmentNamedEvaluation.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsgo --limit 10
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/classFieldsAssignmentNamedEvaluation.ts --detail
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/classFieldsAssignmentNamedEvaluation.ts
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

## Affected test files

- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsAssignmentNamedEvaluation.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsNamedEvaluationDestructuringAssignment.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/errorInUnnamedClassExpression.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/noCrashOnMixin2.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/protectedAccessibilityCheck.ts`

## Duplicate detection

- `issues/open/017b-implement-gc-strategy.md` - issues/open/017b-implement-gc-strategy.md (same feature label, same group key)
- `issues/open/021-implement-full-wasm-backend.md` - issues/open/021-implement-full-wasm-backend.md (same feature label, same group key)
- `issues/open/050-implement-date.md` - Implement Date (same feature label, same group key, title overlap)
- `issues/open/052-implement-json.md` - Implement JSON (same feature label, same group key, title overlap)
- `issues/open/052d-implement-json-stringify-broader-replacer-semantics.md` - Implement broader JSON.stringify replacer semantics (same feature label, same group key, title overlap)
- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, same group key, title overlap)
- `issues/open/066-implement-regexp-literal.md` - Implement RegExp literal support (same feature label, same group key, title overlap)
- `issues/open/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/open/068-implement-unsupported-expression.md` - Implement unsupported expression types (same feature label, same group key, title overlap)
- `issues/open/069-implement-APILibCheck.md` - Implement Apilibcheck (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.

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
