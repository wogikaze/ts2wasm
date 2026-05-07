---
id: 1435
title: "Implement Conflictingdeclarationsimportfromnamespace"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1435.

## Summary

Triage conflictingDeclarationsImportFromNamespace across 2 failing reference
test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory
`conflictingDeclarationsImportFromNamespace` with diagnostics: import-export.
Fresh triage on 2026-05-07 shows both paths stop at the same
`ExportAssignment` / `export = ...` coexistence blocker split into issue 5306.

Problem: conflictingDeclarationsImportFromNamespace has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace1.ts --detail
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
- [x] Child issue 5306 contains exact `mise run reference-triage -- ...` commands
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5306-report-export-assignment-with-other-exports.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace1.ts`
- `reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace2.ts`

## Duplicate detection

- `issues/done/548-implement-ExportAssignment.md` covers the same
  `ExportAssignment` plus other exported elements behavior for
  `ExportAssignment7.ts` and `ExportAssignment8.ts`.
- Split into `issues/open/5306-report-export-assignment-with-other-exports.md`.

## Smart triage

### Smart triage: Triage import export: conflictingDeclarationsImportFromNamespace1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 730,
  "lines": 31,
  "extension": ".ts",
  "first_code_line": "import _ = require(\"./index\");"
}
```

Compiler failure:

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 406..412
```

TypeScript oracle includes `ExportAssignment` at `export = pick;`,
`ExportAssignment` at `export = _;`, `NamespaceExportDeclaration` at
`export as namespace _;`, and TS2309:

```text
An export assignment cannot be used in a module with other exported elements.
```

### Smart triage: Triage import export: conflictingDeclarationsImportFromNamespace2

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace2.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 734,
  "lines": 31,
  "extension": ".ts",
  "first_code_line": "import _ = require(\"./index\");"
}
```

Compiler failure:

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 410..416
```

TypeScript oracle includes `ExportAssignment` at `export = pick;`,
`ExportAssignment` at `export = _;`, `NamespaceExportDeclaration` at
`export as namespace _;`, `export const pick = () => pick();`, and TS2309:

```text
An export assignment cannot be used in a module with other exported elements.
```

Coverage result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conflictingDeclarationsImportFromNamespace --detail --no-dashboard-data
result: pass; executed=2 unsupported=2 unsupported_features=import-export:2
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split into `issues/open/5306-report-export-assignment-with-other-exports.md`

Validation result:

```text
command: python scripts/manager.py update-issue-index; python scripts/manager.py update-issue-index --check; python scripts/manager.py check-issue-health; python scripts/manager.py check-issue-readiness -- --fail-ready-below 80; git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
