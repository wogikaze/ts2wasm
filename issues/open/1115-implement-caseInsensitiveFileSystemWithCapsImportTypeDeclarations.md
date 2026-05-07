---
id: 1115
title: "Implement Caseinsensitivefilesystemwithcapsimporttypedeclarations"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [232]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1115.

## Summary

Triage caseInsensitiveFileSystemWithCapsImportTypeDeclarations across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `caseInsensitiveFileSystemWithCapsImportTypeDeclarations` with diagnostics: import-export. Fresh triage shows the current runner view parses the file and stops on existing issue-232 missing local module diagnostics for the virtual `// @filename:` module layout.

Problem: caseInsensitiveFileSystemWithCapsImportTypeDeclarations is not a standalone implementation order in the current runner view; the current failure is an oracle-matching missing local module diagnostic covered by issue 232 module graph behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/caseInsensitiveFileSystemWithCapsImportTypeDeclarations.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/caseInsensitiveFileSystemWithCapsImportTypeDeclarations.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/done/232-resolve-local-relative-es-module-graph.md` for the current missing local module diagnostic. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 232's missing local module diagnostic behavior
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/caseInsensitiveFileSystemWithCapsImportTypeDeclarations.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/caseInsensitiveFileSystemWithCapsImportTypeDeclarations.ts
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

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/caseInsensitiveFileSystemWithCapsImportTypeDeclarations.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: caseInsensitiveFileSystemWithCapsImportTypeDeclarations

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/caseInsensitiveFileSystemWithCapsImportTypeDeclarations.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/caseInsensitiveFileSystemWithCapsImportTypeDeclarations.ts
```

Current compiler failure:

```text
error: [UnsupportedModule] issue-232: missing local module `./type-b` imported from .../caseInsensitiveFileSystemWithCapsImportTypeDeclarations.ts; tried ./type-b.ts, ./type-b.js, ./type-b.d.ts, ./type-b.tsx, ./type-b.mjs, ./type-b.cjs at 167..177
```

Source context:

```text
5 | // @filename: Uppercased_Dir/src/index.ts
6 | import { TypeB } from './type-b';
7 |
8 | export class Broken {
```

Compiler evidence:

- Tokens succeed for the named imports, exported class, and type aliases.
- AST succeeds for `ImportNamed`, `ExportDecl(ClassDecl Broken)`, and later
  dependency imports.
- Module graph validation rejects `./type-b` before declaration emit or
  case-insensitive virtual file behavior.

TypeScript oracle evidence:

```text
TS2307: Cannot find module './type-b' or its corresponding type declarations.
TS2307: Cannot find module './types' or its corresponding type declarations.
TS2307: Cannot find module './type-a' or its corresponding type declarations.
```

Resolution:

```text
Issue 232 established source-spanned missing local module diagnostics for static
module graph construction. The current reference-triage failure is the same
missing-module boundary rather than an actionable declaration emit or
case-insensitive filesystem slice in this runner view.
```

## Completion evidence

caseInsensitiveFileSystemWithCapsImportTypeDeclarations triage is complete.
The current failure is superseded by issue 232 missing local module diagnostics.

Commits:

- superseded by `issues/done/232-resolve-local-relative-es-module-graph.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/caseInsensitiveFileSystemWithCapsImportTypeDeclarations.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedModule import-export
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/caseInsensitiveFileSystemWithCapsImportTypeDeclarations.ts
result: pass; AST succeeds and module_graph reports issue-232 missing local module `./type-b`
date: 2026-05-06
```

Remaining risks:

- none
