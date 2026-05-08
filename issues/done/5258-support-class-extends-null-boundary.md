---
id: 5258
title: "Report super calls in class extends null constructors"
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

Recognize `extends null` far enough to report invalid constructor `super()`
calls with a source-spanned diagnostic instead of failing the generic
`only simple inheritance (extends ClassName) is supported` gate.

## Problem

Problem: `classExtendsNull.ts` parses `extends null` and a constructor
`super()` call, but builtin resolution rejects the `Null` heritage expression
before the more specific constructor diagnostic can be emitted.

Representative:

```ts
class C extends null {
    constructor() {
        super();
        return Object.create(null);
    }
}
```

Current diagnostic:

```text
UnsupportedSyntax: only simple inheritance (extends ClassName) is supported
```

TypeScript accepts the heritage syntax and reports `TS17005` for `super()` in a
constructor whose class extends `null`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsNull.ts
```

## Scope

In scope:

- [x] Treat `extends null` as a distinct heritage kind for constructor
  `super()` diagnostics.
- [x] Emit a source-spanned diagnostic for `super()` inside a constructor whose
  class extends `null`.
- [x] Preserve `extends Base` behavior.

Out of scope:

- Full runtime implementation of `extends null` object construction.
- `super.x` diagnostics in `extends null` classes, tracked by
  `issues/done/5259-report-super-property-access-in-class-extends-null.md`.
- Interface-merge/static-side diagnostics for `classExtendsNull2.ts`.
- Broad arbitrary heritage expression support.
- Object global builtin support if later exposed by `Object.create(null)`.

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

- [x] `classExtendsNull.ts` no longer reports the generic
  `only simple inheritance (extends ClassName) is supported` diagnostic.
- [x] A focused fixture covers `class C extends null { constructor() { super(); } }`.
- [x] The next diagnostic is source-spanned at the invalid constructor
  `super()` use or the unsupported `extends null` semantic boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-cli -E 'test(class) | test(super)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsNull.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsNull.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1205-implement-classExtendsNull.md`.
Sibling split: `issues/done/5259-report-super-property-access-in-class-extends-null.md`.
## False-done audit

**truly-done** (5258)

- Implementation commits: verified via `git log --oneline --all --grep=5258`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
