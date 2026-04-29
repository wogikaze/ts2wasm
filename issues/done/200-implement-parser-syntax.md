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
completed: 2026-04-29
status: done
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

- [x] Run the representative `mise run reference-triage -- ...` command
- [x] Confirm whether duplicate candidates already cover this failure
- [x] Close as superseded/resolved instead of splitting: the representative path now builds, and broader parser syntax ownership is covered by done issue 059 plus its child wave.
- [x] Carry source context, diagnostic code, AST evidence, and validation commands into completion evidence

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

- [x] Duplicate candidates are confirmed as superseding issues: `issues/done/059-implement-parser-syntax-extensions.md`, `issues/done/065-implement-parser-syntax.md`, and `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md`.
- [x] No new child issue is required for the representative path because focused coverage now reports `build_pass`.
- [x] Source context, diagnostic code, visible symbols, and AST/resolved evidence are recorded below.
- [x] The exact reference path and diagnostic/stdout change are named below.

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

- [x] not affected

Current state:

- [x] not updated; this is generated bucket cleanup with no implementation state change

Follow-up issues:

- [x] none for the representative path; broader remaining parser/runtime work is already tracked by specific open issues such as `250`-`255`.

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

## Closure note

Issue 200 is a generated parser-syntax bucket and is not a direct implementation work order. The representative path `reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts` no longer reproduces a parser-syntax failure: focused reference coverage reports `build_pass`, and `reference-triage` shows token, AST, and resolved dumps are all available. Broader parser syntax ownership has been closed or split through issue 059 and its child issues.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending parent commit

Validation result:

```text
command: mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts
result: pass; diagnostic now reports Unknown/unknown from TypeScript oracle context, while tokens/ast/resolved compiler dumps are ok; no parser-syntax failure remains for this path
date: 2026-04-29

command: mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/assertsPredicateParameterMismatch.ts --detail
result: pass; executed=1, build_pass=1, semantic_pass=1, unsupported=0
date: 2026-04-29
date: 2026-04-29
```

Remaining risks:

- The old 51-case generated bucket should not be selected directly again; future failures must be represented by exact reference-backed child issues.
