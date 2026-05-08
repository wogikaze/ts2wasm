---
id: 1183
title: "Implement Classexpressionnames"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: [5169]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1183.

## Summary

Triage classExpressionNames across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `classExpressionNames` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExpressionNames has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionNames.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionNames.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionNames.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionNames.ts
```

Not run:

- cargo gates; issue close only, no implementation changes

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5169-parse-asi-after-expression-statement.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExpressionNames.ts`

## Duplicate detection

- `issues/open/5169-parse-asi-after-expression-statement.md` owns the current parser blocker: a completed expression statement followed by a later-line statement starter.
- Broad parser-syntax epic `issues/open/059-implement-parser-syntax-extensions.md` is related but not directly selectable.

## Smart triage

Fresh triage on 2026-05-06:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionNames.ts
```

Result:

```text
Smart triage class: triage-needed
Feature label: parser-syntax
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Current error: expected Semicolon, got Some(Var) at 225..228
```

Compiler dump evidence:

```text
tokens: ok; includes object-literal class expression, variable class expression, assignment class expression, and following var declarations
ast: fails after the semicolonless expression statement `A = class {}` before `var a = new A()`
resolved: same parser failure
```

TypeScript oracle:

```text
ok; diagnostics=[]
top-level AST includes ExpressionStatement `A = class { }` followed by `var a = new A()`
```

The generated bucket is closed as superseded by issue 5169 because the current
blocker is ASI after a completed expression statement before a later-line `var`.
No new child issue is needed.

## Completion evidence

Commits:

- local superseded-close commit for issue 1183

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionNames.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_features=unknown-unsupported:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionNames.ts
result: pass; current blocker is expression-statement ASI before var, owned by issue 5169
date: 2026-05-06
```

Remaining risks:

- After issue 5169 advances parsing, this reference may expose class-expression naming or destructuring default initializer parity gaps.
