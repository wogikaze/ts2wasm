---
id: 1354
title: "Implement Commentonsignature"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: [5280]
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1354.

## Summary

Closed after splitting the current duplicate-function blocker into
`issues/open/5280-validate-commented-top-level-function-overloads.md`.

## Problem

Reference test results show 1 cases fail in directory `commentOnSignature` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: `commentOnSignature1.ts` fails at a top-level function overload
signature plus implementation boundary with comments/trivia around the overload
signatures.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnSignature1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnSignature1.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
DuplicateFunction: duplicate function definition: `foo` at 231..239
unsupported_features=duplicate-function:1
TypeScript oracle: ok, no diagnostics
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5280-validate-commented-top-level-function-overloads.md`.

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnSignature1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnSignature1.ts
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

- [x] `issues/open/5280-validate-commented-top-level-function-overloads.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentOnSignature1.ts`

## Duplicate detection

- `issues/open/5200-validate-top-level-function-overload-implementations.md`
  is related, but broadening it with this comments/signature reference made the
  issue too large for readiness gates.
- `issues/open/5280-validate-commented-top-level-function-overloads.md` owns
  this narrower commented top-level overload group.
- `issues/open/5199-report-function-overload-list-class-merge-diagnostics.md`
  is related but covers function overload lists followed by a class merge.
- `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`
  is related but covers class method overload signatures; it is not the first
  blocker in this path.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage duplicate function: commentOnSignature1

- Issue class: triage-needed
- Feature label: duplicate-function
- Diagnostic: DuplicateFunction / compiler-diagnostic
- Path: reference/typescript/tests/cases/compiler/commentOnSignature1.ts
```

Failure location:

```text
duplicate function definition: `foo` at 231..239
line 12, column 10
```

Source context:

```text
 9 | /*! Don't keep this pinned comment */
10 | function foo(n: number): void;
11 | // Don't keep this comment.
12 | function foo(s: string): void;
13 | function foo(a: any): void {
```

Compiler evidence:

```text
tokens: ok
ast: ok
Function foo(n), Function foo(s), Function foo(a) with body
validate_ast: DuplicateFunction for second bodyless overload signature
```

TypeScript oracle:

```text
ok: true
diagnostics: []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnSignature1.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_features=duplicate-function:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnSignature1.ts
result: DuplicateFunction for valid commented overload signatures; TypeScript ok
date: 2026-05-06
```

Remaining risks:

- Later constructor overload and class method overload blockers in this same
  reference file are not yet proven because issue 5200 owns the first top-level
  overload blocker.
