---
id: 1162
title: "Implement Circularreferenceinimport"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5229]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage circularReferenceInImport across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `circularReferenceInImport` with diagnostics: import-export. Fresh triage shows the current runner view parses the namespace, export assignment, namespace import, and exported function, then stops in module graph resolution for a virtual `// @filename:` sibling file.

Problem: `circularReferenceInImport` is not a standalone implementation order in the current runner view; the first blocker is resolving imports between TypeScript reference `// @filename:` sections, now covered by issue 5229.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularReferenceInImport.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInImport.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/open/5229-resolve-imports-between-filename-sections.md` for the current virtual-file module graph blocker. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5229's virtual `@filename` module resolution work
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInImport.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInImport.ts
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

- [x] folded into `issues/open/5229-resolve-imports-between-filename-sections.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularReferenceInImport.ts`

## Duplicate detection

- `issues/open/5229-resolve-imports-between-filename-sections.md` owns resolving local imports between TypeScript reference `// @Filename:` / `// @filename:` virtual sections.
- `issues/done/232-resolve-local-relative-es-module-graph.md` owns real on-disk local relative module graph diagnostics, but not virtual `@filename` section registration.
- `issues/open/432-implement-import-export.md` is a broad import/export triage bucket and is not an implementation-ready owner.

## Smart triage

Fresh triage shows this generated import-export bucket has advanced past
frontend parsing. The first blocker is module graph resolution for virtual
`@filename` sections.

### Smart triage: circularReferenceInImport

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `module-resolution`
- Current compiler message: `issue-232: missing local module ./db`
- Path: `reference/typescript/tests/cases/compiler/circularReferenceInImport.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInImport.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInImport.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Source context:

```ts
// @module: commonjs
// @target: es2015
// @declaration: true

// @filename: db.d.ts
declare namespace Db {
    export import Types = Db;
}

export = Db;

// @filename: app.ts
import * as Db from "./db"

export function foo() {
    return new Object()
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; namespace Db, export assignment expression Db, ImportNamespace("./db"), ExportDecl function foo
module_graph: issue-232 missing local module `./db` imported from .../circularReferenceInImport.ts; tried ./db.ts, ./db.js, ./db.d.ts, ./db.tsx, ./db.mjs, ./db.cjs
coverage: UnsupportedModule/import-export
```

The smart-triage header also reports an internal `UnresolvedName` for `Db`
inside `export import Types = Db`, but coverage and the resolved dump show the
first build blocker is the module graph's missing virtual `./db` module.

TypeScript oracle evidence:

```text
TS2309: An export assignment cannot be used in a module with other exported elements.
TS2307: Cannot find module './db' or its corresponding type declarations.
```

The TypeScript AST for the raw source includes:

```text
ModuleDeclaration: declare namespace Db { ... }
ExportAssignment: export = Db
ImportDeclaration: import * as Db from "./db"
FunctionDeclaration: export function foo() { ... }
```

Resolution:

```text
Issue 5229 owns registering TypeScript reference `@filename` sections as virtual module paths and resolving local imports between those virtual sections. The current reference-triage failure is that exact module materialization boundary, not a standalone circular import implementation slice.
```

## Completion evidence

Fill only when moving to `done/`.

The `circularReferenceInImport` triage bucket is complete. The current failure is superseded by issue 5229's virtual `@filename` module resolution work.

Commits:

- superseded by `issues/open/5229-resolve-imports-between-filename-sections.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInImport.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedModule/import-export
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInImport.ts
result: pass; module_graph reports issue-232 missing virtual ./db module and bucket folded into issue 5229
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5229 may expose later `export = Db`, namespace alias, or circular import semantic-parity blockers in this reference case.
