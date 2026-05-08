---
id: 5389
title: "Support nested function default parameters in closure lowering"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Allow nested ordinary function expressions with optional/default parameters to
advance past the issue-062e closure guard, starting with `function self(a, b?)`
and `function self(a, b?: number)`.

## Problem

`contextuallyTypedParametersWithQuestionToken.ts` tokenizes and parses
successfully. The parser erases the question token into a parameter default of
`undefined`, but lowering rejects the nested `self` function expression before
the source can reach the TypeScript-style diagnostic for `acceptNum(b)`.

Current diagnostic:

```text
UnsupportedRuntimeSubset: issue-062e: nested function `self` closure parameters with defaults or rest are not supported in this slice
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```ts
function acceptNum(num: number) {}

const f1: (a: string, b: number) => void = function self(a, b?) {
  acceptNum(b);
  self("");
  self("", undefined);
};

const f2: (a: string, b: number) => void = function self(a, b?: number) {
  acceptNum(b);
  self("");
  self("", undefined);
};
```

Compiler evidence:

```text
tokens: ok; includes optional parameter question token on nested function
ast: ok; optional b? is represented as parameter default Undefined
resolved/lowered: UnsupportedRuntimeSubset issue-062e nested function closure
     parameters with defaults or rest
TypeScript oracle: TS2345 for acceptNum(b), where b is number | undefined
```

## Desired final state

The compiler no longer reports the generic issue-062e default/rest closure
guard for nested function expressions with optional/default parameters. The
representative path either lowers the nested functions far enough to build or
reports the earlier source-spanned TS2345-style diagnostic for passing
`number | undefined` to `acceptNum(number)`.

## Scope

In scope:

- [ ] Narrow the `lower_nested_function` guard for default parameters when the
  lowered function signature can represent the nested function.
- [ ] Support optional parameters represented as `Undefined` defaults in nested
  function expressions.
- [ ] Preserve hidden capture parameter ordering when default parameters are
  present.
- [ ] Add a focused lowering regression for
  `function outer() { const f = function self(a, b?) { return b; }; return f; }`.
- [ ] Re-run the representative reference triage and record the next diagnostic.

Out of scope:

- Rest parameters, tracked by
  `issues/open/5236-w1-implement-wasi-args-and-environment-variable-lowering.md`.
- Nested functions using `this` or `arguments`, tracked by
  `issues/open/5218-parse-typescript-this-parameters-in-function-expressions.md`.
- Mutable captured environment support beyond existing issue-062e boundaries.
- Full TypeScript contextual typing or TS2345 semantic diagnostics if lowering
  advances to a later semantic gap.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver_extra.rs`
- `crates/ir/src/lowered/program.rs`
- `crates/ir/src/lowered/validate.rs`
- focused lowering tests

Do not touch:

- frontend parser code unless a focused fixture proves optional/default
  parameter AST is not preserved
- backend/runtime ABI unless lowering already produces a valid function
  signature and the next blocker moves to emission/runtime

## Acceptance criteria

- [ ] `contextuallyTypedParametersWithQuestionToken.ts` no longer reports
  `issue-062e: nested function self closure parameters with defaults or rest`.
- [ ] A nested function expression with an optional/default parameter can be
  lowered as a closure/local function value without the default-parameter
  issue-062e guard.
- [ ] Existing issue-062e diagnostics remain for nested functions using rest
  parameters, `this`, `arguments`, or mutable captured outer locals.
- [ ] If the reference advances to TS2345-style diagnostic parity, that
  diagnostic is source-spanned at `acceptNum(b)`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(closure) or test(function) or test(default)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithQuestionToken.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithQuestionToken.ts --detail --no-dashboard-data
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
`issues/open/1547-implement-contextuallyTypedParametersWithQuestionToken.md`.

Related but distinct:

- `issues/open/5236-w1-implement-wasi-args-and-environment-variable-lowering.md` owns rest
  parameters and explicitly excludes nested function default parameters.
- `issues/done/062e-function-closures.md` records the historical closure work
  and broader issue-062e boundary.
- Also owns `issues/open/3401-implement-multiCallOverloads.md`: fresh triage
  for `multiCallOverloads.ts` gets through interface call signatures and stops
  at the same issue-062e guard for anonymous function expressions with optional
  parameters, such as `function(z?) {}`.

## Completion evidence

Fill when implemented.
