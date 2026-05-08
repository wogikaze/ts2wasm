---
id: 3387
title: "Implement Moduleresolutionwithsymlinks Import Export"
type: maintenance
area: compiler/module-graph
class: superseded
priority: P1
depends_on: [432, 5324, 5426, 5292]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage moduleResolutionWithSymlinks-import-export across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh coverage shows the broader symlink prefix currently has mixed first
blockers. For this bucket's three listed files:

- `moduleResolutionWithSymlinks.ts` and `_withOutDir.ts` reach the existing
  dependency-module `export class` boundary owned by issue 5324.
- `moduleResolutionWithSymlinks_notInNodeModules.ts` needs focused
  `@symlink` alias handling for local imports, split to issue 5426. Its dumps
  also show a later virtual `tsconfig.json` body boundary owned by issue 5292.

Problem: this generated import/export bucket is not one executable work item;
its current failures are covered by narrower issues.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_notInNodeModules.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks --detail --no-dashboard-data
```

Observed result:

```text
executed=5
unsupported=5
unsupported_diagcodes=UnsupportedSyntax:3,UnsupportedModule:2
unsupported_features=module-resolution:3,import-export:2
```

## Desired final state

This generated bucket is closed as superseded by:

- `issues/open/5324-support-dependency-export-class-declarations.md`
- `issues/open/5426-resolve-symlink-filename-aliases-for-local-imports.md`
- `issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one focused child issue for the unmatched symlink-alias behavior
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/compiler/src/module_graph.rs`
- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue 5426 contains an exact reference-triage command
- [x] Child and owner issues include failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Follow-up issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

Issue-only close; Rust gates were not required for this lifecycle split.

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_notInNodeModules.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_withOutDir.ts
```

Not run:

- cargo fmt --all --check
- cargo nextest run

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5324-support-dependency-export-class-declarations.md`
- [x] `issues/open/5426-resolve-symlink-filename-aliases-for-local-imports.md`
- [x] `issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_notInNodeModules.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_withOutDir.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/open/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/open/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/open/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)
- Existing issue 5324 owns the dependency `export class MyClass` boundary in
  `moduleResolutionWithSymlinks.ts` and `_withOutDir.ts`.
- No existing issue owned `@symlink` aliases for local imports in
  `moduleResolutionWithSymlinks_notInNodeModules.ts`; split to issue 5426.
- Existing issue 5292 owns the later `/src/tsconfig.json` section parsing
  boundary visible in the same `notInNodeModules` dumps.

## Smart triage

Fresh coverage:

```text
executed=5
unsupported=5
unsupported_diagcodes=UnsupportedSyntax:3,UnsupportedModule:2
unsupported_features=module-resolution:3,import-export:2
```

`moduleResolutionWithSymlinks.ts` and `_withOutDir.ts`:

```text
failure: UnsupportedModule issue-5005 dependency module declaration export uses a form outside the current static export slice
resolved dump: module graph also reaches unsupported non-local module specifier `library-a`
owner: issue 5324 for the immediate dependency export-class boundary
```

`moduleResolutionWithSymlinks_notInNodeModules.ts`:

```text
failure: UnresolvedName `x` after local imports from `./shared/abc` and `./shared2/abc`
source: `/shared/abc.ts` has `@symlink: /src/shared/abc.ts,/src/shared2/abc.ts`
owner: issue 5426 for registering symlink aliases as virtual module paths
later dump boundary: virtual `/src/tsconfig.json` JSON parsing, owned by issue 5292
```

## Completion evidence

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks --detail --no-dashboard-data
result: pass; prefix reports mixed UnsupportedSyntax/UnsupportedModule blockers across five symlink files
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_notInNodeModules.ts
result: pass; current smart failure is unresolved `x` with symlink aliases for `./shared/abc` and `./shared2/abc`, split to issue 5426; dumps also show later tsconfig parsing owned by 5292
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks.ts
result: pass; current failure is dependency export-class issue-5005, superseded by issue 5324
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_withOutDir.ts
result: pass; current failure is dependency export-class issue-5005, superseded by issue 5324
date: 2026-05-08
```

Remaining risks:

- After issues 5324, 5426, and 5292 land, these references may expose
  symlink realpath semantics, package traversal through symlinked node_modules,
  duplicate/private class diagnostics, or outDir emit behavior.

## Close note

Superseded by focused module-graph and multi-section issues.

superseded-by: 5324
superseded-by: 5426
superseded-by: 5292
