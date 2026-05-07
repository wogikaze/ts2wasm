---
id: 5236
title: "Support nested function rest parameters in closure lowering"
type: feature
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Allow nested ordinary function expressions with rest parameters to use the
existing lowered function signature machinery instead of stopping at the
`issue-062e` closure guard.

## Problem

`checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts` parses
`export function`, object literals, and the nested function expression
successfully, but lowering rejects:

```ts
const composed = function (...args) { }
```

Problem: nested function expressions with rest parameters report
`UnsupportedRuntimeSubset: issue-062e: nested function `` closure parameters
with defaults or rest are not supported in this slice`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts
```

Source context:

```ts
export function test(fn) {
  const composed = function (...args) { }

  Object.defineProperty(composed, 'name', {
    value: composed.fn + '_test'
  })

  return composed
}
```

Compiler evidence:

```text
tokens: ok; includes export function, nested Function, DotDotDot args, Object.defineProperty, and object literal descriptor
ast: ok; Let composed = FunctionExpr(params=[args is_rest=true], body=[])
resolved/lowered: UnsupportedRuntimeSubset issue-062e nested function closure parameters with defaults or rest
TypeScript oracle: TS2339 Property 'fn' does not exist on type '(...args: any[]) => void'
```

## Desired final state

Nested function expressions use the same rest parameter lowering already
available for ordinary lowered functions, including functions assigned to
locals and returned from their declaring function. The representative case
advances past the `issue-062e` rest closure guard.

## Scope

In scope:

- [ ] Remove or narrow the `lower_nested_function` guard for rest parameters when the lowered function signature can represent the case.
- [ ] Preserve hidden capture parameter ordering for nested functions with rest parameters, keeping capture parameters before the final rest parameter.
- [ ] Add a focused lowering regression for `function outer() { const f = function (...args) {}; return f; }`.
- [ ] Re-run the representative reference triage and confirm it no longer reports the rest-parameter `issue-062e` diagnostic.

Out of scope:

- Mutable captured environment support beyond the existing issue-062e boundary.
- Nested function default parameters; split a follow-up only after a representative default-parameter closure case is triaged.
- Nested functions using `this` or `arguments`, tracked separately by issue-062e follow-up slices such as `issues/open/5218-support-nested-function-closures-capturing-this.md`.
- Direct function-expression spread calls with rest parameters, still guarded by issue-274.
- Function object metadata such as `.name`, `.length`, or `.prototype`.
- TypeScript checker diagnostics such as TS2339 for `composed.fn`.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver_extra.rs`
- `crates/ir/src/lowered/program.rs`
- `crates/ir/src/lowered/validate.rs`
- focused lowering tests

Do not touch:

- frontend parser code unless a new fixture proves the AST is not preserving rest parameters correctly
- backend/runtime function metadata support

## Acceptance criteria

- [ ] A nested function expression with a rest parameter can be lowered as a closure/local function value without the rest-parameter `issue-062e` guard.
- [ ] Rest parameter index validation remains correct after hidden capture parameters are inserted.
- [ ] Existing diagnostics remain for nested functions that use `this`, `arguments`, or mutable captured outer locals.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts` advances past the current rest-parameter closure boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(closure) or test(function) or test(rest)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash1.ts --detail --no-dashboard-data
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

Split from generated bucket
`issues/done/1147-implement-checkingObjectDefinePropertyOnFunctionNonexistentPropertyNoCrash.md`.

Related but broader:

- `issues/open/445-implement-runtime-subset.md`
- `issues/done/062e-function-closures.md`
- `issues/done/212-implement-rest-parameter-argument-collection.md`
- `issues/done/040-implement-default-parameters.md`
- `issues/open/5218-support-nested-function-closures-capturing-this.md`

## Completion evidence

Fill when implemented.
