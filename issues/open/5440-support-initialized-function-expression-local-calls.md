---
id: 5440
title: "Support initialized function expression local calls"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Support direct calls to a local variable initialized with a function
expression, such as `var recurser = function foo() { recurser(); }`.

Split from generated bucket `3425`.

## Problem

`namedFunctionExpressionCall.ts` parses and resolves its named function
expressions, but lowering stops at the direct call to the local variable that
holds the function expression:

```ts
var recurser = function foo() {
    // using the local name
    foo();

    // using the globally visible name
    recurser();
};
```

The compiler reports the generic issue-211 extracted-method/function-valued
local diagnostic at `recurser()`, while TypeScript accepts the reference with no
diagnostics.

Problem: initialized function-expression locals fall into the generic issue-211 function-valued local call boundary.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts
```

Observed result:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `recurser(...)` are not supported; call receiver.method(...) directly at 140..150
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Compiler evidence:

```text
tokens: ok through `var recurser = function foo() { ... }` and `(function bar() { ... });`
ast: Let recurser = FunctionExpr name foo; body contains Call foo() and Call recurser()
resolved/lowered: lower_program fails at issue-211 for recurser()
```

TypeScript oracle:

```text
ok; no diagnostics
binding recurser has type () => void
```

## Desired final state

The compiler classifies calls to local bindings initialized by function
expressions before the generic issue-211 extracted-method path. The
representative should advance past `recurser()` to the inline named function
expression self-call or build successfully.

## Scope

In scope:

- [ ] Track local bindings initialized with `FunctionExpr`.
- [ ] Support or precisely classify direct calls to those initialized local function bindings.
- [ ] Preserve existing support for named function expression self-calls such as `foo()` inside `function foo()`.
- [ ] Re-triage `namedFunctionExpressionCall.ts` and record the next diagnostic or build result.

Out of scope:

- Uninitialized function-typed local diagnostics, tracked by `issues/open/5279-report-function-typed-local-call-definite-assignment.md`.
- Loop-local arrow calls from arrow closures, tracked by `issues/open/5215-support-loop-local-arrow-calls-from-arrow-closures.md`.
- Callable interface locals, tracked by `issues/open/5195-support-callable-interface-typed-local-calls.md`.
- Arbitrary extracted method calls where the callee is not a known local function binding.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused lowering/reference tests

Do not touch:

- unrelated method receiver lowering
- backend/runtime ABI unless fresh implementation evidence proves it is required

## Acceptance criteria

- [ ] `namedFunctionExpressionCall.ts` no longer reports generic issue-211 at `recurser()`.
- [ ] A focused fixture covers `var recurser = function foo() { recurser(); }`.
- [ ] A focused fixture or reference evidence covers inline named function expression self-call `(function bar() { bar(); });` after `recurser()` advances.
- [ ] Existing unsupported extracted-method/function-valued local diagnostics remain unchanged for non-function-expression locals.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function) or test(closure) or test(call)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionCall.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Issue `273` records completed support for named function expression self-calls.
This issue covers calling through the outer local binding that stores the
function expression.

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
