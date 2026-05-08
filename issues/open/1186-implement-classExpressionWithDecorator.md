---
id: 1186
title: "Implement Classexpressionwithdecorator"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5253]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1186.

## Summary

Triage classExpressionWithDecorator across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `classExpressionWithDecorator` with diagnostics: decorator. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExpressionWithDecorator has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts
```

Not run:

- cargo gates; issue split only, no implementation changes

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5253-report-class-expression-decorator-boundary.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts`

## Duplicate detection

- `issues/open/4807-implement-decorator.md` is the broad generated decorator bucket and lists this issue as a duplicate candidate.
- No existing implementation-ready issue covered the first concrete lexer/parser blocker for class expression decorators.

## Smart triage

Fresh triage on 2026-05-06:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts
```

Result:

```text
Smart triage class: triage-needed
Feature label: decorator
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Current error: unsupported character: @ at 62..63
```

Compiler dump evidence:

```text
tokens: fail at @
ast: fail at @
resolved: fail at @
```

TypeScript oracle:

```text
TS2304: Cannot find name 'decorate'.
AST: ClassExpression "@decorate class C { static p = 1 }" with Decorator "@decorate"
```

The executable child issue is the narrow lexer/parser classifier boundary for
class expression decorators. Full decorator transforms remain out of scope.

## Completion evidence

Commits:

- local split commit for issue 1186 / child 5253

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_features=decorator:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts
result: pass; current blocker is lexer UnsupportedSyntax for `@`, split to issue 5253
date: 2026-05-06
```

Remaining risks:

- Full decorator transform/runtime semantics remain outside issue 5253.
