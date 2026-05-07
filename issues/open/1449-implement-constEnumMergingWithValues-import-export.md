---
id: 1449
title: "Implement Constenummergingwithvalues Import Export"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1449.

## Summary

Closed this generated import-export bucket because fresh triage for its four
reference files stops at the `export = foo` parser/module boundary already
owned by `issues/open/5186-parse-export-assignment-for-diagnostics.md`.

## Problem

Reference test results show 4 cases fail in directory `constEnumMergingWithValues-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constEnumMergingWithValues-import-export has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constEnumMergingWithValues2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constEnumMergingWithValues2.ts --detail
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5186-parse-export-assignment-for-diagnostics.md`.

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
- [x] Superseding issue acceptance names the exact reference paths and diagnostic changes

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 8
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constEnumMergingWithValues2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constEnumMergingWithValues2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/open/5186-parse-export-assignment-for-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constEnumMergingWithValues2.ts`
- `reference/typescript/tests/cases/compiler/constEnumMergingWithValues4.ts`
- `reference/typescript/tests/cases/compiler/constEnumMergingWithValues1.ts`
- `reference/typescript/tests/cases/compiler/constEnumMergingWithValues5.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/open/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/open/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

Generated on 2026-05-07 for all four listed files.

Common current failure:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented
source: export = foo
```

Per-file spans:

- `constEnumMergingWithValues1.ts`: `export` at `123..129`
- `constEnumMergingWithValues2.ts`: `export` at `118..124`
- `constEnumMergingWithValues4.ts`: `export` at `144..150`
- `constEnumMergingWithValues5.ts`: `export` at `133..139`

TypeScript oracle parses `ExportAssignment` for each file and reports no
diagnostics. The files differ in the value merged with namespace `foo`
(`function`, `class`, namespace/var, and `preserveConstEnums`), but the current
compiler boundary is identical: the frontend does not represent `export = foo`.

Superseding issue: `issues/open/5186-parse-export-assignment-for-diagnostics.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumMergingWithValues2.ts
result: pass; current blocker is export-assignment parsing tracked by issue 5186
date: 2026-05-07
```

Remaining risks:

- Later triage may expose namespace/value merging or const-enum semantics after issue 5186 parses export assignments.
