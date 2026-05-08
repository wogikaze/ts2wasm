---
id: 1145
title: "Implement Checktypepredicateforredundantproperties"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: [5235]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1145.

## Summary

Triage checkTypePredicateForRedundantProperties across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkTypePredicateForRedundantProperties` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkTypePredicateForRedundantProperties has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts --detail
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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5235-w1-implement-wasi-stdin-fd_read-lowering-for-input.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts`

## Duplicate detection

Fresh duplicate scan found broader type-predicate buckets, but no exact open
issue for erasing an object type literal in a type-predicate return annotation:

- `issues/open/4523-implement-typeInferenceTypePredicate-type-system.md`
  covers a different reference path and type inference behavior.
- `issues/open/4564-implement-typePredicateStructuralMatch.md` and related
  `typePredicate*` generated buckets are broad type-system buckets, not this
  parser erasure boundary.
- This bucket's first blocker is `InvalidTopLevelReturn`, not a type-system
  diagnostic yet.

## Smart triage

Fresh triage shows the current blocker is parser erasure for the return type,
not duplicate-property type checking yet. The parser consumes the type
predicate object type literal as labeled function-body statements and leaves
the real `return` statement at top level.

### Smart triage: checkTypePredicateForRedundantProperties

- Issue class: `triage-needed`
- Feature label: `top-level-return`
- Diagnostic: `InvalidTopLevelReturn` / `compiler-diagnostic`
- Current compiler message: `top-level return is not supported`
- Path: `reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=InvalidTopLevelReturn:1
unsupported_features=top-level-return:1
```

Source context:

```ts
function addProp2(x: any): x is { a: string; a: string; } {
    return true;
}
```

Compiler evidence:

```text
tokens: ok
ast: Function body incorrectly contains Labeled("a": Ident("string")) twice
ast: following Return(true) is emitted as a top-level statement
validate_ast: InvalidTopLevelReturn at return true
TypeScript oracle: TS2300 Duplicate identifier 'a' on the two object type properties
```

Split result:

- `issues/open/5235-w1-implement-wasi-stdin-fd_read-lowering-for-input.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts --detail --no-dashboard-data
result: pass; reproduced current unsupported bucket
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkTypePredicateForRedundantProperties.ts
result: pass; reproduced type-predicate object return erasure boundary and split to issue 5235
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5235 may expose the intended TS2300 duplicate-property diagnostic as the next blocker.
