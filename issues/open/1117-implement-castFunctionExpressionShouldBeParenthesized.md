---
id: 1117
title: "Implement Castfunctionexpressionshouldbeparenthesized"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] `issues/open/5217-support-method-calls-on-call-expression-receivers.md`

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
`issues/open/5217-support-method-calls-on-call-expression-receivers.md` owns
this receiver-lowering slice.

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
