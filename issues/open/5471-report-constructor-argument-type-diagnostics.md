---
id: 5471
title: "Report constructor argument type diagnostics"
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

Report TS2345-style diagnostics when a `new C(arg)` call passes an argument
that is incompatible with a simple annotated class constructor parameter.

This is the current false build-pass gap from `noErrorsInCallback.ts`.

## Problem

`noErrorsInCallback.ts` now parses, resolves, and builds successfully, including
the zero-argument arrow callback body. TypeScript still reports two TS2345
diagnostics because `{}` is passed to `constructor(public foo: string)` both
outside and inside the callback.

Problem: class constructor calls can build even when TypeScript reports an
argument type mismatch against a simple constructor parameter annotation.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noErrorsInCallback.ts
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noErrorsInCallback.ts --detail --no-dashboard-data
```

Current result:

```text
coverage: executed=1, build_pass=1, unsupported=0, blocked=0, semantic_enabled=0
triage: BuildPass / pass
```

Source context:

```ts
class Bar {
    constructor(public foo: string) { }
}
var one = new Bar({}); // Error
[].forEach(() => {
    var two = new Bar({}); // No error?
});
```

Compiler evidence:

```text
tokens: ok through class Bar, new Bar({}), [].forEach, zero-argument arrow, and the nested new Bar({})
ast: ok; constructor parameter property is represented as parameter foo, but the string type annotation is erased
resolved: ok; both New expressions are present, including the nested arrow body
```

TypeScript oracle:

```text
TS2345 at line 5, character 19: Argument of type '{}' is not assignable to parameter of type 'string'.
TS2345 at line 7, character 23: Argument of type '{}' is not assignable to parameter of type 'string'.
```

Oracle hints include constructor parameter `foo: string`, `one: Bar`, and
`two: Bar`.

## Desired final state

The semantic checker preserves enough simple constructor parameter type metadata
to report source-spanned TS2345-style diagnostics for object literals passed to
`string` constructor parameters, including calls inside arrow callback bodies.

## Scope

In scope:

- [ ] Preserve simple primitive constructor parameter annotations needed for `new C(...)` argument checks.
- [ ] Infer or classify object literal arguments as incompatible with `string` parameters for direct constructor calls.
- [ ] Check constructor argument compatibility both at top level and inside arrow callback bodies.
- [ ] Re-run `noErrorsInCallback.ts` and record the next semantic or compiler result.

Out of scope:

- Full TypeScript structural assignability.
- Generic, overload, union, optional, or rest constructor parameter semantics.
- Runtime constructor arity preservation, tracked separately by `issues/done/5286-preserve-class-constructor-parameters-for-new-arity.md`.
- Array callback library typing or contextual typing of `forEach` callback parameters.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/semantic.rs`
- focused semantic or lowering tests
- `fixtures/`

Do not touch:

- backend emit unless a focused regression proves semantic diagnostics must be emitted after lowering
- builtin array method implementations
- broad TypeScript type-system machinery

## Acceptance criteria

- [ ] A focused regression covers `class C { constructor(public s: string) {} } new C({});` and reports a source-spanned argument type diagnostic.
- [ ] A focused regression covers the same constructor call inside `[].forEach(() => { ... })`.
- [ ] `noErrorsInCallback.ts` no longer reports plain `BuildPass` in `reference-triage`; it reports the constructor argument mismatch or a narrower follow-up diagnostic.
- [ ] The implementation does not change unrelated successful `new C("ok")` calls into diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noErrorsInCallback.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noErrorsInCallback.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/open/3531-implement-noErrorsInCallback.md`.

Related but not duplicates:

- `issues/done/5286-preserve-class-constructor-parameters-for-new-arity.md`
  covers false zero-argument constructor arity in lowering, not source-spanned
  TS2345-style type compatibility diagnostics.
- `issues/open/5188-report-block-scoped-function-call-arity-diagnostics.md`
  covers wrong-arity diagnostics for user-defined function calls, not class
  constructor argument type compatibility.

## Completion evidence

Fill when implemented.
