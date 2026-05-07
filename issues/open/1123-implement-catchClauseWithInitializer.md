---
id: 1123
title: "Implement Catchclausewithinitializer"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5219]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1123.

## Summary

Triage catchClauseWithInitializer across 1 failing reference test case and split this bucket into an implementation-ready child issue.

## Problem

Reference test results show 1 case failing in directory `catchClauseWithInitializer` with diagnostics: parser-syntax. Fresh triage shows the specific blocker is a generic parser `expected RightParen` error for `catch (e = 1)`, where TypeScript reports TS1197.

Problem: catchClauseWithInitializer has 1 reference failure whose current actionable blocker is now tracked by child issue 5219.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/catchClauseWithInitializer1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/catchClauseWithInitializer1.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/done/5219-support-explicit-this-parameter-function-expression-lowering.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable blocker into child issue 5219
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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
- [x] Child issue 5219 contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/catchClauseWithInitializer1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/catchClauseWithInitializer1.ts
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

- [x] created: `issues/done/5219-support-explicit-this-parameter-function-expression-lowering.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/catchClauseWithInitializer1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/catchClauseWithInitializer1.ts`
- issue class: `triage-needed`
- feature label: `parser-syntax`
- diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- message: `expected RightParen, got Some(Equal) at 39..40`
- child issue: `issues/done/5219-support-explicit-this-parameter-function-expression-lowering.md`

Source context:

```text
1 | // @target: es2015
2 | try {
3 | }
4 | catch (e = 1) {
5 | }
```

Compiler evidence:

```text
tokens: ok; Catch, LeftParen, Ident("e"), Equal, Number(1), RightParen
AST: fails at the Equal token before catch block parsing
resolved: same parser diagnostic
TypeScript oracle: TS1197 "Catch clause variable cannot have an initializer."; TypeScript AST has CatchClause -> VariableDeclaration `e = 1`
```

## Completion evidence

Closed as a generated triage bucket. The actionable catch-clause initializer
diagnostic blocker is tracked by child issue 5219.

Commits:

- this split commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/catchClauseWithInitializer1.ts
result: fail with generic parser `expected RightParen` at catch initializer; split to issue 5219
date: 2026-05-06
```

Remaining risks:

- After issue 5219 is implemented, follow-up TypeScript diagnostic parity may need separate tracking for destructuring catch initializers.
