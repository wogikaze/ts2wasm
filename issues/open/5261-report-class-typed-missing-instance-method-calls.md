---
id: 5261
title: "Report class-typed missing instance method calls"
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

Report or classify method calls on class-typed ambient locals when the method is
not an instance method on the receiver class, instead of falling through to the
generic `issue-211` unknown receiver class diagnostic.

## Problem

`classImplementsClass6.ts` parses class declarations, `implements` clauses,
ambient value declarations, assignments, and method-call expressions. Lowering
then rejects `c.bar()` before it can expose the TypeScript diagnostic that
`bar` is not an instance member of `C`.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `bar` at 279..286
```

TypeScript reports:

```text
TS2339: Property 'bar' does not exist on type 'C'.
TS2576: Property 'bar' does not exist on type 'C2'. Did you mean to access the static member 'C2.bar' instead?
```

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsClass6.ts
```

Representative source:

```ts
class A {
    static bar(): string {
        return "";
    }
    foo(): number { return 1; }
}
class C implements A {
    foo() {
        return 1;
    }
}
class C2 extends A {}
declare var c: C;
declare var c2: C2;
c.bar();
c2.bar();
```

Compiler evidence:

```text
tokens: ok
ast: ok; ClassDecl A has static::bar and foo, ClassDecl C implements A has foo, ClassDecl C2 extends A, ambient c/c2, calls c.bar() and c2.bar()
resolved/lowered: issue-211 unknown receiver class for method `bar`
```

## Desired final state

The compiler no longer reports generic unknown receiver class for class-typed
ambient locals when the receiver type is known but the requested method is not
a supported instance method. The representative path should either produce a
source-spanned missing-instance-method diagnostic or advance to the next
semantic blocker.

## Scope

In scope:

- [ ] Preserve class type information for ambient value declarations such as
  `declare var c: C`.
- [ ] Resolve `c.bar()` / `c2.bar()` against the receiver class before the
  generic issue-211 unknown-receiver path.
- [ ] Distinguish static-only methods such as `A.bar` / inherited `C2.bar`
  from instance methods.
- [ ] Emit a source-spanned diagnostic at the property access when no supported
  instance method exists.

Out of scope:

- Full TypeScript assignability for `implements` clauses.
- Runtime support for arbitrary erased ambient class instances.
- Interface-typed erased receiver method calls covered by issue 5222.
- Builtin array/object/string receiver method support.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/` unless lowering already produces a supported runtime representation
- unrelated builtin method tables

## Acceptance criteria

- [ ] `classImplementsClass6.ts` no longer reports
  `issue-211: unknown receiver class for method bar`.
- [ ] A focused fixture covers `declare var c: C; c.bar();` where `bar` is not
  an instance method of `C`.
- [ ] A focused fixture covers an inherited static method accessed through an
  instance, e.g. `declare var c2: C2; c2.bar();`.
- [ ] Existing supported direct class instance method calls remain unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) or test(method) or test(name)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsClass6.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter classImplementsClass --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1216-implement-classImplementsClass.md`. The sibling
references `classImplementsClass1.ts`, `2.ts`, `3.ts`, `4.ts`, `5.ts`, and
`7.ts` are currently build-pass.
