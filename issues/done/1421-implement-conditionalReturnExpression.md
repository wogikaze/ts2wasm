---
id: 1421
title: "Implement Conditionalreturnexpression"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5005]
blocks: [5160]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Triage conditionalReturnExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case in directory
`conditionalReturnExpression`. Fresh triage on 2026-05-07 shows the parser
already emits `Expr::Ternary` inside return expressions and builtin resolution
rejects it with the same unsupported boundary owned by issue 5160.

Problem: conditionalReturnExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalReturnExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalReturnExpression.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5160
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

- [x] Duplicate candidates below are confirmed; issue 5160 covers this bucket
- [x] Issue 5160 already contains exact `reference-triage` commands for the same unsupported ternary boundary
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue 5160 acceptance names the exact ternary diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalReturnExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalReturnExpression.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by issue 5160

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionalReturnExpression.ts`

## Duplicate detection

- Issue 5160 `Lower plain ternary conditional expressions` is an exact
  implementation owner for the current `ternary operator not yet supported`
  boundary. It covers `Expr::Ternary` lowering regardless of whether the
  ternary appears in a variable initializer or return expression.
- Other type-system duplicate candidates from smart triage are broad generated
  buckets and do not match the concrete ternary lowering boundary.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: type-system
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Path: reference/typescript/tests/cases/compiler/conditionalReturnExpression.ts
Failure: ternary operator not yet supported at 87..98
line 5, column 17
Source context:
2 | // @target: esnext
3 |
4 | function return1(x: boolean): 3 {
5 |     return (x ? (1) : 2);
6 | }
Visible symbols:
- function return1(x: boolean)
```

Compiler evidence:

```text
tokens: ok; Ident("x") Question LeftParen Number(1) RightParen Colon Number(2)
ast: ok; Return { expr: Ternary { condition: Ident("x"), then_expr: Number(1), else_expr: Number(2) } }
resolved: resolve_builtins fails with UnsupportedSyntax ternary operator not yet supported at 87..98
TypeScript oracle: TS2322 for branch literals not assignable to declared return type 3
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalReturnExpression.ts --detail --no-dashboard-data
result: executed=1, build_pass=0, unsupported=1, blocked=0, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=type-system:1
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/done/5160-lower-plain-ternary-conditional-expressions.md`.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- Issue 5160 still needs implementation. After 5160 lands, this reference may expose TS2322 return-type diagnostic work.
