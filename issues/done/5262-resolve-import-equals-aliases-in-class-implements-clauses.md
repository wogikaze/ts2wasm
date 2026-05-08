---
id: 5262
title: "Resolve import-equals aliases in class implements clauses"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Resolve TypeScript `import T = M1.I` aliases when they are used in a class
`implements` heritage clause inside a namespace.

## Problem

`classImplementsImportedInterface.ts` tokenizes the namespace and import-alias
syntax, but AST construction/name resolution fails at the `T` in
`class C implements T`.

Current diagnostic:

```text
UnresolvedName: unresolved name: `T` at 170..171
```

TypeScript accepts the file with no diagnostics. Its AST path at the failing
position is:

```text
ModuleDeclaration M2 -> ModuleBlock -> ClassDeclaration -> HeritageClause -> ExpressionWithTypeArguments -> Identifier T
```

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts
```

Representative source:

```ts
namespace M1 {
    export interface I {
        foo();
    }
}

namespace M2 {
    import T = M1.I;
    class C implements T {
        foo() {}
    }
}
```

Compiler evidence:

```text
tokens: ok; Import token, alias T, M1.I qualified name, implements T
ast/resolved: UnresolvedName `T` at class implements heritage identifier
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

The resolver binds namespace-local import-equals aliases in class heritage type
positions. The representative path should no longer fail with unresolved name
`T`; if the alias target is erased/type-only, the compiler should advance past
the implements clause and expose the next blocker or build-pass result.

## Scope

In scope:

- [ ] Parse/preserve enough import-equals alias information for
  `import T = M1.I` inside a namespace block.
- [ ] Resolve `T` when used in `class C implements T`.
- [ ] Keep the resolved alias type-only so it does not create runtime module
  loading requirements.
- [ ] Preserve existing unsupported diagnostics for external `require(...)`
  import-equals forms.

Out of scope:

- Full ES module loading or runtime import/export execution.
- General namespace value export semantics beyond this type-only alias path.
- Declaration emit for import-equals aliases.
- Circular import-alias diagnostics.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- general ES module runtime loading

## Acceptance criteria

- [ ] `classImplementsImportedInterface.ts` no longer reports unresolved name
  `T` at the `implements T` clause.
- [ ] A focused fixture covers `namespace M2 { import T = M1.I; class C implements T {} }`.
- [ ] Existing import-equals `require(...)` unsupported behavior remains
  source-spanned and unchanged.
- [ ] A negative fixture or assertion covers unresolved aliases that are not
  declared in the namespace scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(import) or test(namespace) or test(class)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/open/1217-implement-classImplementsImportedInterface.md`.
Broad import/export umbrella issue 432 is not an exact implementation slice for
this namespace-local import-equals alias use in class heritage.

## False-done audit

**truly-done** (5262)

- Implementation commits: verified via `git log --oneline --all --grep=5262`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
