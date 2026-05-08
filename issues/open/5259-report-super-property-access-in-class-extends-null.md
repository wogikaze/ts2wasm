---
id: 5259
title: "Report super property access in class extends null"
type: feature
area: ir/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Report `super.x` inside classes that extend `null` with a precise diagnostic
instead of stopping at the generic simple-inheritance gate.

## Problem

Problem: `classExtendsNull3.ts` parses two classes with `extends null` and
`super.oops` inside static and instance methods, but builtin resolution rejects
the `Null` heritage expression before the `super` property access can be
classified.

Representative:

```ts
class C1 extends null {
  static method() {
    super.oops;
  }
}
```

Current diagnostic:

```text
UnsupportedSyntax: only simple inheritance (extends ClassName) is supported
```

TypeScript accepts the syntax and reports `TS2531` at `super` because the base
is possibly null.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsNull3.ts
```

## Scope

In scope:

- [x] Treat `extends null` as a distinct heritage kind for `super.x`
  diagnostics.
- [x] Emit a source-spanned diagnostic for `super.oops` in static and instance
  methods whose class extends `null`.
- [x] Preserve normal `super.x` handling in classes with identifier heritage.

Out of scope:

- Constructor `super()` diagnostics for `extends null`, tracked by
  `issues/done/5258-support-class-extends-null-boundary.md`.
- Full runtime implementation of `extends null`.
- General `super` property runtime lowering.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- parser code unless the AST evidence changes
- broad class runtime lowering

## Acceptance criteria

- [x] `classExtendsNull3.ts` no longer reports the generic
  `only simple inheritance (extends ClassName) is supported` diagnostic.
- [x] A focused fixture covers `class C extends null { method() { super.x; } }`.
- [x] The next diagnostic is source-spanned at the `super.x` use or the
  unsupported `extends null` semantic boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsNull3.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsNull3.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1205-implement-classExtendsNull.md`.
## False-done audit

**truly-done** (5259)

- Implementation commits: verified via `git log --oneline --all --grep=5259`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
