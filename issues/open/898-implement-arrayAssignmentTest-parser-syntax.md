---
id: 898
title: "Implement Arrayassignmenttest Parser Syntax"
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

Triage arrayAssignmentTest-parser-syntax across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3 cases fail in directory `arrayAssignmentTest-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayAssignmentTest-parser-syntax has 3 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayAssignmentTest1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayAssignmentTest1.ts --detail
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
mise run reference-coverage -- tsc --limit 6
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayAssignmentTest1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayAssignmentTest1.ts
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

- `reference/typescript/tests/cases/compiler/arrayAssignmentTest1.ts`
- `reference/typescript/tests/cases/compiler/arrayAssignmentTest4.ts`
- `reference/typescript/tests/cases/compiler/arrayAssignmentTest2.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/open/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same reference path, same feature label, same group key, title overlap)
- `issues/open/674-implement-arrayFakeFlatNoCrashInferenceDeclarations.md` - Implement Arrayfakeflatnocrashinferencedeclarations (same feature label, same group key, title overlap)
- `issues/open/675-implement-arrayFilter.md` - Implement Arrayfilter (same feature label, same group key, title overlap)
- `issues/open/680-implement-arrayFrom.md` - Implement Arrayfrom (same feature label, same group key, title overlap)
- `issues/open/685-implement-arrayLiteralComments.md` - Implement Arrayliteralcomments (same feature label, same group key, title overlap)
- `issues/open/686-implement-arrayLiteralContextualType.md` - Implement Arrayliteralcontextualtype (same feature label, same group key, title overlap)
- `issues/open/689-implement-arrayOfSubtypeIsAssignableToReadonlyArray.md` - Implement Arrayofsubtypeisassignabletoreadonlyarray (same feature label, same group key, title overlap)

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
