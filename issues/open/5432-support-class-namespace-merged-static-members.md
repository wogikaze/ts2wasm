---
id: 5432
title: "Support class namespace merged static members"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Support the value side of TypeScript class/namespace merging so exported members from `namespace C { ... }` are visible as static members on class value `C`.

## Problem

`multiModuleClodule1.ts` declares `class C` and later same-name `namespace C` blocks that export `x` and `foo`. TypeScript accepts `new C(C.x)` and `C.foo`, but the compiler erases the namespace declarations and later reports unresolved `C`.

Problem: `multiModuleClodule1.ts` reports `UnresolvedName` for class/namespace merged value access because exported namespace members are not represented on the class static side.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiModuleClodule1.ts
```

Observed:

```text
UnresolvedName: unresolved name: `C`
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiModuleClodule1.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

## Source Context

```ts
class C {
    constructor(x: number) { }
    foo() { }
    static boo() { }
}
namespace C {
    export var x = 1;
}
namespace C {
    export function foo() { }
}
var c = new C(C.x);
c.foo = C.foo;
```

Compiler evidence:

```text
tokens: ok through class, namespace, export var, and export function
ast: class C retained; namespace C declarations erased; executable new C(C.x) and C.foo remain
typescript oracle: no diagnostics
```

## Desired final state

Same-file `namespace C` declarations merged with `class C` expose exported value members for static-side lookup. The representative case should advance past the current unresolved-name blocker or split a narrower runtime-lowering blocker.

## Scope

In scope:

- [ ] Preserve exported value members from same-name `namespace C` blocks for class `C` static-side lookup.
- [ ] Resolve `C.x` and `C.foo` in `new C(C.x)` / `c.foo = C.foo` with focused coverage for `class C {}; namespace C { export var x = 1; } C.x`.

Out of scope:

- Function/namespace merging; tracked by #5244.
- Namespace root-only qualified access such as `M.x`; tracked by #5287.
- Duplicate class/namespace member diagnostics; tracked by #5329.
- Full cross-file or ES module declaration merging.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused namespace/class merge tests or fixtures

Do not touch:

- static ES module resolution
- backend namespace emit unless a focused lowering test requires a reviewed shape

## Acceptance criteria

- [ ] `multiModuleClodule1.ts` no longer reports unresolved `C` for the class/namespace merged value access.
- [ ] A focused regression covers `class C {}; namespace C { export var x = 1; } C.x` while plain namespace and function/namespace blockers remain owned by #5287 and #5244.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(class)'
cargo nextest run -p ts2wasm-ir -E 'test(namespace) or test(class)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiModuleClodule1.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiModuleClodule1.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Notes

Split from #3406 on 2026-05-08. Related issues #5287, #5244, and #5329 cover adjacent namespace merge behavior but not this valid class static-side value lookup.
