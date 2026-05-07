---
id: 1442
title: "Implement Constdeclarations Parser Syntax"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5349]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed this generated const-declaration parser bucket after splitting the
current `*=`, `/=`, and `%=` parser boundary to
`issues/open/5349-parse-multiplicative-compound-assignment-operators.md`.

## Problem

Reference test results show 6 cases fail in directory `constDeclarations-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constDeclarations-parser-syntax has 6 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarations-access2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations-access2.ts --detail
```

## Desired final state

This generated bucket is closed. Implement the current parser boundary from
`issues/open/5349-parse-multiplicative-compound-assignment-operators.md`.

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
- [x] Child issue contains exact `reference-triage` commands
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
mise run reference-coverage -- tsc --limit 12
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations-access2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarations-access2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5349-parse-multiplicative-compound-assignment-operators.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constDeclarations-access2.ts`
- `reference/typescript/tests/cases/compiler/constDeclarations-ambient-errors.ts`
- `reference/typescript/tests/cases/compiler/constDeclarations-access.ts`
- `reference/typescript/tests/cases/compiler/constDeclarations-invalidContexts.ts`
- `reference/typescript/tests/cases/compiler/constDeclarations-useBeforeDefinition.ts`
- `reference/typescript/tests/cases/compiler/constDeclarations-validContexts.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/open/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/done/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

## Smart triage

Generated on 2026-05-07.

- Path: `reference/typescript/tests/cases/compiler/constDeclarations-access2.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `expected Semicolon, got Some(StarEqual) at 92..94`
- First failing source line after existing `+=` / `-=` support: `x *= 4;`
- Visible symbol before failure: const binding `x` initialized to `0`
- Compiler evidence: token dump emits `Ident("x")`, `StarEqual`, `Number(4)`, `Semicolon`; AST/resolved construction fails before representing `x *= 4`.
- TypeScript oracle: reports TS2588 "Cannot assign to 'x' because it is a constant." for the assignment and update expressions.
- Superseding child: `issues/open/5349-parse-multiplicative-compound-assignment-operators.md`

Nearby non-owners:

- `issues/done/5178-parse-bitwise-compound-assignment-operators.md` owns bitwise `^=`, `&=`, and `|=`.
- `issues/done/5164-parse-exponentiation-compound-assignment.md` owns `**=`.
- `issues/open/5311-parse-property-access-arithmetic-compound-assignments.md` owns namespace property `+=`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-access2.ts
result: pass; current blocker identified as identifier-target multiplicative compound assignment parser syntax, split to issue 5349
date: 2026-05-07
```

Remaining risks:

- Later constDeclarations triage may expose shift compound assignment, update
  expression, or final const-assignment diagnostics after issue 5349 advances
  past `*=`.
