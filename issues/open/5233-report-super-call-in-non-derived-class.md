---
id: 5233
title: "Report super call in non-derived class"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Convert `super()` inside a class with no `extends` clause from an unsupported
compiler boundary into a source-spanned TypeScript-compatible semantic
diagnostic.

## Problem

`checkSuperCallBeforeThisAccessing9.ts` parses three classes successfully, but
lowering stops at `Derived2.constructor()` because it treats `super()` in a
non-derived class as unsupported syntax.

Problem: `super()` in a class without `extends` reports `UnsupportedSyntax: super(...) used in class without extends` instead of a TS2335-style diagnostic.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts
```

Source context:

```ts
// @filename: noSuperInJSDocExtends.js
class Based { }
/** @extends {Based} */
class Derived2 {
    constructor() {
        super();
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; Derived2 has no extends and constructor body contains Call(callee=Ident("super"))
resolved/lowered: UnsupportedSyntax: super(...) used in class without extends
TypeScript oracle: TS2335 "'super' can only be referenced in a derived class."
```

## Desired final state

The compiler reports a source-spanned semantic/type-check diagnostic matching
the TypeScript oracle for `super()` in non-derived classes, and the case no
longer counts as an unsupported syntax bucket.

## Scope

In scope:

- [ ] Preserve the `super()` call span through the non-derived-class diagnostic path.
- [ ] Emit `DiagCode::TypeScriptTypeCheck` or the nearest established semantic diagnostic for `super()` in class constructors without `extends`.
- [ ] Add a focused lowering/CLI diagnostic test for `class C { constructor() { super(); } }`.
- [ ] Re-run the representative reference triage and confirm the current `UnsupportedSyntax` boundary is gone.

Out of scope:

- Treating JSDoc `@extends` as an actual class heritage clause.
- Runtime support for legal `super()` in derived classes.
- `this` before `super` flow analysis.
- `super.method(...)` and `super.x` diagnostics, unless the same helper can cover them without broadening the slice.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/cli/tests/ir_lowering.rs` or focused CLI diagnostic tests
- `fixtures/`

Do not touch:

- parser class syntax unless span evidence proves the call span is unavailable
- backend/runtime implementation

## Acceptance criteria

- [ ] `class C { constructor() { super(); } }` reports a source-spanned semantic diagnostic instead of `UnsupportedSyntax`.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts` no longer reports `super(...) used in class without extends` as `UnsupportedSyntax`.
- [ ] Legal `super()` in a derived constructor keeps existing behavior.
- [ ] Existing unsupported class-runtime subset diagnostics remain unchanged outside this slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli ir_lowering
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1142-implement-checkSuperCallBeforeThisAccessing-class.md`.

Related but broader:

- `issues/open/449-implement-super.md`
- `issues/done/047-implement-super-keyword.md`

## Completion evidence

Fill when implemented.
