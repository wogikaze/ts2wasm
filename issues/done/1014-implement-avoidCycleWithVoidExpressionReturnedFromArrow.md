---
id: 1014
title: "Implement Avoidcyclewithvoidexpressionreturnedfromarrow"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage avoidCycleWithVoidExpressionReturnedFromArrow across 1 failing reference test case and split this generated bucket into a smaller implementation-ready child issue.

## Problem

Reference test results show 1 case fails in directory `avoidCycleWithVoidExpressionReturnedFromArrow` with diagnostics: operator. Fresh triage shows the specific blocker is lowering support for unary `void`.

Problem: avoidCycleWithVoidExpressionReturnedFromArrow has 1 reference failure that is now tracked by child issue 5143.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/done/5143-implement-unary-void-operator-lowering.md`.

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
- [x] Child issue 5143 contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5143-implement-unary-void-operator-lowering.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage operator: avoidCycleWithVoidExpressionReturnedFromArrow

- Issue class: `triage-needed`
- Feature label: `operator`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unary operator Void not yet supported",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "operator",
  "error_type": "parser-or-frontend-unsupported"
}
```

Compiler evidence:

```text
tokens: Token::Void is emitted for the `void` keyword
AST: object literal callback contains Unary { op: Void, expr: Call(Member(Ident("instance"), "once"), ...) }
resolved: lower_program reports `unary operator Void not yet supported`
TypeScript oracle: ok, no diagnostics
```

## Completion evidence

Commits:

- pending local commit

Validation result:

```text
command:
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/avoidCycleWithVoidExpressionReturnedFromArrow.ts
result:
pass; emitted UnsupportedSyntax / operator report for unary `void`; split to issue 5143
date:
2026-05-06
```

Remaining risks:

- none
