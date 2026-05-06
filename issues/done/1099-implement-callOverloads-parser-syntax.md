---
id: 1099
title: "Implement Calloverloads Parser Syntax"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5199, 5200]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage callOverloads-parser-syntax across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3 cases fail in directory `callOverloads-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callOverloads-parser-syntax has 3 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callOverloads1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 6
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callOverloads1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5199-report-function-overload-list-class-merge-diagnostics.md`
- [x] `issues/open/5200-validate-top-level-function-overload-implementations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/callOverloads1.ts`
- `reference/typescript/tests/cases/compiler/callOverloads2.ts`
- `reference/typescript/tests/cases/compiler/callOverloads5.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/open/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/open/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

## Smart triage

### Smart triage: Triage duplicate function: callOverloads parser bucket

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Paths:
  - `reference/typescript/tests/cases/compiler/callOverloads1.ts`
  - `reference/typescript/tests/cases/compiler/callOverloads2.ts`
  - `reference/typescript/tests/cases/compiler/callOverloads5.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads1.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads2.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads5.ts
```

Current failures:

```text
callOverloads1.ts: error: [DuplicateFunction] duplicate function definition: `F1` at 257..287
callOverloads2.ts: error: [DuplicateFunction] duplicate function definition: `F1` at 283..313
callOverloads5.ts: error: [DuplicateFunction] duplicate function definition: `Foo` at 69..77
```

Evidence:

- Tokens and AST succeed for all three files, so this bucket is no longer a
  parser-syntax blocker.
- `callOverloads1.ts` contains `class Foo`, `function Foo();`, and a valid
  `F1` overload signature plus implementation.
- `callOverloads2.ts` contains `class Foo`, `function Foo();`, duplicate
  implemented `F1` declarations, and a bodyless `Goo` overload without an
  implementation.
- `callOverloads5.ts` contains two bodyless `function Foo` declarations followed
  by `class Foo` with bodyless `bar1` method overload signatures.
- TypeScript oracle reports class/function merge diagnostics for `Foo`; it also
  reports duplicate implementation and missing implementation diagnostics for
  the invalid top-level function overload shapes.
- Child issue `issues/open/5199-report-function-overload-list-class-merge-diagnostics.md`
  owns the `Foo` class/function merge blocker.
- Child issue `issues/open/5200-validate-top-level-function-overload-implementations.md`
  owns the `F1` and `Goo` top-level function overload implementation blockers.
- Duplicate candidates `issues/open/442-implement-parser-syntax.md`,
  `issues/open/550-implement-FunctionDeclaration-parser-syntax.md`, and broad
  parser-syntax buckets are no-match owners because this fresh triage already
  passes tokenization and AST construction.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOverloads1.ts
result: pass; reproduced DuplicateFunction for F1 overload implementation grouping
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOverloads2.ts
result: pass; reproduced DuplicateFunction for duplicate implemented F1 declarations
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOverloads5.ts
result: pass; reproduced DuplicateFunction for Foo overload list before class merge diagnostics
date: 2026-05-06
```

Remaining risks:

- none
