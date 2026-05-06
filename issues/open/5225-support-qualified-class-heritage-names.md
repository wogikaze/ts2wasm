---
id: 5225
title: "Support qualified class heritage names"
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

Support class heritage clauses whose base class is a qualified name, such as
`class Bar extends Foo.Object`.

## Problem

`checkForObjectTooStrict.ts` tokenizes and parses `class Bar extends
Foo.Object`, and name resolution preserves the member-expression heritage.
`crates/ir/src/builtin_resolver.rs` then rejects the class because it currently
only accepts simple identifier heritage clauses.

Current diagnostic:

```text
UnsupportedSyntax: only simple inheritance (extends ClassName) is supported
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```ts
namespace Foo {
    export class Object {}
}

class Bar extends Foo.Object {
    constructor() { super(); }
}
```

Compiler evidence:

```text
tokens: ok; includes Class Bar, Extends, Ident Foo, Dot, Ident Object
ast: ok; ClassDecl Bar extends Member(Ident Foo, "Object")
resolved: stops in builtin_resolver with only simple inheritance diagnostic
TypeScript oracle: reports TS2725 for the exported class named Object, not a parser/lowering unsupported heritage diagnostic
```

## Desired final state

The compiler no longer rejects qualified class heritage expressions with the
generic simple-inheritance diagnostic. The representative path should either
resolve `Foo.Object` as a qualified class reference or report a more precise
source-spanned diagnostic for unsupported namespace-qualified heritage.

## Scope

In scope:

- [ ] Accept `Expr::Member(Ident namespace, property)` in class `extends` clauses.
- [ ] Preserve enough qualified-name text or symbol identity for class lowering.
- [ ] Add a focused fixture for `class Bar extends Foo.Object`.

Out of scope:

- Full namespace/module runtime semantics.
- Arbitrary computed heritage expressions.
- TypeScript's TS2725 diagnostic for classes named `Object`.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/builtin_resolved.rs`
- `crates/ir/src/lowered/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- parser heritage parsing unless the AST shape regresses
- unrelated class private/static feature validation

## Acceptance criteria

- [ ] `checkForObjectTooStrict.ts` no longer reports `only simple inheritance (extends ClassName) is supported`.
- [ ] A focused fixture covers `class Bar extends Foo.Object`.
- [ ] Existing `class Derived extends Base` fixtures still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) or test(inherit) or test(namespace)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts --detail
```

Not run:

- none

## Notes

Split from generated bucket `issues/done/1131-implement-checkForObjectTooStrict.md`.

The broader generated bucket `issues/open/1195-implement-classExtendingQualifiedName.md` is related, but this issue records the exact current blocker exposed by `checkForObjectTooStrict.ts`.

## Completion evidence

Fill when implemented.
