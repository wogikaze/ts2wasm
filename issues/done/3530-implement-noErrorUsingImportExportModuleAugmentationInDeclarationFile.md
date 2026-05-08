---
id: 3530
title: "Implement Noerrorusingimportexportmoduleaugmentationindeclarationfile"
type: spike
area: frontend/parser
class: superseded
priority: P1
depends_on: [5346, 5285, 5306]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by existing implementation-ready issues for the first blockers in
the three `noErrorUsingImportExportModuleAugmentationInDeclarationFile`
representatives.

## Problem

The bucket contains three generated representatives:

- `noErrorUsingImportExportModuleAugmentationInDeclarationFile1.ts` first stops
  at `export = a;`, owned by `issues/open/5346-parse-commonjs-export-assignment-statements.md`.
- `noErrorUsingImportExportModuleAugmentationInDeclarationFile2.ts` first stops
  at initialized `export var j = "hello";`, owned by
  `issues/open/5285-support-export-var-initializer-declarations.md`.
- `noErrorUsingImportExportModuleAugmentationInDeclarationFile3.ts` first stops
  at `export = a;`, owned by `issues/open/5346-parse-commonjs-export-assignment-statements.md`.

After those parser blockers advance, the invalid `export =` plus other exports
diagnostic is owned by
`issues/open/5306-report-export-assignment-with-other-exports.md`.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile3.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile3.ts --detail --no-dashboard-data
```

Observed coverage on 2026-05-08:

```text
case 1: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=import-export:1
case 2: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=import-export:1
case 3: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=import-export:1
```

Triage evidence:

```text
case 1: tokens include Export Equal Ident("a") Semicolon; AST fails with issue-055 unsupported static export at export = a;
case 2: tokens include Export Var Ident("j") Equal String("hello"); AST fails with issue-055 unsupported variable export at export var j = "hello";
case 3: tokens include Export Equal Ident("a") Semicolon; AST fails with issue-055 unsupported static export at export = a;
```

TypeScript oracle:

```text
case 1: ok, diagnostics=[]
case 2: TS2309, An export assignment cannot be used in a module with other exported elements.
case 3: TS2309, An export assignment cannot be used in a module with other exported elements.
```

## Desired final state

This generated bucket is closed as superseded by issues 5346, 5285, and 5306.
Do not implement directly from this bucket.

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
- [x] Existing owner issues contain exact `python scripts/manager.py reference-triage ...` commands.
- [x] Owner issues include failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Owner issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile3.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile3.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue lifecycle change.
- `cargo nextest run`; metadata-only issue lifecycle change.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5346-parse-commonjs-export-assignment-statements.md`
- [x] `issues/open/5285-support-export-var-initializer-declarations.md`
- [x] `issues/open/5306-report-export-assignment-with-other-exports.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile1.ts`
- `reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile2.ts`
- `reference/typescript/tests/cases/compiler/noErrorUsingImportExportModuleAugmentationInDeclarationFile3.ts`

## Duplicate detection

- `issues/open/5346-parse-commonjs-export-assignment-statements.md` owns
  representatives 1 and 3 at `export = a;`.
- `issues/open/5285-support-export-var-initializer-declarations.md` owns
  representative 2 at `export var j = "hello";`.
- `issues/open/5306-report-export-assignment-with-other-exports.md` owns the
  later TS2309 invalid export-assignment combination in representatives 2 and 3.

## Smart triage

Generated manually on 2026-05-08 from focused `reference-triage` runs:

- case 1: CommonJS export assignment parser blocker, TypeScript oracle accepts.
- case 2: initialized export-var parser blocker, then TS2309.
- case 3: CommonJS export assignment parser blocker, then TS2309.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
