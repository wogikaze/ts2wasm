---
id: 1425
title: "Implement Conditionaltypediscriminatinglargeunionregulartypefetchingspeedreasonable"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1425.

## Summary

Triage conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results originally showed 1 case failing in directory
`conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable` with
diagnostics: import-export. Fresh triage on 2026-05-07 shows the concrete
blocker is a trailing comma before `)` in an exported generic function
declaration parameter list.

Problem: conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts --detail
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
- [x] At least one child issue contains an exact `python scripts/manager.py reference-triage ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5303-parse-trailing-comma-in-typed-function-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts`

## Duplicate detection

- `issues/done/5278-parse-trailing-comma-in-function-parameters-with-comments.md`
  is related but narrower: ordinary function parameters with comments.
- `issues/done/5149-parse-trailing-comma-in-typed-class-method-parameters.md`
  is related but covers class methods, not exported function declarations.
- Generic type-system candidates from smart triage are no-match because the
  concrete failure happens before AST construction at a parameter-list
  `RightParen`.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: type-system
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Path: reference/typescript/tests/cases/compiler/conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts
Failure: issue-247: expected binding identifier or pattern, got Some(RightParen) at 139298..139299
line: 8013, column: 6
Visible symbols before failure: []
```

Source context:

```text
8010 | export function makeThing<T extends BigUnion['name']>(
8011 |     name: T,
8012 |     children: ChildrenOf<WithName<T>>[] = [],
8013 | ) { }
```

Compiler evidence:

```text
tokens: ok
ast: false; fails before AST construction with issue-247 at RightParen
resolved: false; same parser diagnostic
TypeScript oracle: ok, diagnostics=[]
```

TypeScript AST evidence:

```text
TypeAliasDeclaration BigUnion
TypeAliasDeclaration DiscriminateUnion
TypeAliasDeclaration WithName
TypeAliasDeclaration ChildrenOf
FunctionDeclaration export function makeThing<T extends BigUnion['name']>(...)
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeDiscriminatingLargeUnionRegularTypeFetchingSpeedReasonable.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=type-system:1, build_pass=0, semantic_pass=0, blocked=0
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split concrete parser work to issue 5303 and closed this generated bucket.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
