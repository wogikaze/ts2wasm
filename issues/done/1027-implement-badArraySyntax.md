---
id: 1027
title: "Implement Badarraysyntax"
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

Triage badArraySyntax across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `badArraySyntax` with diagnostics: parser-syntax. Fresh smart triage shows the failure is the same empty element access diagnostic gap already split to `issues/open/5150-report-empty-element-access-diagnostics.md`.

Problem: `badArraySyntax` is not a standalone implementation order; it is another reference path for issue 5150.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badArraySyntax.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badArraySyntax.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by issue 5150. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing empty element access child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the closed bucket

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
- [x] This closed bucket contains the exact `python scripts/manager.py reference-triage ...` command and issue 5150 owns the same empty element access diagnostic family
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5150 acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badArraySyntax.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/badArraySyntax.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] updated: `issues/open/5150-report-empty-element-access-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/badArraySyntax.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: unknown unsupported: badArraySyntax

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/badArraySyntax.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badArraySyntax.ts
```

Source context:

```text
 7 | var a1: Z[] = [];
 8 | var a2 = new Z[];
 9 | var a3 = new Z[]();
10 | var a4: Z[] = new Z[];
11 | var a5: Z[] = new Z[]();
12 | var a6: Z[][] = new   Z     [      ]   [  ];
```

Current compiler failure:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightBracket, span: Span { start: 106, end: 107 } }) at 107..108
```

Token evidence:

```text
Var a2 = New Ident("Z") LeftBracket RightBracket ;
```

TypeScript oracle evidence:

```text
TS1011: An element access expression should take an argument.
AST path: NewExpression `new Z[]` -> ElementAccessExpression `Z[]`.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/open/5150-report-empty-element-access-diagnostics.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badArraySyntax.ts
result: pass; reproduced generic UnsupportedSyntax on empty element access in `new Z[]`
date: 2026-05-06
```

Remaining risks:

- none
