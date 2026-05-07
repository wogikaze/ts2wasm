---
id: 5357
title: "Avoid eval diagnostic for qualified Function constructors"
type: bug
area: ir/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Do not classify namespace-qualified `new M.Function(...)` as the global dynamic
`Function` constructor. Only an unqualified, unshadowed `Function(...)` or
`new Function(...)` should report the issue-062 eval boundary.

Split from generated bucket `1473`.

## Problem

`constructorOverloads4.ts` declares a namespace member `M.Function`, then calls
`new M.Function("return 5")`. TypeScript treats this as the namespace class
constructor and reports a later callability diagnostic, but ts2wasm reports
issue-062 as if it were the global dynamic `Function` constructor.

Problem: current failure is a false eval diagnostic for qualified `new M.Function(...)` because resolver `Expr::New` collapses member callees to their property name.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads4.ts
```

Current diagnostic:

```text
UnsupportedEval: issue-062: dynamic Function constructor is not supported; runtime code evaluation is intentionally not implemented at 244..270
```

Source context:

```ts
declare namespace M {
    export class Function {
        constructor(...args: string[]);
    }
}

(new M.Function("return 5"))();
M.Function("yo");
```

Compiler evidence:

```text
tokens/ast: ok; New callee is Member(Ident("M").Function)
resolved: issue-062 dynamic Function constructor diagnostic at new M.Function
oracle: TS2349 because the constructed M.Function value has no call signatures
```

## Desired final state

`new M.Function(...)` is resolved as a qualified member/class construction, or it
advances to a narrower namespace/class diagnostic. The global dynamic Function
constructor diagnostic remains reserved for unqualified `Function` calls and
constructors.

## Scope

In scope:

- [x] Keep issue-062 for unqualified global `Function(...)` and `new Function(...)`.
- [x] Stop treating `Expr::New { expr: Member(..., property: "Function") }` as global dynamic Function.
- [x] Add focused resolver coverage for `new M.Function("return 5")`.

Out of scope:

- Implementing dynamic eval or global Function semantics.
- Full namespace value/class construction lowering after this diagnostic advances.
- TypeScript TS2349 callability diagnostics.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`
- focused CLI/reference test if needed

Do not touch:

- backend/runtime eval implementation
- broad eval issue 429

## Acceptance criteria

- [x] `constructorOverloads4.ts` no longer reports issue-062 for `new M.Function("return 5")`.
- [x] A focused resolver test proves member callee `M.Function` is not treated as global `Function`.
- [x] Existing tests for unqualified `Function("return 1")` and `new Function("return 1")` still report issue-062.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(name_resolver) or test(eval) or test(Function)'
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads4.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads4.ts --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] record the next namespace/class construction blocker if exposed.

## Notes

The current resolver code extracts `property` from `Expr::Member` in `Expr::New`
and then compares the extracted callee name to `Function`.

## Completion evidence

Fill only when moving to `done/`.
