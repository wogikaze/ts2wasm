---
id: 1120
title: "Implement Castparentheses"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5192]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage castParentheses across 1 failing reference test case and fold the current blocker into an implementation-ready issue.

## Problem

Reference test results showed 1 case failing in directory `castParentheses` with diagnostics: unknown-unsupported. Fresh triage shows the parser handles the cast-parentheses forms, and the current blocker is `issue-5011` class constructor value usage.

Problem: castParentheses has 1 reference failure whose actionable blocker is now tracked by `issues/open/5192-support-first-class-class-constructor-values.md`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castParentheses.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castParentheses.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/open/5192-support-first-class-class-constructor-values.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold the current observable blocker into issue 5192
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this issue and issue 5192

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
- [x] Issue 5192 contains an exact `mise run reference-triage -- ...` command
- [x] Issue 5192 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5192 acceptance names the exact reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castParentheses.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castParentheses.ts
```

Not run:

- `cargo fmt --all --check`; issue triage only, no Rust code changed
- `cargo nextest run`; issue triage only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] updated: `issues/open/5192-support-first-class-class-constructor-values.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/castParentheses.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castParentheses.ts`
- issue class: `triage-needed`
- feature label: `class`
- diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- message: `issue-5011: class a cannot be used as a value - class runtime is not yet supported at 73..74`
- follow-up: `issues/open/5192-support-first-class-class-constructor-values.md`

Source context:

```text
 4 | }
 5 |
 6 | var b = (<any>a);
 7 | var b = (<any>a).b;
 8 | var b = (<any>a.b).c;
 9 | var b = (<any>a.b()).c;
10 | var b = (<any>new a);
```

Visible symbols before failure:

```json
[
  { "kind": "class", "name": "a", "line": 2, "column": 1 },
  { "kind": "binding", "name": "b", "line": 6, "column": 1, "initializer": "(<any>a)" }
]
```

Compiler evidence:

```text
tokens: ok
AST: ClassDecl `a`; Let b = Ident("a") for `(<any>a)`; later cast/new/member forms are also parsed
resolved: issue-5011 at identifier `a`
TypeScript oracle: ok, diagnostics: []
```

## Completion evidence

Closed as a generated triage bucket. The actionable first blocker is tracked by
`issues/open/5192-support-first-class-class-constructor-values.md`.

Commits:

- this fold commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castParentheses.ts
result: fail with issue-5011 class constructor value diagnostic at `(<any>a)`; folded into issue 5192
date: 2026-05-06
```

Remaining risks:

- After issue 5192 is implemented, later static member or `new a.b` forms in this reference path may expose follow-up blockers.
