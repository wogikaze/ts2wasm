---
id: 5239
title: "Bind nested class declarations in function scopes"
type: bug
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Resolve local class declarations that appear inside function bodies when later
expressions in the same function reference the class binding.

## Problem

The parser and triage symbol dump see the nested `PrismaClient` class in
`circularConstructorWithReturn.ts`, but name resolution still reports
`UnresolvedName` for `return PrismaClient`.

Problem: a nested class declaration inside a function body is not bound as a local value/type name for later statements in that function scope, blocking `return PrismaClient` before the existing class-constructor-value issues can run.

## Current failure

Reference reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] unsupported: unresolved name: PrismaClient at 474..486
```

Source context:

```ts
export function getPrismaClient(options?: any) {
  class PrismaClient {
    self: Client;
    constructor(options?: any) {
      return (this.self = applyModelsAndClientExtensions(this));
    }
  }

  return PrismaClient
}
```

Triage evidence:

```text
tokens: ok
ast: ok; ExportDecl function getPrismaClient contains nested ClassDecl PrismaClient followed by Return Ident("PrismaClient")
visible symbols: function getPrismaClient and nested class PrismaClient are listed before failure
resolved: fails in resolve_names with UnresolvedName PrismaClient at 474..486
TypeScript oracle: ok, no diagnostics; getPrismaClient has type typeof PrismaClient
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

## Desired final state

Name resolution binds function-local class declarations so references after the
declaration resolve to the class symbol. If later lowering rejects the class
constructor value as unsupported, that diagnostic is source-spanned and tracked
by the class-value issues instead of surfacing as raw `UnresolvedName`.

## Scope

In scope:

- [ ] Bind nested `ClassDecl` names in function/block scopes before resolving later statements in the same scope
- [ ] Preserve existing class behavior for `new C()` and `C.staticMethod()`
- [ ] Add a focused resolver regression for a function-local class returned by name
- [ ] Keep out-of-scope or pre-declaration class references reporting `UnresolvedName`

Out of scope:

- First-class class constructor value support after the binding resolves; tracked by issue 5192
- Full class runtime/prototype semantics
- Constructor return-type semantic parity with TypeScript

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/`
- resolver-focused tests or fixtures under `fixtures/` / `crates/cli/tests/`

Do not touch:

- backend/runtime class-value implementation unless fresh triage proves this slice has advanced past name resolution

## Acceptance criteria

- [ ] A focused test proves `function f() { class C {} return C; }` no longer reports `UnresolvedName`
- [ ] The representative path `reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts` advances past `UnresolvedName PrismaClient`
- [ ] Existing class tests for `new C()` and `C.staticMethod()` continue to pass
- [ ] A negative test or existing fixture proves an out-of-scope class name still reports `UnresolvedName`
- [ ] If the representative path next reports `issue-5011`, update issue 5192 evidence instead of broadening this issue

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(name_resolver) or test(class)'
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none expected; update issue 5192 only if this fix exposes class constructor value rejection

## Notes

Issue 5192 is related but later in the pipeline: it covers class constructor
bindings that already resolve and then fail as values with `issue-5011`. This
issue is narrower because `PrismaClient` currently fails during name resolution
even though the nested class declaration is visible in the triage output.

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
