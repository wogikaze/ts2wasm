---
id: 1024
title: "Implement Awaitedtypenolib"
type: spike
area: runtime/builtins
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

Triage awaitedTypeNoLib across 1 failing reference test case and split this generated bucket into a smaller implementation-ready child issue.

## Problem

Reference test results show 1 case fails in directory `awaitedTypeNoLib` with diagnostics: parser-syntax. Fresh triage shows the concrete blocker is a trailing comma in a typed class method parameter list.

Problem: awaitedTypeNoLib has 1 reference failure that is now tracked by child issue 5149.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/open/5149-parse-trailing-comma-in-typed-class-method-parameters.md`.

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
- [x] Child issue 5149 contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts
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

- [x] created: `issues/open/5149-parse-trailing-comma-in-typed-class-method-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: awaitedTypeNoLib

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-247: expected binding identifier or pattern, got Some(RightParen) at 424..425",
  "span_start": 424,
  "span_end": 425,
  "line": 20,
  "column": 15,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
16 |   private handleResolve<TResult>(
17 |     result: NotPromise<TResult> | Thenable<NotPromise<TResult>>,
18 |     resolve: Receiver<TResult>,
19 |   ) {
20 |     if (result instanceof Thenable) {
```

Compiler evidence:

```text
tokens include `resolve: Receiver<TResult>, )`.
AST/resolved: parser fails before AST with issue-247 expected binding identifier or pattern at the closing `)`.
TypeScript oracle: ok, no diagnostics.
```

## Completion evidence

Commits:

- pending local commit

Validation result:

```text
command:
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts
result:
pass; emitted UnsupportedSyntax parser-syntax report for a trailing comma in typed class method parameters; split to issue 5149
date:
2026-05-06
```

Remaining risks:

- none
