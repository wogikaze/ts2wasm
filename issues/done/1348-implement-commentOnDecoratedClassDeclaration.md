---
id: 1348
title: "Implement Commentondecoratedclassdeclaration"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: [5276]
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed after splitting the current parser blocker into
`issues/open/5276-report-class-declaration-decorator-boundary.md`.

## Problem

Reference test results show 1 cases fail in directory `commentOnDecoratedClassDeclaration` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: `commentOnDecoratedClassDeclaration.ts` fails in lexer/parser before
decorated class declarations can reach the TypeScript decorator boundary.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
UnsupportedSyntax: unsupported character: @ at 123..124
line 8, column 1
unsupported_features=unknown-unsupported:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5276-report-class-declaration-decorator-boundary.md`.

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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts
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

- [x] `issues/open/5276-report-class-declaration-decorator-boundary.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts`

## Duplicate detection

- `issues/open/059-implement-parser-syntax-extensions.md` is a broad parser
  syntax epic, not an exact implementation slice.
- `issues/open/4807-implement-decorator.md` is a broad decorator bucket, not an
  exact implementation slice.
- `issues/open/5253-report-class-expression-decorator-boundary.md` covers class
  expression decorators in expression position. This issue needs a separate
  statement/declaration-position class decorator slice.
- Other smart-triage duplicate candidates share `parser-syntax` only and do not
  cover this decorated class declaration path.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage parser syntax: commentOnDecoratedClassDeclaration

- Issue class: triage-needed
- Feature label: parser-syntax
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts
```

Failure location:

```text
unsupported character: @ at 123..124
line 8, column 1
```

Source context:

```text
 5 | /**
 6 |  * Leading trivia
 7 |  */
 8 | @decorator("hello")
 9 | class Remote { }
10 |
11 | /**
```

Visible symbols before failure:

```text
function decorator(x: string)
```

TypeScript AST evidence:

```text
SourceFile
- FunctionDeclaration "declare function decorator(x: string): any;"
- ClassDeclaration "@decorator(\"hello\")\nclass Remote { }"
  - Decorator "@decorator(\"hello\")"
- ClassDeclaration "@decorator(\"hi\")\nclass AnotherRomote { ... }"
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_features=unknown-unsupported:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts
result: UnsupportedSyntax / parser-or-frontend-unsupported
date: 2026-05-06
```

Remaining risks:

- The child issue intentionally stops at a frontend decorator boundary; full
  decorator transform/runtime semantics remain out of scope.
