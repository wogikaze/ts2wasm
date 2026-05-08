---
id: 1353
title: "Implement Commentonparenthesizedexpressionopenparen"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: [5279]
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1353.

## Summary

Closed after splitting the current method-call/lowering blocker into
`issues/done/5279-report-function-typed-local-call-definite-assignment.md`.

## Problem

Reference test results show 1 cases fail in directory `commentOnParenthesizedExpressionOpenParen` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: `commentOnParenthesizedExpressionOpenParen1.ts` now parses the
parenthesized expression and type assertion successfully, then lowering reports
the generic issue-211 function-valued local call boundary for `f()`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `f(...)` are not supported
line 4, column 30
unsupported_features=unknown-unsupported:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/done/5279-report-function-typed-local-call-definite-assignment.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5279-report-function-typed-local-call-definite-assignment.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts`

## Duplicate detection

- `issues/done/211-complete-this-receiver-binding-semantics.md` established
  issue-211 diagnostics for unsupported dynamic/function-valued local calls.
- `issues/done/435-implement-method-call.md` is the broad method-call bucket,
  not an executable slice.
- `issues/done/5195-support-callable-interface-typed-local-calls.md` and
  `issues/done/5196-support-callable-conditional-typed-parameter-calls.md`
  cover related callable typed locals, but not this `var f: () => any; f()`
  definite-assignment oracle.
- `issues/done/5279-report-function-typed-local-call-definite-assignment.md`
  owns this exact current blocker.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage method call: commentOnParenthesizedExpressionOpenParen1

- Issue class: triage-needed
- Feature label: method-call
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts
```

Failure location:

```text
issue-211: function-valued local calls such as extracted method `f(...)` are not supported
line 4, column 30
```

Source context:

```text
1 | // @target: es2015
2 | var j;
3 | var f: () => any;
4 | <any>( /* Preserve */ j = f());
```

Compiler evidence:

```text
tokens: ok
ast: ok
Expr Assign name "j" expr Call callee Ident "f"
resolved/lowered: issue-211 at f()
```

TypeScript AST evidence:

```text
ExpressionStatement -> TypeAssertionExpression -> ParenthesizedExpression ->
BinaryExpression "j = f()" -> CallExpression "f()" -> Identifier "f"
TypeScript diagnostic: TS2454 Variable 'f' is used before being assigned.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_features=unknown-unsupported:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnParenthesizedExpressionOpenParen1.ts
result: issue-211 method-call blocker after AST success; TypeScript TS2454
date: 2026-05-06
```

Remaining risks:

- Issue 5279 intentionally covers only the function-type local definite
  assignment/call diagnostic. General method-call support remains in issue 435.
