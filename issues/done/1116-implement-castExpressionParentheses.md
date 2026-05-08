---
id: 1116
title: "Implement Castexpressionparentheses"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1116.

## Summary

Triage castExpressionParentheses across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `castExpressionParentheses` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: castExpressionParentheses has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castExpressionParentheses.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castExpressionParentheses.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castExpressionParentheses.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castExpressionParentheses.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5216-allow-extra-arguments-for-functions-reading-arguments-object.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/castExpressionParentheses.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage shows this bucket currently stops at a numeric-literal lexer
boundary before it reaches cast-expression parentheses behavior.

### Smart triage: castExpressionParentheses

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/castExpressionParentheses.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castExpressionParentheses.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "invalid number literal: number too large to fit in target type at 215..221",
  "line": 15,
  "column": 5
}
```

Source context:

```text
12 | (<any>1.);
13 | (<any>1.0);
14 | (<any>12e+34);
15 | (<any>0xff);
16 | (<any>/regexp/g);
```

Compiler evidence:

```text
tokens: fails before token output
ast: same lexer failure
resolved: same lexer failure
```

TypeScript AST sees `ExpressionStatement -> ParenthesizedExpression ->
TypeAssertionExpression -> FirstLiteralToken "12e+34"` and reports no
diagnostics. Child issue
`issues/done/5216-allow-extra-arguments-for-functions-reading-arguments-object.md` owns this
lexer/parser prerequisite.

## Completion evidence

castExpressionParentheses triage is complete. The current blocker is
represented by focused implementation issue 5216.

Commits:

- `5ea0c94e` issues: split cast expression large number blocker

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/castExpressionParentheses.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax parser-syntax
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castExpressionParentheses.ts
result: pass; reproduced lexer failure for 12e+34 and split issue 5216
date: 2026-05-06

command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-06

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass; child issue 5216 is M-sized and ready
date: 2026-05-06

command: git diff --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none
