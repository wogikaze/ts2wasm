---
id: 5398
title: "Resolve namespace import-equals alias value access"
type: feature
area: frontend/name-resolution
class: implementation-ready
priority: P1
depends_on: [5287]
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Resolve TypeScript namespace import-equals aliases in value positions so an
alias such as `import beez = foo.bar` can be used by qualified member access
like `beez.baz.boo`.

## Problem

Problem: `aliasErrors.ts` currently fails in `resolve_names` with
`UnresolvedName: unresolved name: beez` at `beez.baz.boo`, even though
`beez` is declared as `import beez = foo.bar`.

Fresh triage on 2026-05-08 shows the older namespace syntax boundary is gone.
The parser tokenizes the namespace declarations and import-equals aliases, but
AST construction erases the alias declarations to `Expr Undefined`, so the
resolver has no binding for `beez` when it reaches the function body.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasErrors.ts
```

Equivalent repo task:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasErrors.ts
```

Representative source:

```ts
namespace foo {
    export class Provide {}
    export namespace bar { export namespace baz { export class boo {} } }
}

import beez = foo.bar;

function use() {
    beez.baz.boo;
}
```

Concrete current failure:

```text
coverage: unsupported=1, unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1
triage: UnresolvedName unresolved name: `beez` at 404..408
oracle: no diagnostic for `beez.baz.boo`; later diagnostics include invalid aliases and TS2694 for `booz.bar`
```

Compiler evidence:

```text
tokens: ok through namespace foo, nested exported namespace/class declarations, and `import beez = foo.bar`
ast: ok but import-equals aliases are erased to `Expr Undefined`
resolved: fails in resolve_names at `beez` inside `beez.baz.boo`
visible symbols before failure: classes `Provide` and `boo`, binding `p`, function `use`, binding `p1`
```

## Desired final state

`aliasErrors.ts` no longer reports `UnresolvedName` for `beez` at
`beez.baz.boo`. The resolver either resolves the namespace import-equals alias
for qualified value access or advances to a narrower namespace lowering/runtime
diagnostic.

## Scope

In scope:

- [ ] Preserve `import beez = foo.bar` as a resolver-visible namespace alias.
- [ ] Resolve `beez.baz.boo` far enough that the top-level alias name `beez`
  no longer reports `UnresolvedName`.
- [ ] Keep invalid namespace aliases such as `import m = no` and primitive
  alias targets source-spanned for later diagnostics.

Out of scope:

- Same-file namespace declaration value binding itself, tracked by issue 5287.
- Missing namespace alias member diagnostics for type positions, tracked by issue 5397.
- Full runtime namespace object emission.
- External `require(...)` import-equals module loading.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused namespace/name-resolution fixtures or tests

Do not touch:

- static ES module loading or package resolution
- backend/runtime code unless this slice exposes a reviewed namespace value representation

## Acceptance criteria

- [ ] `aliasErrors.ts` no longer reports `UnresolvedName: unresolved name: beez` at `beez.baz.boo`.
- [ ] A focused fixture covers `namespace foo { export namespace bar { export namespace baz { export class boo {} } } } import beez = foo.bar; beez.baz.boo;`.
- [ ] The next `aliasErrors.ts` blocker, if any, is recorded in this issue or split to a follow-up if outside namespace alias value access.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(import) or test(alias) or test(name)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasErrors.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/aliasErrors.ts --detail --no-dashboard-data
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

Split from generated bucket `580` on 2026-05-08. This issue depends on 5287
because namespace aliases need the target namespace declarations to be
resolver-visible values before alias value access can be resolved reliably.

After `beez` resolves, the same reference is expected to expose invalid alias
diagnostics for `no`, numeric/string/null/undefined aliases, or the TS2694
`booz.bar` diagnostic already represented by issue 5397.

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
