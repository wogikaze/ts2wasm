---
id: 1351
title: "Implement Commentonimportstatement"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1351.

## Summary

Closed as superseded by `issues/done/232-resolve-local-relative-es-module-graph.md`.
All three current `commentOnImportStatement` representatives parse far enough
to reach the completed issue-232 missing local module diagnostic for `./foo`,
and TypeScript reports TS2307 for the same specifier.

## Problem

Reference test results show 3 cases fail in directory `commentOnImportStatement` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: `commentOnImportStatement` is not a standalone implementation order in
the current runner view; the current failure is an oracle-matching missing local
module diagnostic covered by issue 232 module graph behavior.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnImportStatement2.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnImportStatement --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
commentOnImportStatement1.ts: issue-232 missing local module `./foo`
commentOnImportStatement2.ts: issue-232 missing local module `./foo`
commentOnImportStatement3.ts: issue-232 missing local module `./foo`
TypeScript oracle: TS2307 Cannot find module './foo'
```

## Desired final state

This generated bucket is closed as superseded by issue 232's missing local
module diagnostic behavior. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 232's missing local module diagnostic behavior
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
- [x] Exact `reference-triage` commands are preserved
- [x] Current path, diagnostic code, source context, visible symbols, parser AST, and TypeScript oracle evidence are recorded
- [x] Completion evidence names the exact reference paths and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnImportStatement --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnImportStatement1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnImportStatement2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnImportStatement3.ts
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

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentOnImportStatement2.ts`
- `reference/typescript/tests/cases/compiler/commentOnImportStatement3.ts`
- `reference/typescript/tests/cases/compiler/commentOnImportStatement1.ts`

## Duplicate detection

- `issues/done/232-resolve-local-relative-es-module-graph.md` owns
  source-spanned missing local module diagnostics for local relative module
  graph construction.
- Prior generated buckets with the same current issue-232 missing-module shape
  were closed as superseded by issue 232, including
  `issues/done/1087-implement-bundledDtsLateExportRenaming.md` and
  `issues/done/1127-implement-chainedImportAlias.md`.
- Other smart-triage duplicate candidates share the broad `import-export` label
  only and do not supersede this exact missing `./foo` diagnostic evidence.

## Smart triage

Generated 2026-05-06 for all three affected paths.

```text
### Smart triage: Triage import export: commentOnImportStatement1
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Message: issue-232: missing local module `./foo` ... at 109..116
- TypeScript oracle: TS2307 Cannot find module './foo'

### Smart triage: Triage import export: commentOnImportStatement2
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Message: issue-232: missing local module `./foo` ... at 111..118
- TypeScript oracle: TS2307 Cannot find module './foo'

### Smart triage: Triage import export: commentOnImportStatement3
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Message: issue-232: missing local module `./foo` ... at 130..137
- TypeScript oracle: TS2307 Cannot find module './foo'
```

Representative source context:

```text
4 | /* not copyright */
5 | import foo = require('./foo');
```

Compiler evidence:

```text
tokens: ok
ast: ok, ImportDefault source "./foo"
resolved: module_graph issue-232 missing local module `./foo`
```

TypeScript AST evidence:

```text
ImportEqualsDeclaration: import foo = require('./foo');
ExternalModuleReference: require('./foo')
StringLiteral: './foo'
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnImportStatement --detail --no-dashboard-data
result: executed=3, unsupported=3, unsupported_features=import-export:3
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnImportStatement1.ts
result: issue-232 missing local module `./foo`; TypeScript TS2307
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnImportStatement2.ts
result: issue-232 missing local module `./foo`; TypeScript TS2307
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnImportStatement3.ts
result: issue-232 missing local module `./foo`; TypeScript TS2307
date: 2026-05-06
```

Remaining risks:

- Coverage still classifies issue-232 missing-module diagnostics under
  `import-export`; this close only removes the generated bucket as a duplicate
  implementation order.
