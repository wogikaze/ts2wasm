---
id: 5192
title: "Support first-class class constructor values"
type: feature
area: ir/runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Allow class constructor bindings to be used as runtime values when they flow through
ordinary expressions, such as passing a class declaration to a generic factory
function that later constructs instances.

## Problem

The compiler now parses the representative TypeScript source and lowers class
declarations, but name resolution still rejects `MenuWorkbenchToolBar` when it is
used as a value argument. TypeScript accepts this pattern because class
declarations are both types and constructor values.

Problem: class constructor bindings used as expression values still fail with `issue-5011`, blocking reference cases that pass constructors to helper functions or cast/access class constructor values.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cachedContextualTypes.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-5011: class `MenuWorkbenchToolBar` cannot be used as a value - class runtime is not yet supported at 483..503
```

Source context:

```ts
class MenuWorkbenchToolBar {
    constructor(
        options: IMenuWorkbenchToolBarOptions | undefined,
    ) { }
}

createInstance(MenuWorkbenchToolBar, {
    toolbarOptions: {
        foo(bar) { return bar; }
    }
});
```

Parser evidence:

```text
ast: ok
top-level class: ClassDecl { name: "MenuWorkbenchToolBar", constructor: Some(...) }
failing expression: Call createInstance(MenuWorkbenchToolBar, { toolbarOptions: ... })
resolved: issue-5011 at identifier MenuWorkbenchToolBar
TypeScript oracle: ok, diagnostics: []
```

Additional generated bucket evidence:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castParentheses.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-5011: class `a` cannot be used as a value — class runtime is not yet supported at 73..74
```

Source context:

```ts
class a {
    static b: any;
}

var b = (<any>a);
var b = (<any>a).b;
var b = (<any>a.b).c;
var b = (<any>a.b()).c;
var b = (<any>new a);
var b = (<any>new a.b);
var b = (<any>new a).b
```

Parser evidence:

```text
ast: ok
top-level class: ClassDecl { name: "a" }
failing expression: Let b = Ident("a") from `(<any>a)`
resolved: issue-5011 at identifier `a`
TypeScript oracle: ok, diagnostics: []
```

## Desired final state

Class constructor bindings can be represented as first-class runtime values where
the class value is passed through expressions and invoked by supported constructor
paths. The representative `cachedContextualTypes.ts` reference case no longer
fails with `issue-5011`.

## Scope

In scope:

- [ ] Represent a class constructor binding as a value in lowered IR/runtime data
- [ ] Preserve existing direct `new C()` and method-call class behavior
- [ ] Support passing a class constructor value as an argument to a function
- [ ] Add a focused fixture covering `createInstance(C, args)` or an equivalent constructor factory

Out of scope:

- Dynamic static member access through arbitrary aliases, such as `let c = C; c.one()`
- Full TypeScript contextual typing
- Complete class semantic parity beyond constructor value flow

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `fixtures/`
- `crates/cli/tests/`

Do not touch:

- unrelated parser syntax unless new triage proves the failure regressed before name resolution

## Acceptance criteria

- [ ] `cachedContextualTypes.ts` no longer reports `issue-5011` for `MenuWorkbenchToolBar`
- [ ] `castParentheses.ts` no longer reports `issue-5011` for the first class constructor value use `(<any>a)`
- [ ] A focused fixture passes for a class constructor value passed to a helper and used to construct an instance
- [ ] Existing direct class fixtures, including `new C()` and static method calls, still pass
- [ ] Unsupported class-value cases that remain out of scope keep a source-spanned diagnostic

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cachedContextualTypes.ts
cargo nextest run -p ts2wasm-cli -E 'test(class)'
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

Issue 5011 deliberately replaced silent class-value erasure with a structural
diagnostic. This issue narrows the next semantic step: support constructor values
that are actually needed for factory-call flows, without reopening broad dynamic
class alias semantics.

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
