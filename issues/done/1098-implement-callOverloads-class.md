---
id: 1098
title: "Implement Calloverloads Class"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5199]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage callOverloads-class across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `callOverloads-class` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: callOverloads-class has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callOverloads3.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/callOverloads3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads3.ts
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

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/callOverloads3.ts`
- `reference/typescript/tests/cases/compiler/callOverloads4.ts`

## Duplicate detection

- `issues/done/255-implement-private-class-element-runtime-semantics.md` - Implement private class element runtime semantics (same feature label, title overlap)
- `issues/open/421-implement-class.md` - Implement class syntax (same feature label, title overlap)
- `issues/done/045-implement-class-syntax.md` - Implement class declaration and expression (same feature label, title overlap)
- `issues/done/248-implement-private-class-element-parser.md` - Implement private class element parser support (same feature label, title overlap)
- `issues/done/249-implement-class-static-block-parser.md` - Implement class static block parser support (same feature label, title overlap)
- `issues/done/254-implement-class-static-block-runtime-semantics.md` - Implement class static block runtime semantics (same feature label, title overlap)

## Smart triage

### Smart triage: Triage duplicate function: callOverloads3 / callOverloads4

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Paths:
  - `reference/typescript/tests/cases/compiler/callOverloads3.ts`
  - `reference/typescript/tests/cases/compiler/callOverloads4.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads3.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOverloads4.ts
```

Failure in both files:

```text
error: [DuplicateFunction] duplicate function definition: `Foo` at 52..60
```

Source context:

```ts
function Foo():Foo; // error
function Foo(s:string):Foo; // error
class Foo { // error
    bar1() { }
    constructor(x: any) { }
}
```

Evidence:

- Tokens and AST succeed for both reference files.
- AST contains two bodyless top-level `Function Foo` declarations followed by
  `ClassDecl Foo`.
- `callOverloads4.ts` also includes a bodyless constructor overload before the
  constructor implementation.
- TypeScript oracle reports TS2814, TS2391, and TS2813 instead of treating the
  second `function Foo` declaration as a duplicate concrete implementation.
- Duplicate candidates `issues/open/2043-implement-duplicateIdentifierRelatedSpans-duplicate-function.md`,
  `issues/open/2600-implement-getAndSetNotIdenticalType-duplicate-function.md`,
  and `issues/open/4258-implement-staticVisibility-duplicate-function.md` are
  no-match buckets for different duplicate-function windows.
- Related `issues/open/769-implement-augmentedTypesFunction.md` is a different
  parser-syntax bucket; child issue
  `issues/open/5199-report-function-overload-list-class-merge-diagnostics.md`
  owns this already-parsed callOverloads blocker.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOverloads3.ts
result: pass; reproduced DuplicateFunction for top-level function overload list before class merge diagnostics
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOverloads4.ts
result: pass; reproduced same DuplicateFunction blocker with constructor overload variant
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

