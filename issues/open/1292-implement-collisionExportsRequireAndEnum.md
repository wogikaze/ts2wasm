---
id: 1292
title: "Implement Collisionexportsrequireandenum"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1292.

## Summary

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts`
currently stops at the `export enum` issue-055 module-syntax boundary already
owned by `issues/done/5277-parse-export-enum-declarations-to-enum-boundary.md`.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionExportsRequireAndEnum` with diagnostics: import-export. Fresh triage
shows the first current compiler blocker is the shared `export enum` boundary.

Problem: the generated bucket does not need a new child issue. The actionable
slice is already tracked by issue 5277, which accepts `export enum Name { ... }`
far enough to reach the enum-specific boundary.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/done/5277-parse-export-enum-declarations-to-enum-boundary.md`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5277 covers the current first blocker
- [x] Supersede this bucket instead of creating a duplicate child issue
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

- [x] Duplicate candidates below are confirmed and this issue is superseded
- [x] Existing issue 5277 contains the implementation-ready `export enum` slice
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts
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

- [x] none; current first blocker is already tracked by issue 5277

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts`

## Duplicate detection

Current first blocker is covered by
`issues/done/5277-parse-export-enum-declarations-to-enum-boundary.md`.

Resolution:

```text
Superseded by issue 5277. The active diagnostic is issue-055 for an `export enum` static export form, which 5277 owns directly.
```

## Smart triage

### Smart triage: Triage import export: collisionExportsRequireAndEnum

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Current diagnostic:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 99..105
```

Source context:

```ts
export enum require { // Error
    _thisVal1,
    _thisVal2,
}
export enum exports { // Error
    _thisVal1,
    _thisVal2,
}
```

Compiler evidence:

```text
tokens: ok; includes export enum require/exports and namespace m1/m2/m3/m4 enum declarations
ast: fails with issue-055 unsupported static export at export enum
resolved: fails with issue-055 unsupported static export at export enum
visible symbols before failure: []
```

TypeScript oracle evidence:

```text
TS2395: Individual declarations in merged declaration 'require' must be all exported or all local.
TS2395: Individual declarations in merged declaration 'exports' must be all exported or all local.
TypeScript AST: EnumDeclaration with ExportKeyword for `export enum require`.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as superseded; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; reproduced issue-055 export-enum boundary owned by issue 5277
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndEnum.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; executed=1, unsupported=1, unsupported_features=import-export
date: 2026-05-07
```

Remaining risks:

- After issue 5277 lands, this path may expose enum-specific lowering or TS2395
  diagnostic parity for mixed exported/local declarations.
