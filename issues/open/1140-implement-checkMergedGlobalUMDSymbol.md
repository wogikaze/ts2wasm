---
id: 1140
title: "Implement Checkmergedglobalumdsymbol"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5231]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1140.

## Summary

Triage checkMergedGlobalUMDSymbol across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkMergedGlobalUMDSymbol` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkMergedGlobalUMDSymbol has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts --detail
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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5231a-parse-export-as-namespace-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts`

## Duplicate detection

Fresh duplicate scan found `issues/open/609-implement-ambientExportDefaultErrors.md`,
which hits the same `export as namespace` syntax, but it is still a broad
blocked generated bucket. This bucket was split to the exact parser slice
instead of depending on another blocked bucket.

Related but no-match:

- `issues/open/609-implement-ambientExportDefaultErrors.md` is a generated
  bucket with the same first syntax boundary.
- Broad import/export buckets are not implementation-ready work orders.

## Smart triage

Fresh triage shows the current blocker is not UMD merge semantics yet. The
frontend stops at `export as namespace THREE;` before AST construction.

### Smart triage: checkMergedGlobalUMDSymbol

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-055: unsupported static export`
- Path: `reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
// @Filename: global.d.ts
import * as _three from './three';

export as namespace THREE;
```

Compiler evidence:

```text
tokens: ok; Export Ident("as") Ident("namespace") Ident("THREE") Semicolon
ast: UnsupportedModule issue-055 at export as namespace
resolved/lowered: same parser/module boundary
TypeScript AST: NamespaceExportDeclaration
TypeScript oracle: TS1315 Global module exports may only appear in declaration files
```

Split result:

- `issues/open/5231a-parse-export-as-namespace-declarations.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts --detail --no-dashboard-data
result: pass; reproduced current unsupported bucket
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts
result: pass; reproduced export-as-namespace issue-055 boundary and split to issue 5231
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5231 may expose virtual module resolution, declare global, or UMD merge semantics blockers.
