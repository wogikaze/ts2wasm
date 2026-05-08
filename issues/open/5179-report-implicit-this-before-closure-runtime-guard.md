---
id: 5179
title: "Report implicit this before closure runtime guard"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-08
---

## Summary

`blockScopedBindingCaptureThisInFunction.ts` parses successfully, but lowering reports the generic `issue-062e` nested-function `this` closure runtime guard before the TypeScript-style `TS2683` diagnostic for `this`.

## Problem

The representative TSC case is not blocked by syntax. The AST contains an outer arrow function returning a nested ordinary function, a `for (let someKey in {})` loop, `this.helloWorld()`, and an inner arrow that captures `someKey`. Name resolution/lowering then stops with `UnsupportedRuntimeSubset`, while TypeScript reports a source diagnostic at the `this` expression.

Problem: a TypeScript implicit-`this` diagnostic is hidden by the nested-function closure runtime-subset guard.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts
```

Current compiler diagnostic:

```text
UnsupportedRuntimeSubset: issue-062e: nested function `` closures with `this` or `arguments` are not supported in this slice
```

Representative source:

```ts
() => function () {
    for (let someKey in {}) {
        this.helloWorld();
        () => someKey;
    }
};
```

Compiler evidence:

- Token dump includes `This`, member `helloWorld`, the `for-in` binding `someKey`, and the nested arrow `() => someKey`.
- AST construction succeeds and preserves `This` inside `Call(Member(This, "helloWorld"))`.
- Resolved/lowered pipeline fails at the `issue-062e` runtime guard.
- Visible symbols before failure include binding `someKey` at line 5, column 10.

TypeScript oracle evidence:

```text
TS2683: 'this' implicitly has type 'any' because it does not have a type annotation.
```

The oracle reports the diagnostic at the `this` token on line 6.

## Desired final state

The frontend reports a source-spanned implicit-`this` diagnostic for this TSC pattern before the generic nested-function closure runtime guard is reached.

## Scope

In scope:

- [x] Detect `this` use in the representative nested ordinary function without an explicit `this` parameter/type context.
- [x] Report a source-spanned diagnostic at the `this` token before lowering emits `issue-062e`.
- [x] Preserve the existing `issue-062e` runtime guard for JavaScript closure cases that have no earlier TypeScript diagnostic.
- [x] Add focused coverage for `() => function () { this.helloWorld(); };`.

Out of scope:

- Full `this` type inference or binding semantics.
- Runtime support for nested function closures that capture `this` or `arguments`.
- Captured block-scoped loop variable runtime semantics after this diagnostic advances.
- Async/generator closure semantics.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/diagnostic.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/ir/src/lowered/resolver_extra.rs`
- reference triage diagnostic mapping only if classification needs refining

Do not touch:

- Closure object ABI or backend closure dispatch.
- Existing `issue-062e` diagnostics for non-TypeScript runtime cases.

## Acceptance criteria

- [x] A focused frontend/compiler test covers `() => function () { this.helloWorld(); };`.
- [x] The diagnostic is source-spanned at the `this` token.
- [x] The existing nested-function `this`/`arguments` runtime-subset guard remains available for runtime-only cases without an earlier implicit-`this` diagnostic.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts` no longer reports `UnsupportedRuntimeSubset: issue-062e` as the first blocker.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend this
cargo nextest run -p ts2wasm-ir closure
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket `1066` on 2026-05-06. Completed issue `062e` intentionally left broader nested-function `this`/`arguments` closure semantics out of scope; this issue is narrower because the TSC reference case should surface an earlier TypeScript diagnostic instead of requiring runtime closure support.

Also owns `issues/open/3521-implement-noCollisionThisExpressionInFunctionAndVarInGlobal.md`: fresh triage reaches `UnresolvedName: this` inside an arrow in `function x()`, while TypeScript reports TS2683 at that `this`.

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

- Later triage may expose loop-capture or closure runtime work after the implicit-`this` diagnostic is handled.


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/open/. Implementation commits confirmed.
