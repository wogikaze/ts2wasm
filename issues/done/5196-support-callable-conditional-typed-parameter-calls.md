---
id: 5196
title: "Support callable conditional-typed parameter calls"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Classify or support calls to function parameters whose erased TypeScript type is
a callable conditional type, such as `arg: Q<T>` where `Q<T>` resolves to
`(n: number) => void` for the concrete branch under test.

## Problem

The parser and resolver preserve the function body shape for
`callOfConditionalTypeWithConcreteBranches.ts`, but lowering treats `arg(10)` as
an unsupported function-valued local call and emits the generic `issue-211`
diagnostic. TypeScript accepts this reference case with no diagnostics.

Problem: callable parameters typed through conditional type aliases currently fall into the generic `issue-211` function-valued local call boundary.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOfConditionalTypeWithConcreteBranches.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-211: function-valued local calls such as extracted method `arg(...)` are not supported; call receiver.method(...) directly at 177..184
```

Representative source:

```ts
type Q<T> = number extends T ? (n: number) => void : never;
function fn<T>(arg: Q<T>) {
  arg(10);
}
```

Triage evidence:

- Tokens and AST succeed.
- AST contains `Function fn` with parameter `arg` and body `Call(Ident arg, Number 10)`.
- TypeScript oracle reports no diagnostics and hints `arg: Q<T>`.

## Desired final state

The representative call no longer falls through to the generic `issue-211`
function-valued local diagnostic. The compiler either supports the callable
parameter path for the concrete function branch or reports a narrower
source-spanned unsupported conditional-call diagnostic.

## Scope

In scope:

- [ ] Preserve enough callable metadata for function parameters annotated with conditional function type aliases
- [ ] Classify `arg(10)` before the generic extracted-method issue-211 path
- [ ] Keep ordinary unsupported extracted method diagnostics unchanged

Out of scope:

- Full conditional type evaluation
- Full generic type inference for callable conditional aliases
- Runtime support for arbitrary higher-order function values

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/` unless lowering already has a supported callable representation
- unrelated receiver-method semantics

## Acceptance criteria

- [ ] `callOfConditionalTypeWithConcreteBranches.ts` no longer reports generic `issue-211` for the first `arg(10)` call
- [ ] A focused fixture covers `type Q<T> = number extends T ? (n: number) => void : never; function fn<T>(arg: Q<T>) { arg(10); }`
- [ ] Existing issue-211 extracted method fixtures keep their unsupported diagnostics
- [ ] The new diagnostic or support path is source-spanned at the callable parameter use

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOfConditionalTypeWithConcreteBranches.ts
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

This is related to `issues/open/5195-support-callable-interface-typed-local-calls.md`
but covers parameters whose callability comes from a conditional type alias. Keep
the implementation slice narrow enough to avoid taking on full type-system
evaluation.

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
