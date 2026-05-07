---
id: 3406
title: "Split multiModuleClodule to class namespace merge owner"
type: maintenance
area: frontend/semantics
class: superseded
priority: P1
depends_on: [5432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Closed the generated `multiModuleClodule` bucket by splitting the current blocker into focused child issue #5432.

## Problem

`reference/typescript/tests/cases/compiler/multiModuleClodule1.ts` now fails with `UnresolvedName` after the parser erases same-name `namespace C` declarations that should augment class `C`'s static side.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiModuleClodule1.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiModuleClodule1.ts
result: UnresolvedName for `C` after class `C` plus `namespace C` declarations
date: 2026-05-08
```

## Evidence

Source shape:

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
tokens: ok through class C, namespace C, exported var x, exported function foo
ast: class C retained; namespace C declarations erased; later executable code remains new C(C.x) and C.foo
resolved/lowered: UnresolvedName for `C`
typescript oracle: accepts the file with no diagnostics
```

Related existing issues are not exact owners:

- #5287 owns namespace root value binding for `M.x` / `new M.N.C()`, not a namespace merged into an existing class value.
- #5244 owns function/namespace static properties.
- #5329 owns duplicate diagnostics for class/namespace member collisions, not valid merged member lookup.

## Child Issues

- #5432: support class namespace merged static member lookup.

## Validation

Issue sync and health checks:

```text
python scripts/manager.py update-issue-index
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Focused reference checks:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiModuleClodule1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiModuleClodule1.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- #5432 remains open for implementation.
