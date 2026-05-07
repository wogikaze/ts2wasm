---
id: 5370
title: "Bind ambient namespace declarations for qualified value access"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Bind same-file ambient `declare namespace` declarations as resolver-visible namespace values so `app.foo.bar = ...` does not fail on the top-level namespace name.

## Problem

The parser consumes `declare namespace app { ... }` and keeps later qualified expressions, but name resolution cannot find `app`.

Problem: ambient namespace declarations are not visible as namespace values for same-file qualified value access.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE3.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE2.ts
```

Observed result:

```text
contextualReturnTypeOfIIFE3.ts: UnresolvedName: unresolved name: `app` at 173..176
contextualReturnTypeOfIIFE2.ts: UnresolvedName: unresolved name: `app` at 119..122
```

Compiler evidence:

```text
tokens/ast: ok through declare namespace and later app.foo.bar statements
resolved: UnresolvedName for app
TypeScript oracle: diagnostics=[]
```

## Desired final state

The resolver preserves enough ambient namespace metadata to resolve qualified accesses rooted at the namespace name, without emitting runtime declarations for the ambient namespace.

## Scope

In scope:

- [ ] Bind `declare namespace app { var foo: ... }` as a namespace value for same-file qualified accesses.
- [ ] Preserve erasure: ambient namespace declarations must not emit runtime declarations.
- [ ] Add focused resolver coverage and re-run both contextual IIFE references.

Out of scope:

- Non-ambient namespace value access, tracked by `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.
- Plain ambient `declare var` / `declare let` / `declare const` value references, tracked by `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`.
- Full namespace lowering, module ownership, runtime namespace object materialization, and contextual return type inference.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/ast.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`

Do not touch:

- static ES module loading, package resolution, or backend namespace emit

## Acceptance criteria

- [ ] `declare namespace app { var foo: { bar: number }; } app.foo.bar;` no longer reports `UnresolvedName` for `app`.
- [ ] `contextualReturnTypeOfIIFE3.ts` no longer reports `UnresolvedName: unresolved name: \`app\`` at `173..176`.
- [ ] `contextualReturnTypeOfIIFE2.ts` no longer reports `UnresolvedName: unresolved name: \`app\`` at `119..122`.
- [ ] Ambient namespace declarations remain erased and do not introduce runtime initialization.
- [ ] Any next blocker from the two reference files is recorded here or split to a follow-up if outside this scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(namespace) or test(resolve)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE3.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE2.ts
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

Split from generated bucket `issues/done/1495-implement-contextualReturnTypeOfIIFE-import-export.md` on 2026-05-07.

Related:

- `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md` covers non-ambient namespaces and explicitly excludes ambient `declare namespace`.
- `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md` covers erased ambient variables/lets/consts, not ambient namespace roots.
- Also owns `issues/done/3400-implement-moduledecl.md`: fresh triage for the
  broad module declaration reference stops at unresolved ambient namespace root
  `mAmbient` for qualified value accesses such as `mAmbient.foo()` and
  `new mAmbient.m3.C()`.

## Completion evidence

Fill only when implemented.
