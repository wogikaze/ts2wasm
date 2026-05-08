---
id: 5450
title: "Support function-typed parameter local calls"
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

Support or precisely classify direct calls to function parameters with ordinary
function type annotations, such as `subFunc: () => T[]`.

Split from generated bucket
`issues/open/3452-implement-narrowingAssignmentReadonlyRespectsAssertion.md`.

## Problem

Problem: `narrowingAssignmentReadonlyRespectsAssertion.ts` parses and resolves,
then lower_program rejects `subFunc()` with the generic issue-211
function-valued local call diagnostic.

The representative function parameter has a direct function type annotation,
not a callable interface, conditional type alias, ambient const, or initialized
function expression. It should be handled before the generic extracted-method
issue-211 fallback.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `subFunc(...)` are not supported; call receiver.method(...) directly at 584..593
```

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts
```

Representative source:

```ts
function dataFunc<T>(subFunc: () => T[]): MultiCaseFixture<T> {
  return { cases: subFunc() };
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; interfaces, generic functions, object literal return, and call expression parse
resolved: ok through builtins
lower_program: issue-211 at `subFunc()`
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

Calls to function-typed parameters are represented or classified before the
generic issue-211 extracted-method path. The representative path should advance
beyond `subFunc()` to later readonly/narrowing behavior or a narrower
source-spanned diagnostic.

## Scope

In scope:

- [ ] Preserve callable metadata for parameters annotated with direct function
  types such as `() => T[]`.
- [ ] Support or classify direct parameter calls like `subFunc()`.
- [ ] Preserve existing issue-211 diagnostics for arbitrary extracted methods
  and unrelated function-valued local calls.
- [ ] Re-run the representative triage and record the next blocker.

Out of scope:

- Callable interface locals, tracked by
  `issues/open/5195-support-callable-interface-typed-local-calls.md`.
- Callable conditional-typed parameter calls, tracked by
  `issues/open/5196-support-callable-conditional-typed-parameter-calls.md`.
- Ambient callable const local calls, tracked by
  `issues/open/5374-support-callable-ambient-const-local-calls.md`.
- Initialized function-expression local calls, tracked by
  `issues/open/5440-support-initialized-function-expression-local-calls.md`.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused CLI/reference fixtures

Do not touch:

- unrelated method receiver lowering
- backend/runtime ABI unless lowering already has a supported callable
  representation for this parameter shape

## Acceptance criteria

- [ ] `narrowingAssignmentReadonlyRespectsAssertion.ts` no longer reports
  generic issue-211 for `subFunc()`.
- [ ] A focused fixture covers `function f(g: () => number) { return g(); }`.
- [ ] Existing issue-211 extracted-method diagnostics remain source-spanned.
- [ ] Any later readonly assignment/narrowing diagnostic from the representative
  path is recorded here or split into a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function) or test(call)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts --detail --no-dashboard-data
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

This issue intentionally keeps the callable source narrow: direct function type
parameters only. Broader callable local families have their own owners listed
in the out-of-scope section.

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
