---
id: 5346
title: "Parse CommonJS export assignment statements"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse TypeScript CommonJS export assignment statements such as `export = x;`
instead of reporting the generic issue-055 static export boundary.

## Problem

`augmentExportEquals1.ts` currently stops in the first file section:

```ts
var x = 1;
export = x;
```

The lexer tokenizes the full multi-file reference input, but AST generation
fails at `export = x;` before reaching the later import or augmentation checks.

Current diagnostic:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 97..103
```

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentExportEquals1.ts
```

Source context:

```text
4 | // @filename: file1.ts
5 | var x = 1;
6 | export = x;
7 |
8 | // @filename: file2.ts
```

Evidence observed 2026-05-07: tokens include `Export`, `Equal`,
`Ident("x")`, and `Semicolon`; AST/resolved fail before AST with issue-055;
TypeScript parses an `ExportAssignment`.

## Desired final state

The front-end recognizes `export = expr;` as a TypeScript export assignment and
`augmentExportEquals1.ts` advances past the current generic static export
boundary to the next narrower module/import or augmentation diagnostic.

## Scope

In scope:

- [ ] Parse top-level `export = expr;` as a distinct TypeScript export assignment shape or equivalent frontend representation.
- [ ] Preserve the expression span for the exported value.
- [ ] Add focused frontend coverage for `var x = 1; export = x;`.
- [ ] Re-run `augmentExportEquals1.ts` and record the next diagnostic.

Out of scope:

- Runtime lowering for CommonJS exports.
- Full module graph loading.
- `import x = require("./file1")` support.
- `declare module "./file1"` augmentation semantics.
- The invalid `export =` plus other exported declarations rule, tracked by `issues/open/5306-report-export-assignment-with-other-exports.md`.
- Virtual `@Filename` section import resolution, tracked by `issues/open/5229-resolve-imports-between-filename-sections.md`.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tests.rs`
- focused frontend tests

Do not touch:

- IR/lowering unless required only to surface a controlled unsupported diagnostic after parsing succeeds
- backend emit unless the parser/resolver advance reaches a lowering-only blocker
- broad package/module resolution

## Acceptance criteria

- [ ] `export = x;` no longer reports `issue-055: unsupported static export`.
- [ ] `narrowedImports_assumeInitialized.ts` no longer reports issue-055 at
  `export = a;` in the `/a.d.ts` section.
- [ ] A focused test proves `var x = 1; export = x;` parses as a CommonJS export assignment.
- [ ] Existing unsupported ES module export forms still report issue-055 where they are not in this slice.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentExportEquals1.ts` advances past the current static export diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentExportEquals1.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from `issues/done/763-implement-augmentExportEquals.md`.

Related but not duplicate:

- `issues/open/5306-report-export-assignment-with-other-exports.md` covers the
  TypeScript diagnostic for `export =` combined with other exported elements.
- `issues/open/5229-resolve-imports-between-filename-sections.md` covers
  virtual multi-file import resolution after import syntax is parsed.
- `issues/open/432-implement-import-export.md` is the broad import/export
  umbrella and is too large to implement directly.
- Also owns `issues/done/3325-implement-moduleAugmentationWithNonExistentNamedImport.md`:
  fresh triage for `moduleAugmentationWithNonExistentNamedImport.ts` stops at
  `export = Foo;` before AST construction. Later blockers are
  `export as namespace` parsing (issue 5231) and virtual `./foo` import
  resolution (issue 5229).
- Also owns `issues/done/3314-implement-moduleAugmentationDuringSyntheticDefaultCheck.md`:
  fresh triage for `moduleAugmentationDuringSyntheticDefaultCheck.ts` stops at
  `export = moment;` in the virtual `node_modules/moment/index.d.ts` section
  before package resolution or ambient module augmentation diagnostics.
- Also owns `issues/done/3411-implement-multipleExportAssignments.md`: fresh
  triage for `multipleExportAssignments.ts` stops at the first
  `export = server;` with the same issue-055 static export boundary. After
  export-assignment parsing lands, this path may need a narrower duplicate
  `export =` diagnostic issue for TypeScript TS2300.
- Also owns `issues/done/3428-implement-namedImportNonExistentName.md`: fresh
  triage for `namedImportNonExistentName.ts` stops at the first
  `export = Foo;` at 85..91 with the same issue-055 static export boundary.
  Later oracle diagnostics include duplicate `export=`, invalid
  `export as namespace`, and missing virtual `./foo` and `./foo2` modules,
  covered by follow-up module-diagnostics and virtual import-resolution issues.
- Also owns `issues/done/3430-implement-namespaceMergedWithFunctionWithOverloadsUsage.md`:
  fresh triage stops at `export = Foo;` after parsing the ambient namespace and
  declare function overload signatures. Later surfaces include
  `import X = require("./file")`, virtual module resolution, and the `X(0)`
  overload usage.
- Also owns `issues/done/3451-implement-narrowedImports.md` for
  `narrowedImports_assumeInitialized.ts`: fresh triage on 2026-05-08 tokenizes
  `declare namespace a { export const x: number; }`, then stops at
  `export = a;` with `UnsupportedModule: issue-055: unsupported static export`.
  Later surfaces include `import a = require("./a")` and virtual filename
  section resolution.
- Also owns `issues/done/3512-implement-noCircularDefinitionOnExportOfPrivateInMergedNamespace.md`:
  fresh triage for `noCircularDefinitionOnExportOfPrivateInMergedNamespace.ts`
  tokenizes `const cat`, `class Foo`, `export = Foo`, and merged
  `declare namespace Foo { export { cat }; }`, then stops at the
  `export = Foo;` issue-055 static export boundary before the merged namespace
  export-of-private behavior becomes actionable.

## Completion evidence

Fill when implemented.
