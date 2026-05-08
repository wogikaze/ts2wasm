---
id: 1383
title: "Implement Commonjsexporttypedeclarationerror"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: [5291]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1383.

## Summary

Closed after splitting the executable parser work into
`issues/open/5291-report-malformed-export-type-declarations.md`.

Fresh triage shows the first blocker is a malformed `export type` declaration
diagnostic/recovery gap, not CommonJS runtime behavior.

## Problem

Reference test results originally showed 1 case failing in directory
`commonJsExportTypeDeclarationError` with diagnostics: import-export. Fresh
focused triage on 2026-05-07 shows tokenization succeeds, but parsing
`export type test` continues until the next `import` token before reporting
`expected Equal`.

Problem: `commonJsExportTypeDeclarationError.ts` currently reports
`expected Equal, got Some(Import)` at the next file section instead of a local
malformed `export type` diagnostic.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5291-report-malformed-export-type-declarations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split this bucket into a malformed `export type` parser diagnostic issue
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
- [x] Child issue contains the exact `export type` malformed declaration parser family
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts
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

- [x] created/updated: `issues/open/5291-report-malformed-export-type-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts`

## Duplicate detection

- No existing focused issue owns malformed `export type Name` parser recovery.
- Broad import/export buckets such as `issues/done/432-implement-import-export.md`
  are not implementation-ready owners for this exact parser diagnostic.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage import export: commonJsExportTypeDeclarationError

- Issue class: triage-needed
- Feature label: import-export
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts
```

Source context:

```text
// @Filename: ./types1.ts
import test from "./test";
export type test

// @Filename: ./types2.ts
import test from "./test";
export type test =
```

Compiler evidence:

```text
tokens: ok; Export, Ident("type"), Ident("test") are present
ast/resolved: expected Equal, got Some(Import) at 266..272
visible symbols: import "./test" from the virtual file sections
```

TypeScript oracle:

```text
TS1005: "=" expected at the following import after `export type test`
TS1005/TS1141/TS2304 diagnostics for later malformed `export type test =`
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts
result: malformed export type parser diagnostic/recovery blocker; split to issue 5291
date: 2026-05-07
```

Remaining risks:

- After issue 5291 advances, this path may reveal CommonJS `module.exports`,
  default import, or module-resolution blockers.
