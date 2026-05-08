---
id: 1087
title: "Implement Bundleddtslateexportrenaming"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [232]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage bundledDtsLateExportRenaming across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `bundledDtsLateExportRenaming` with diagnostics: import-export. Fresh smart triage shows the compiler parses the static re-export and reports the existing issue-232 missing local module diagnostic for `./nested`; TypeScript's raw-source oracle also reports TS2307 for the same module specifier.

Problem: `bundledDtsLateExportRenaming` is not a standalone implementation order in the current runner view; the current failure is an oracle-matching missing local module diagnostic covered by issue 232 module graph behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bundledDtsLateExportRenaming.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bundledDtsLateExportRenaming.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/open/232-resolve-local-relative-es-module-graph.md` for the current missing local module diagnostic. Do not implement directly from this bucket.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bundledDtsLateExportRenaming.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bundledDtsLateExportRenaming.ts
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

- `reference/typescript/tests/cases/compiler/bundledDtsLateExportRenaming.ts`

## Duplicate detection

- `issues/open/232-resolve-local-relative-es-module-graph.md` owns source-spanned missing local module diagnostics for static imports and re-exports.
- Declaration emit issues are not exact matches for the current blocker because triage stops at missing module resolution before declaration emit behavior.

## Smart triage

### Smart triage: import export: bundledDtsLateExportRenaming

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/bundledDtsLateExportRenaming.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bundledDtsLateExportRenaming.ts
```

Source context:

```text
// @Filename: index.ts
export * from "./nested";
```

Current compiler failure:

```text
error: [UnsupportedModule] issue-232: missing local module `./nested` re-exported from index.ts; tried ./nested.ts, ./nested.js at 14..24
```

Compiler evidence:

- Tokens succeed for `export * from "./nested";`.
- AST succeeds as `ExportAllFrom` with source module specifier `"./nested"`.
- Module graph validation rejects the unresolved `./nested` specifier before name resolution or declaration emit.

TypeScript oracle evidence:

```text
TS2307: Cannot find module './nested' or its corresponding type declarations.
```

The oracle reports the diagnostic at the same module specifier and later reports additional missing modules and declaration-merge diagnostics.

Resolution:

```text
Issue 232 established missing local module diagnostics for static module graph construction. The current reference-triage failure is the same missing module boundary rather than an actionable declaration-emit or export-renaming slice.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/open/232-resolve-local-relative-es-module-graph.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bundledDtsLateExportRenaming.ts
result: pass; reproduced oracle-matching missing local module diagnostic for `./nested`
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

