---
id: 400
title: "Implement ambient declaration erasure and rejection boundary"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: [399]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement the TypeScript ambient declaration boundary defined by issue 399 for the top `tsc` `ambient-declaration` bucket.

Problem: Generated ambient declaration issues are currently split across many one-case buckets, but the shared frontend behavior is the same boundary decision: erase declaration-only forms that cannot affect runtime, preserve module-shaped declarations for module ownership, and reject unsupported ambient forms with a precise TypeScript diagnostic.

## Problem

Problem: `tsc` coverage reports `ambient-declaration` as a top TypeScript-only unsupported bucket, and the compiler lacks one implementation-ready slice for the common ambient erasure/rejection behavior.

## Current failure

Representative generated issues with `feature_label: ambient-declaration` include:

- `issues/open/140-implement-ambientClassDeclarationWithExtends.md`
- `issues/open/142-implement-ambientClassMergesOverloadsWithInterface.md`
- `issues/open/144-implement-ambientConstLiterals.md`
- `issues/open/145-implement-ambientEnum.md`
- `issues/open/148-implement-ambientExportDefaultErrors.md`
- `issues/open/150-implement-ambientExternalModuleReopen.md`
- `issues/open/160-implement-ambientModules.md`
- `issues/open/162-implement-ambientPropertyDeclarationInJs.md`

Issue 399 records the coverage window with `tsc --limit 200` at `ambient-declaration:30` and the broader `tsc --limit 500` window at `ambient-declaration:29`.

## Desired final state

Ambient declarations are handled according to the TypeScript boundary contract:

- declaration-only ambient forms that introduce no executable binding are parsed and erased;
- ambient module declarations preserve enough shape to route to module ownership or produce `UnsupportedModule`;
- unsupported ambient forms produce source-spanned `UnsupportedTypeScriptSyntax`;
- no ambient declaration silently creates a runtime object, function, class, enum, or module binding.

## Scope

In scope:

- [ ] Add parser support for a narrow representative set of `declare` declarations used by the listed ambient coverage cases.
- [ ] Add an erasure path for declaration-only ambient forms with no runtime effect.
- [ ] Route ambient module declarations to `UnsupportedModule` when module shape is the blocker.
- [ ] Add source-spanned `UnsupportedTypeScriptSyntax` diagnostics for ambient forms outside the slice.
- [ ] Add parser/coverage regression tests for the representative cases selected from the generated issues above.

Out of scope:

- Full `.d.ts` emit support.
- Full TypeScript checker parity.
- Runtime class, enum, accessor, or module semantics not emitted by the ambient erasure boundary.
- JSX, decorators, and non-ambient TypeScript transforms.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `scripts/run/reference-coverage.py`
- `fixtures/`
- `issues/open/`
- `current-state.md`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] At least three representative `ambient-declaration` coverage cases are re-triaged from generated one-case issues into this boundary slice.
- [ ] Declaration-only ambient forms in the selected slice parse and erase before runtime lowering.
- [ ] Ambient module-shaped forms in the selected slice produce `UnsupportedModule` or are split into a module issue with evidence.
- [ ] Unsupported ambient forms produce source-spanned `UnsupportedTypeScriptSyntax`.
- [ ] Regression coverage proves no erased ambient form creates runtime bindings.
- [ ] Docs/current-state/issues are synchronized when status or design changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run update-issue-index
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 200
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/05-compatibility-and-semantics.md` if the boundary changes

Current state:

- [ ] updated: `current-state.md` (repo root) when implementation status changes

Follow-up issues:

- [ ] created/updated for ambient module or runtime forms split out of this issue

## Notes

This issue is the third concrete TypeScript-only child bucket created from issue 399, alongside issue 345 (`type-alias`) and issue 346 (`declaration-emit`).

Progress 2026-05-01:

- Added parser erasure for declaration-only ambient function declarations: top-level `declare function` and `export declare function`.
- Added parser erasure for declaration-only ambient variable declarations: `declare const`, `declare let`, `declare var`, and `export declare const`.
- Added regression coverage that erased ambient variable declarations leave only the following runtime `let value = 1;` binding in the AST.
- Added parser erasure for representative declaration-only ambient class declarations, ambient enum declarations, and class-element `declare` fields.
- Kept ambient variable/class-element initializers and runtime `enum` declarations on the `UnsupportedTypeScriptSyntax` boundary.
- Kept ambient module and namespace declarations on the `UnsupportedModule` boundary.

Representative generated buckets re-triaged into this boundary slice:

- `issues/open/140-implement-ambientClassDeclarationWithExtends.md`: `declare class A { }` / `declare class B extends A { }` is declaration-only and now parses/erases before runtime lowering.
- `issues/open/145-implement-ambientEnum.md`: `declare enum E1 { ... }` is declaration-only and now parses/erases before runtime lowering.
- `issues/open/150-implement-ambientExternalModuleReopen.md`: `declare module "fs" { ... }` is module-shaped and now routes to `UnsupportedModule` instead of a generic parser error.
- `issues/open/160-implement-ambientModules.md`: `declare namespace Foo.Bar { ... }` is module-shaped and now routes to `UnsupportedModule`.
- `issues/open/162-implement-ambientPropertyDeclarationInJs.md`: `declare prop: string;` inside a class is declaration-only and now parses/erases without adding a runtime class element.

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
