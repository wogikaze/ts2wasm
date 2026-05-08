---
id: 5229
title: "Resolve imports between @Filename sections"
type: feature
area: compiler/module-graph
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Resolve local imports between virtual files declared by TypeScript reference
`// @Filename:` / `// @filename:` sections.

## Problem

`checkJsdocTypeTagOnExportAssignment2.ts` contains virtual files `a.ts`,
`b.js`, and `c.js`. The `c.js` section imports `./b`, but module graph
resolution looks on disk for `b.js` instead of resolving the virtual section.
`circularReferenceInImport.ts` has the same first blocker with virtual
`db.d.ts` and `app.ts` sections.

Problem: `import b from "./b"` in a `// @Filename: c.js` section reports `issue-232: missing local module ./b`.
Problem: `import * as Db from "./db"` in a `// @filename: app.ts` section reports `issue-232: missing local module ./db`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts
```

Source shape:

```ts
// @Filename: b.js
/** @type {import("./a").Foo} */
export default { c: false };

// @Filename: c.js
import b from "./b";
b;
```

Compiler evidence:

```text
tokens: ok
ast: ok; ExportDefault, ImportDefault("./b"), Expr Ident("b")
module_graph: issue-232 missing local module ./b; tried on-disk b.ts/b.js/etc.
TypeScript oracle: TS2307 for ./b in this reference window
```

## Desired final state

For reference-style multi-section input, the module graph can resolve a local
specifier to another `@Filename` section in the same source file.

## Scope

In scope:

- [ ] Register `@Filename` / `@filename` section names as virtual module paths.
- [ ] Resolve `./b` from a `c.js` section to the sibling `b.js` section.
- [ ] Resolve `./db` from an `app.ts` section to the sibling `db.d.ts` section.
- [ ] Resolve re-export source specifiers such as `export * from "./file"` to sibling virtual sections.
- [ ] Add one focused multi-section fixture using `export default` and a default import.

Out of scope:

- Package resolution and non-local module specifiers.
- Full TypeScript JSDoc type checking for `@type {import("./a").Foo}`.
- CommonJS `module.exports` lowering.
- All eight `checkJsdocTypeTagOnExportAssignment` variants.

## Affected paths

Expected:

- `crates/compiler/src/module_graph.rs`
- `crates/compiler/src/lib.rs`
- focused fixtures or compiler tests

Do not touch:

- `crates/backend-wasm/`
- unrelated frontend parser syntax

## Acceptance criteria

- [ ] `checkJsdocTypeTagOnExportAssignment2.ts` no longer reports `issue-232: missing local module ./b`.
- [ ] `circularReferenceInImport.ts` no longer reports `issue-232: missing local module ./db`.
- [ ] `moduleAugmentationDoesInterfaceMergeOfReexport.ts` no longer reports `issue-232: missing local module ./file` for `export * from "./file"`.
- [ ] `nounusedTypeParameterConstraint.ts` no longer reports
  `issue-232: missing local module ./bar` for `import { IEventSourcedEntity }
  from "./bar"` between `//@filename: bar.ts` and `//@filename: test.ts`
  sections.
- [ ] A focused compiler test proves `// @Filename: b.js` is resolved by `import "./b"` from another section.
- [ ] Existing missing real local module diagnostics still report issue-232.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(module) or test(filename)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/open/1138-implement-checkJsdocTypeTagOnExportAssignment.md`.
Also owns the matching first blocker folded from `issues/open/1162-implement-circularReferenceInImport.md`; see that closed bucket for full `./db` triage evidence.
Also owns `issues/open/3317-implement-moduleAugmentationExtendFileModule.md`: both `moduleAugmentationExtendFileModule1.ts` and `moduleAugmentationExtendFileModule2.ts` currently parse the `@filename` sections and then report issue-232 missing local module `./observable`, with the same sibling virtual-section resolution requirement as the side-effect `./map` import.
Also owns `issues/open/3323-implement-moduleAugmentationNoNewNames.md`: `moduleAugmentationNoNewNames.ts` parses virtual `map.ts`, `observable.ts`, and `main.ts` sections, then reports issue-232 missing local module `./observable` instead of resolving the sibling virtual section.
Also owns `issues/open/3308-implement-moduleAugmentationCollidingNamesInAugmentation.md`: `moduleAugmentationCollidingNamesInAugmentation1.ts` parses virtual `map1.ts`, `map2.ts`, `observable.ts`, and `main.ts` sections, then reports issue-232 missing local module `./observable` before the later duplicate-declaration diagnostics.
Also owns `issues/open/3309-implement-moduleAugmentationDeclarationEmit.md`: both `moduleAugmentationDeclarationEmit1.ts` and `moduleAugmentationDeclarationEmit2.ts` parse virtual `map.ts`, `observable.ts`, and `main.ts` sections, then report issue-232 missing local module `./observable` before declaration emit or merged-declaration diagnostics.
Also owns `issues/open/3311-implement-moduleAugmentationDoesInterfaceMergeOfReexport.md`: `moduleAugmentationDoesInterfaceMergeOfReexport.ts` parses virtual `file.ts`, `reexport.ts`, and `augment.ts` sections, then reports issue-232 missing local module `./file` for `export * from "./file"` before interface-merge diagnostics.
Also owns `issues/open/3312-implement-moduleAugmentationDoesNamespaceEnumMergeOfReexport.md`: `moduleAugmentationDoesNamespaceEnumMergeOfReexport.ts` parses virtual `file.ts`, `reexport.ts`, and `augment.ts` sections, then reports issue-232 missing local module `./file` for `export * from "./file"` before namespace/enum merge diagnostics.
Also owns `issues/open/3313-implement-moduleAugmentationDoesNamespaceMergeOfReexport.md`: `moduleAugmentationDoesNamespaceMergeOfReexport.ts` parses virtual `file.ts`, `reexport.ts`, and `augment.ts` sections, then reports issue-232 missing local module `./file` for `export * from "./file"` before namespace merge diagnostics.
Also owns `issues/open/3332-implement-moduleDeclarationExportStarShadowingGlobalIsNameable.md`: fresh triage for `moduleDeclarationExportStarShadowingGlobalIsNameable.ts` stops at issue-232 missing local module `./account` for `export * from "./account"` in virtual `model/index.ts` before ambient `declare global`, `./model`, or declaration nameability diagnostics become actionable.
Also owns `issues/open/3347-implement-moduleMemberMissingErrorIsRelative.md`: fresh triage for `moduleMemberMissingErrorIsRelative.ts` parses virtual `folder/foo.ts` and `folder/bar.ts`, then reports issue-232 missing local module `./foo` before the intended missing exported member diagnostic for `nosuch` becomes reachable.
Also owns part of `issues/done/3370-implement-moduleResolution-import-export.md`: fresh triage for `moduleResolution_relativeImportJsFile.ts` and `moduleResolution_relativeImportJsFile_noImplicitAny.ts` parses virtual `/src/b.js` plus `/src/a.ts`, then reports issue-232 missing local module `./b`; `moduleResolution_explicitNodeModulesImport.ts` and `_implicitAny.ts` parse virtual `/node_modules/foo/index.js` plus `/src/index.ts`, then report issue-232 missing local module `../node_modules/foo` before module-resolution diagnostics become actionable.
Also owns `issues/open/3377-implement-moduleResolutionNoTsCJS.md`: fresh
triage for `moduleResolutionNoTsCJS.ts` parses virtual `x.ts`, `y.tsx`,
`z.d.ts`, and `user.ts` sections, then reports issue-232 missing local module
`./x.ts` before CommonJS explicit-TypeScript-extension diagnostics become
actionable.
Also owns `issues/open/3378-implement-moduleResolutionNoTsESM.md`: fresh
triage for `moduleResolutionNoTsESM.ts` reaches the same virtual `x.ts`,
`y.tsx`, `z.d.ts`, and `user.ts` section layout, then reports issue-232
missing local module `./x.ts` before ESM explicit-TypeScript-extension
diagnostics become actionable.
Also owns `issues/open/3431-implement-namespaceMergedWithImportAliasNoCrash.md`: fresh
coverage reaches issue-232 missing local module `./file1` for
`import * as Lib from "./file1"` between `file2.ts` and sibling virtual
`file1.ts`. Later oracle diagnostics include namespace-as-value and missing
namespace member checks.
Also owns the local-virtual-section subset of
`issues/done/3380-implement-moduleResolutionWithExtensions-import-export.md`:
fresh triage for `moduleResolutionWithExtensions_notSupported*.ts` parses the
entry imports and then reports issue-232 missing local module `./tsx` or
`./jsx` instead of resolving the sibling empty virtual `.tsx` / `.jsx` sections.
Also owns `issues/open/3389-implement-moduleSameValueDuplicateExportedBindings.md`:
fresh triage for both `moduleSameValueDuplicateExportedBindings1.ts` and
`moduleSameValueDuplicateExportedBindings2.ts` stops at issue-232 missing local
module `./b` for `export * from "./b"` in virtual `a.ts` before duplicate
exported-binding semantics become actionable.
Also owns `issues/open/3608-implement-nounusedTypeParameterConstraint.md`:
fresh triage for `nounusedTypeParameterConstraint.ts` parses virtual
`bar.ts` and `test.ts` sections, then reports issue-232 missing local module
`./bar` for `import { IEventSourcedEntity } from "./bar"` before no-unused
type-parameter or type-only import/export diagnostics become actionable.

## Completion evidence

Fill when implemented.
