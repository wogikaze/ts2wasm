---
id: 200
title: "Implement parser syntax extensions"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-29
---

## Summary

Triage the generated reference bucket `Implement parser syntax extensions` before implementation. This issue records a failing reference case and must be split or superseded before any code change starts.

## Problem

Reference test results show 51 cases fail with parser-syntax diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: generated reference bucket `Implement parser syntax extensions` fails with `parser-syntax` and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts
```

Narrow coverage reproduction:

```sh
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts --detail
```

Representative path: `reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts`
Feature label: `parser-syntax`

## Desired final state

This generated bucket is not used as a direct implementation work order. It is either superseded by an existing open/done issue, closed as a duplicate, or split into implementation-ready child issues that contain exact reproduction evidence and measurable acceptance criteria.

## Scope

In scope:

- [ ] Run the representative `mise run reference-triage -- ...` command
- [ ] Confirm whether duplicate candidates already cover this failure
- [ ] Split one observable behavior or fixed reference window into child issues
- [ ] Carry source context, diagnostic code, AST evidence, and validation commands into each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad fixes that mix unrelated parser, resolver, runtime, and API failures

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [ ] Duplicate candidates are confirmed as no-match, duplicate, or superseding issue
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts --detail
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

- `reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/automaticTypeDirectiveResolutionBundler.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/checkInheritedProperty.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/circularDestructuring.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classExpressionWithComputedPropertyInLoop.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsAssignmentNamedEvaluation.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsNamedEvaluationDestructuringAssignment.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsPrivatePropertyAccessSameNameAsClass.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/constEnumInEmbeddedStatements.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/constructSignatureWithInferReturnType.ts`
- ... and 41 more files

## Duplicate detection

- `issues/done/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

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
