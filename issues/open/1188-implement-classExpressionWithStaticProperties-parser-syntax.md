---
id: 1188
title: "Implement Classexpressionwithstaticproperties Parser Syntax"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5254]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1188.

## Summary

Triage classExpressionWithStaticProperties-parser-syntax across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `classExpressionWithStaticProperties-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExpressionWithStaticProperties-parser-syntax has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties1.ts
```

Not run:

- cargo gates; issue split only, no implementation changes

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5254-parse-asi-between-static-class-fields.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties1.ts`
- `reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties2.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/done/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/open/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/open/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)
- `issues/open/773-implement-autoAsiForStaticsInClassDeclaration.md` has the same `static x` / `static y` ASI parser boundary for class declarations; issue 5254 covers both declaration and expression forms.

## Smart triage

Fresh triage on 2026-05-06:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties2.ts
```

Result:

```text
classExpressionWithStaticProperties1.ts: BuildPass
classExpressionWithStaticProperties2.ts: UnsupportedSyntax: expected LeftParen, got Some(Static) at 99..105
```

Compiler dump evidence for file 2:

```text
tokens: ok; includes `static b` followed by `static c = { ... }`
ast: fails at the second `static`
resolved: same parser failure
```

TypeScript oracle:

```text
ok; diagnostics=[]
AST: ClassExpression with `static b`, `static c = { x: "hi" }`, and `static d = ...`
```

The executable child issue is the narrow ASI parser slice for static class
fields, captured in issue 5254.

## Completion evidence

Commits:

- local split commit for issue 1188 / child 5254

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties --detail --no-dashboard-data
result: pass; executed=7, build_pass=2, unsupported=5
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties1.ts
result: pass; BuildPass
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties2.ts
result: pass; current blocker is static class field ASI, split to issue 5254
date: 2026-05-06
```

Remaining risks:

- Static public field runtime/lowering parity remains outside issue 5254.
