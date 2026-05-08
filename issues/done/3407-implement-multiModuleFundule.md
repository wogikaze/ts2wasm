---
id: 3407
title: "Close multiModuleFundule to function namespace merge owner"
type: maintenance
area: frontend/semantics
class: superseded
priority: P1
depends_on: [5244]
blocks: []
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Closed the generated `multiModuleFundule` bucket as covered by implementation-ready issue #5244.

## Problem

`reference/typescript/tests/cases/compiler/multiModuleFundule1.ts` declares `function C` and same-name `namespace C` blocks. The namespace export `foo` should be visible as `C.foo`, but the compiler currently reports an unsupported function/static member boundary.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiModuleFundule1.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=import-export:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiModuleFundule1.ts
result: UnsupportedSyntax issue-211: unknown receiver class for method `foo` at C.foo()
date: 2026-05-08
```

## Evidence

Source:

```ts
function C(x: number) { }

namespace C {
    export var x = 1;
}
namespace C {
    export function foo() { }
}

var r = C(2);
var r2 = new C(2);
var r3 = C.foo();
```

Compiler evidence:

```text
tokens: ok through function C, namespace C, export var x, export function foo
ast: function C retained; namespace C declarations erased; later executable C.foo() remains
resolved/lowered: UnsupportedSyntax issue-211 unknown receiver class for method foo
typescript oracle: accepts the file with no diagnostics
```

## Owner

- #5244: support namespace-merged function static properties.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiModuleFundule1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiModuleFundule1.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- #5244 remains open for implementation.
