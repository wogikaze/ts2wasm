---
id: 1352
title: "Implement Commentonparameter"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: [5278]
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1352.

## Summary

Closed after splitting the current parser blocker into
`issues/open/5278-parse-trailing-comma-in-function-parameters-with-comments.md`.

## Problem

Reference test results show 1 cases fail in directory `commentOnParameter` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: `commentOnParameter3.ts` fails on an issue-247 RightParen trailing
parameter comma parser blocker in an ordinary function declaration with comments
around parameters.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnParameter3.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnParameter3.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
UnsupportedSyntax: issue-247: expected binding identifier or pattern, got Some(RightParen) at 139..140
line 7, column 6
unsupported_features=unknown-unsupported:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5278-parse-trailing-comma-in-function-parameters-with-comments.md`.

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
- [x] Exact `reference-triage` command is preserved
- [x] Current path, diagnostic code, source context, visible symbols, parser AST, and TypeScript oracle evidence are recorded
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnParameter3.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnParameter3.ts
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

- [x] `issues/open/5278-parse-trailing-comma-in-function-parameters-with-comments.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentOnParameter3.ts`

## Duplicate detection

- `issues/done/5149-parse-trailing-comma-in-typed-class-method-parameters.md`
  tracks a related typed class method trailing comma blocker, but broadening it
  made the work order too large for readiness gates.
- `issues/open/5278-parse-trailing-comma-in-function-parameters-with-comments.md`
  owns this narrower ordinary function declaration/comment representative.
- Other smart-triage duplicate candidates share the broad `parser-syntax` label
  only and do not cover this exact trailing parameter comma diagnostic.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage parser syntax: commentOnParameter3

- Issue class: triage-needed
- Feature label: parser-syntax
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commentOnParameter3.ts
```

Failure location:

```text
issue-247: expected binding identifier or pattern, got Some(RightParen) at 139..140
line 7, column 6
```

Source context:

```text
4 | a /* parameter a */,
5 | b /* parameter b */,
6 | /* extra comment */
7 | ) { }
```

Token evidence:

```text
Function, Ident("commentedParameters"), LeftParen,
Ident("a"), Comma, Ident("b"), Comma, RightParen, LeftBrace, RightBrace
```

TypeScript AST evidence:

```text
FunctionDeclaration "function commentedParameters(a /* parameter a */, b /* parameter b */,) { }"
TypeScript diagnostics: none
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnParameter3.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_features=unknown-unsupported:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnParameter3.ts
result: issue-247 RightParen parser-syntax trailing parameter comma blocker
date: 2026-05-06
```

Remaining risks:

- Issue 5278 intentionally covers only the function declaration/comment
  representative; typed class method trailing commas remain in issue 5149.
