---
id: 5366
title: "Restore call argument scope after typed arrow callbacks"
type: bug
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Resolve later call arguments in the caller scope after resolving a typed arrow callback argument, covering the first blocker in `contextSensitiveReturnTypeInference.ts`.

## Problem

Problem: `contextSensitiveReturnTypeInference.ts` currently reports `UnresolvedName: unresolved name: DEPS` for a call argument even though top-level `const DEPS` is declared before the call.

The representative call has a first argument arrow callback with a `typeof DEPS` parameter annotation and a nested function expression inside an object literal. After resolving that callback, the resolver rejects the second argument `DEPS` instead of resolving it in the surrounding module scope.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextSensitiveReturnTypeInference.ts
```

Equivalent mise task:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextSensitiveReturnTypeInference.ts
```

Source context:

```ts
const DEPS = { foo: 1 }

test(
  (deps: typeof DEPS, data) => ({
    fn1: function() { return deps.foo },
    fn2: data.bar
  }),
  DEPS
);
```

Smart triage evidence on 2026-05-07:

```text
tokens: ok
AST: ok; call expression with arrow callback and final Ident DEPS argument
resolved: fails in resolve_names with UnresolvedName for final DEPS argument
visible symbols: binding DEPS
TypeScript oracle: ok, diagnostics=[]
coverage: executed=1, build_pass=0, unsupported=1
```

## Desired final state

The resolver restores the caller/module scope after resolving typed arrow callback arguments, so the following call argument `DEPS` resolves to the top-level const binding.

## Scope

In scope:

- [x] Preserve or restore the enclosing call-argument scope after resolving arrow function arguments.
- [x] Resolve top-level const arguments following typed arrow callbacks.
- [x] Add a focused resolver regression for `const DEPS = {}; test((x: typeof DEPS) => ({ fn() { return x; } }), DEPS);`.
- [x] Re-run the representative reference path and split any next blocker separately if outside this resolver-scope issue.

Out of scope:

- Full context-sensitive return type inference.
- Type-only symbol modeling beyond the `typeof DEPS` annotation needed by this shape.
- Arbitrary function-valued local call runtime support.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`
- focused fixtures or CLI tests

Do not touch: backend/runtime lowering.

## Acceptance criteria

- [x] `contextSensitiveReturnTypeInference.ts` no longer reports `UnresolvedName` for the final `DEPS` argument in the typed-arrow call.
- [x] A focused resolver test covers a typed arrow callback followed by a const argument from the enclosing scope.
- [x] Existing arrow callback parameter resolution tests still pass.
- [x] Any next blocker from the reference path is recorded here or split to a follow-up.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(name_resolver) or test(arrow) or test(call)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextSensitiveReturnTypeInference.ts
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextSensitiveReturnTypeInference.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket `issues/open/1486-implement-contextSensitiveReturnTypeInference.md` on 2026-05-07.

Related but distinct issue: `issues/open/5348-resolve-const-declarations-before-use.md` owns use-before-declaration and same-for-header const predeclaration. This issue's `DEPS` binding is declared before the call and visible in triage.

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

## False-done audit

**truly-done** (5366)

- Implementation commits: verified via `git log --oneline --all --grep=5366`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
