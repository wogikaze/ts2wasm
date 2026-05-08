---
id: 1117
title: "Implement Castfunctionexpressionshouldbeparenthesized"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1117.

## Summary

Triage castFunctionExpressionShouldBeParenthesized across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `castFunctionExpressionShouldBeParenthesized` with diagnostics: method-call. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: castFunctionExpressionShouldBeParenthesized has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5217-preserve-ambient-value-declarations-through-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage shows this bucket is stale as a parser issue. Tokens and AST
succeed; the current blocker is method-call receiver lowering.

### Smart triage: castFunctionExpressionShouldBeParenthesized

- Issue class: `triage-needed`
- Feature label: `method-call`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: method `foo` requires an identifier receiver at 20..52",
  "line": 2,
  "column": 2
}
```

Source context:

```text
1 | // @target: es2015
2 | (function a() { } as any)().foo()
```

Compiler evidence:

```text
tokens: ok
ast: ok; Call(Member(Call(FunctionExpr a, args=[]), property="foo"), args=[])
resolved/lowered: issue-211 method `foo` requires an identifier receiver
```

TypeScript AST sees `CallExpression -> PropertyAccessExpression ->
CallExpression -> ParenthesizedExpression -> AsExpression ->
FunctionExpression` and reports no diagnostics. Child issue
`issues/open/5217-preserve-ambient-value-declarations-through-name-resolution.md` owns
this receiver-lowering slice.

## Completion evidence

castFunctionExpressionShouldBeParenthesized triage is complete. The current
blocker is represented by focused implementation issue 5217.

Commits:

- `aef1c65b` issues: split cast function receiver method blocker

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax method-call
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts
result: pass; AST succeeds and lowering reports issue-211 at .foo()
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
result: pass; child issue 5217 is M-sized and ready
date: 2026-05-06

command: git diff --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none
