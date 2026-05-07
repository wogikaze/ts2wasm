---
id: 5312
title: "Parse export abstract class declarations"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Accept the TypeScript declaration form `export abstract class Name<...> ...`
far enough that it reaches the existing entry-module export-class boundary.

## Problem

Problem: `export abstract class ConvenientObservable<T, TChange> ...` in
`classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts` fails
before AST output with `UnsupportedModule: issue-055: unsupported static export`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts
```

Observed 2026-05-07:

```text
failure: UnsupportedModule issue-055: unsupported static export at 356..362
source:
19 | export abstract class ConvenientObservable<T, TChange> implements IObservable<T, TChange> {
20 |     get TChange(): TChange { return null!; }
21 |     public abstract get(): T;
compiler evidence: tokens ok; ast/resolved fail at the export keyword
TypeScript oracle: no diagnostics; AST topLevel includes exported abstract class declarations
```

## Desired final state

The frontend represents `export abstract class` as an exported abstract class
declaration, preserving type parameters, `implements`, and class members. It may
then advance to the existing module-build export-class blocker.

## Scope

In scope:

- [ ] Parse `export abstract class ConvenientObservable<T, TChange> ...`.
- [ ] Preserve the abstract modifier and declared class name in the AST.
- [ ] Add one focused parser/frontend regression for `export abstract class`.

Out of scope:

- Entry-module export-class build semantics; issue 5232 owns that boundary.
- Abstract member implementation checking.
- Import/export module loading.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- module graph loading or backend lowering

## Acceptance criteria

- [ ] The representative reference path no longer reports issue-055 at `export abstract class`.
- [ ] A focused regression shows AST output for `export abstract class A {}`.
- [ ] Existing `export class Foo {}` behavior is unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.ts
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

Split from generated bucket `issues/open/1193-implement-classExtendingAbstractClassWithMemberCalledTheSameAsItsOwnTypeParam.md`.

Related but not duplicates:

- `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md` starts after
  `ExportDecl(ClassDecl)` exists and handles module-build export metadata.
- `issues/open/1213-implement-classFunctionMerging-parser-syntax.md` covers
  `declare abstract class`, not `export abstract class`.

## Completion Evidence

Fill when implemented.
