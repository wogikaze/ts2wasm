---
id: 1074
title: "Implement Blockscopedfunctiondeclarationinstrictmodule"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5186]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedFunctionDeclarationInStrictModule across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `blockScopedFunctionDeclarationInStrictModule` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedFunctionDeclarationInStrictModule has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictModule.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictModule.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictModule.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictModule.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/done/5186-parse-export-assignment-for-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictModule.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` is a broad import/export triage bucket, not a focused implementation-ready owner for `export = expr` diagnostics.
- Existing export-assignment generated buckets share the broad module feature label but do not contain the strict-module block-scoped function diagnostic slice.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictModule.ts`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-055: unsupported static export; module resolution and loading are not implemented at 102..108`
- Source context: `if (true) { function foo() { } foo(); } export = foo;`
- Visible symbols before failure: function `foo` declared inside the `if` block.
- Compiler evidence: tokenization succeeds and includes `Export`, `Equal`, `Ident("foo")`, and `Semicolon`; AST construction fails at `export` before an export-assignment node or expression diagnostic can be produced.
- TypeScript oracle: `TS2304: Cannot find name 'foo'.` at byte `111`, the `foo` identifier in `export = foo`.
- Superseding child: `issues/done/5186-parse-export-assignment-for-diagnostics.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictModule.ts
result: pass; current blocker is unsupported export-assignment syntax before the intended unresolved-name diagnostic, split to issue 5186
date: 2026-05-06
```

Remaining risks:

- none
