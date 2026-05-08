---
id: 1291
title: "Implement Collisionexportsrequireandclass"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1291.

## Summary

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts`
currently stops at the entry-module `export class` issue-5005 boundary already
owned by `issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionExportsRequireAndClass` with diagnostics: import-export. Fresh triage
shows the first current compiler blocker is the shared entry-module
`ExportDecl(ClassDecl)` boundary.

Problem: the generated bucket does not need a new child issue. The actionable
slice is already tracked by issue 5232, which supports entry-module
`export class Name {}` declarations.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`. Do not implement
directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5232 covers the current first blocker
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
- [x] Existing issue 5232 contains the implementation-ready `export class` slice
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts
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

- [x] none; current first blocker is already tracked by issue 5232

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts`

## Duplicate detection

Current first blocker is covered by
`issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`.

Resolution:

```text
Superseded by issue 5232. The active diagnostic is issue-5005 for an entry-module `ExportDecl(ClassDecl)` form, which 5232 owns directly.
```

## Smart triage

### Smart triage: Triage import export: collisionExportsRequireAndClass

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts --detail --no-dashboard-data
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
UnsupportedModule: issue-5005: entry module `export require` uses a declaration form outside the current static export slice; only export const and export default are supported at 107..125
```

Source context:

```ts
export class require {
}
export class exports {
}
namespace m1 {
    class require {
    }
    class exports {
    }
}
```

Compiler evidence:

```text
tokens: ok; includes export class require/exports, namespace m1/m2/m3/m4, and class require/exports declarations
ast: ok; ExportDecl ClassDecl require, ExportDecl ClassDecl exports, plus global ClassDecl require/exports
resolved: fails after module build with DuplicateFunction duplicate constructor definition: `require`
visible symbols before failure: []
```

TypeScript oracle evidence:

```text
TS2395: Individual declarations in merged declaration 'require' must be all exported or all local.
TS2395: Individual declarations in merged declaration 'exports' must be all exported or all local.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as superseded; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; reproduced issue-5005 entry-module export-class boundary owned by issue 5232
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndClass.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; executed=1, unsupported=1, unsupported_features=import-export
date: 2026-05-07
```

Remaining risks:

- After issue 5232 lands, this path may expose the currently hidden
  `DuplicateFunction: duplicate constructor definition: require` behavior or
  TS2395 diagnostic parity for mixed exported/local declarations.
