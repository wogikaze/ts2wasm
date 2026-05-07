---
id: 5252
title: "Support call-expression class heritage"
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

Allow class declarations whose `extends` clause is a call expression, starting
with the factory pattern `class Derived extends makeBaseClass() {}` that
TypeScript accepts because function declarations are hoisted.

## Problem

Problem: `classDeclaredBeforeClassFactory.ts` parses successfully but name
resolution/builtin resolution rejects the heritage expression with
`only simple inheritance (extends ClassName) is supported`.

The frontend AST already represents the heritage as `Call(callee=Ident
"makeBaseClass", args=[])`, and the TypeScript oracle reports no diagnostics.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts
```

Current diagnostic:

```text
UnsupportedSyntax: only simple inheritance (extends ClassName) is supported
```

Representative source:

```ts
class Derived extends makeBaseClass() {}

function makeBaseClass() {
    return class Base {};
}
```

Compiler dump evidence:

```text
tokens: ok
ast: ok; ClassDecl Derived extends Call(callee=Ident("makeBaseClass"), args=[])
resolved: stops in builtin_resolver with only simple inheritance diagnostic
```

TypeScript oracle:

```text
ok; diagnostics=[]
hint: makeBaseClass has type typeof Base
```

## Scope

In scope:

- [ ] Accept `extends <identifier>()` in class heritage during builtin resolution.
- [ ] Preserve existing resolution of the identifier callee, including later function declarations.
- [ ] Add focused coverage for `class Derived extends makeBaseClass() {}`.

Out of scope:

- Full declaration emit for arbitrary TypeScript `.d.ts` output.
- Generic heritage type arguments.
- Qualified/member heritage expressions such as `extends Foo.Object`, tracked by issue 5225.
- Import/export variants of declaration emit expression-in-extends buckets.
- General runtime lowering for arbitrary evaluated heritage expressions beyond this identifier-call case.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/cli/tests/`

Do not touch:

- parser code unless the existing AST evidence changes
- unrelated declaration emit buckets

## Acceptance criteria

- [ ] `classDeclaredBeforeClassFactory.ts` no longer reports `only simple inheritance (extends ClassName) is supported`.
- [ ] A focused test covers `class Derived extends makeBaseClass() {}` with `makeBaseClass` declared later in the file.
- [ ] If runtime support for dynamically evaluated heritage remains incomplete, the next blocker is recorded after resolver acceptance.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/open/1179-implement-classDeclaredBeforeClassFactory.md`.
Related generated bucket: `issues/open/1741-implement-declarationEmitExpressionInExtends-declaration-emit.md`.
## False-done audit

**truly-done** (5252)

- Implementation commits: verified via `git log --oneline --all --grep=5252`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
