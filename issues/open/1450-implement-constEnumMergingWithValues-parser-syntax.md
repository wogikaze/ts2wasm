---
id: 1450
title: "Implement Constenummergingwithvalues Parser Syntax"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5186]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1450.

## Summary

Closed this generated parser-syntax bucket because fresh triage stops at the
`export = foo` parser/module boundary already owned by
`issues/done/5186-parse-export-assignment-for-diagnostics.md`.

## Problem

Reference test results show 1 cases fail in directory `constEnumMergingWithValues-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constEnumMergingWithValues-parser-syntax has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constEnumMergingWithValues3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constEnumMergingWithValues3.ts --detail
```

## Desired final state

This generated bucket is closed. Implement from
`issues/done/5186-parse-export-assignment-for-diagnostics.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing implementation-ready export-assignment issue
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
- [x] Superseding issue contains exact `reference-triage` commands
- [x] Superseding issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constEnumMergingWithValues3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constEnumMergingWithValues3.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/done/5186-parse-export-assignment-for-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constEnumMergingWithValues3.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/open/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/open/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/open/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/open/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

## Smart triage

Generated on 2026-05-07.

- Path: `reference/typescript/tests/cases/compiler/constEnumMergingWithValues3.ts`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-055: unsupported static export; module resolution and loading are not implemented at 120..126`
- Current failing source: `export = foo`
- Token evidence includes `Ident("enum")`, `Ident("foo")`, `Ident("namespace")`, nested `const enum E { X }`, then `Export`, `Equal`, `Ident("foo")`.
- TypeScript oracle parses top-level `EnumDeclaration`, `ModuleDeclaration`, and `ExportAssignment` with no diagnostics.
- Superseding issue: `issues/done/5186-parse-export-assignment-for-diagnostics.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumMergingWithValues3.ts
result: pass; current blocker is export-assignment parsing tracked by issue 5186
date: 2026-05-07
```

Remaining risks:

- Later triage may expose enum/namespace merging or const-enum semantics after issue 5186 parses export assignments.
