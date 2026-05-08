---
id: 3370
title: "Implement Moduleresolution Import Export"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432, 5229, 5421]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as split/superseded. Fresh coverage still reports the 5 original
import/export paths as unsupported, but smart triage shows their concrete
current blockers are module graph issue-232 shapes already owned by issue 5229
or split to new issue 5421.

## Problem

Reference test results show 5 cases fail in directory
`moduleResolution-import-export` with diagnostics: import-export. Fresh coverage
on 2026-05-08 reports:

```text
moduleResolution_explicitNodeModulesImport.ts: UnsupportedModule/import-export
moduleResolution_classicPrefersTs.ts: UnsupportedModule/import-export
moduleResolution_explicitNodeModulesImport_implicitAny.ts: UnsupportedModule/import-export
moduleResolution_relativeImportJsFile.ts: UnsupportedModule/import-export
moduleResolution_relativeImportJsFile_noImplicitAny.ts: UnsupportedModule/import-export
```

Problem: this generated bucket is too broad for direct implementation. Four
paths are virtual `@Filename` local module resolution misses covered by issue
5229, and the remaining classic bare import case is split to issue 5421.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolution_explicitNodeModulesImport.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_explicitNodeModulesImport.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue or owner

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
- [x] Existing issue 5229 owns the virtual local `@Filename` section resolution misses
- [x] New issue 5421 owns the classic bare import resolution slice
- [x] Child/owner evidence includes failing path, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence, and exact diagnostic/stdout change

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_ --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_explicitNodeModulesImport.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_classicPrefersTs.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_explicitNodeModulesImport_implicitAny.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_relativeImportJsFile.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_relativeImportJsFile_noImplicitAny.ts
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Not run:

- Cargo gates; no Rust source changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created `issues/open/5421-resolve-classic-module-resolution-bare-imports.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolution_explicitNodeModulesImport.ts`
- `reference/typescript/tests/cases/compiler/moduleResolution_classicPrefersTs.ts`
- `reference/typescript/tests/cases/compiler/moduleResolution_explicitNodeModulesImport_implicitAny.ts`
- `reference/typescript/tests/cases/compiler/moduleResolution_relativeImportJsFile.ts`
- `reference/typescript/tests/cases/compiler/moduleResolution_relativeImportJsFile_noImplicitAny.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/open/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/done/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_ --detail --no-dashboard-data
```

Coverage result:

```text
executed=13
unsupported=13
unsupported_diagcodes=UnsupportedSyntax:7,UnsupportedModule:5,UnresolvedName:1
unsupported_features=module-resolution:7,import-export:5,name-resolution:1
```

The 5 paths owned by this bucket are the `UnsupportedModule/import-export`
entries.

Representative smart triage:

```text
moduleResolution_explicitNodeModulesImport.ts:
  ast: PropertyAssign exports.x, ImportNamed "../node_modules/foo"
  module_graph: issue-232 missing local module `../node_modules/foo`
  TypeScript: TS2304 exports, TS2307 ../node_modules/foo

moduleResolution_explicitNodeModulesImport_implicitAny.ts:
  ast: PropertyAssign exports.x, ImportNamed "../node_modules/foo"
  module_graph: issue-232 missing local module `../node_modules/foo`
  TypeScript: TS2304 exports, TS2307 ../node_modules/foo

moduleResolution_relativeImportJsFile.ts:
  ast: ExportDecl const x, ImportNamespace "./b"
  module_graph: issue-232 missing local module `./b`
  TypeScript: TS2307 ./b

moduleResolution_relativeImportJsFile_noImplicitAny.ts:
  ast: ExportDecl const x, ImportNamespace "./b"
  module_graph: issue-232 missing local module `./b`
  TypeScript: TS2307 ./b

moduleResolution_classicPrefersTs.ts:
  ast: two ExportDefault string declarations and ImportDefault source "a"
  module_graph: issue-232 unsupported non-local module specifier `a`
  TypeScript raw-source oracle: TS2528 duplicate default exports and TS2307 a
```

Ownership:

- `issues/open/5229-resolve-imports-between-filename-sections.md` owns the
  virtual local `@Filename` section misses for `./b` and `../node_modules/foo`.
- `issues/open/5421-resolve-classic-module-resolution-bare-imports.md` owns the
  classic bare import `a` resolution slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- local closure commit; see git log for this issue file

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_ --detail --no-dashboard-data
result: pass; 5 import-export paths still unsupported and mapped to issue 5229 / 5421
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_explicitNodeModulesImport.ts
result: pass; resolved dump reports issue-232 missing local module ../node_modules/foo
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_classicPrefersTs.ts
result: pass; resolved dump reports issue-232 unsupported non-local module specifier a
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_explicitNodeModulesImport_implicitAny.ts
result: pass; resolved dump reports issue-232 missing local module ../node_modules/foo
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_relativeImportJsFile.ts
result: pass; resolved dump reports issue-232 missing local module ./b
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_relativeImportJsFile_noImplicitAny.ts
result: pass; resolved dump reports issue-232 missing local module ./b
date: 2026-05-08
```

Remaining risks:

- After issues 5229 and 5421 land, these references may expose CommonJS emit,
  noImplicitAny, duplicate default export, or deeper module-resolution parity
  work.
