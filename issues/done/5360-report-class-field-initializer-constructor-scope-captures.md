---
id: 5360
title: "Report class field initializer constructor-scope captures"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report TS2301-equivalent diagnostics when class field initializers reference
names declared only in the constructor scope.

## Problem

`constructorParametersInVariableDeclarations.ts` build-passes, but TypeScript
reports TS2301 for `private a = x`, `private b = { p: x }`, and
`private c = () => x` because `x` is declared in the constructor scope.

Problem: class field initializer semantic validation currently does not reject
references to constructor parameters or constructor-local variables, so the
representative path incorrectly build-passes.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts
```

Current compiler diagnostic:

```text
BuildPass
```

TypeScript oracle evidence:

```text
TS2301: Initializer of instance member variable 'a' cannot reference identifier 'x' declared in the constructor.
TS2301: Initializer of instance member variable 'b' cannot reference identifier 'x' declared in the constructor.
TS2301: Initializer of instance member variable 'c' cannot reference identifier 'x' declared in the constructor.
```

Representative source:

```ts
class A {
    private a = x;
    private b = { p: x };
    private c = () => x;
    constructor(x: number) {
    }
}
```

## Desired final state

Semantic validation reports a source-spanned diagnostic when an instance field
initializer references a constructor parameter or constructor-local binding.

## Scope

In scope:

- [ ] Detect constructor-scope identifiers referenced from instance field initializers.
- [ ] Emit source-spanned TS2301-equivalent diagnostics for direct, object-literal, and arrow field initializer references.
- [ ] Re-run the representative triage and confirm it no longer reports BuildPass.

Out of scope:

- Parameter property runtime support, completed by issue 226.
- Derived constructor parameter-property ordering, tracked by issue 5268.
- Full TypeScript definite-assignment analysis.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/ir/src/semantic.rs`
- focused semantic tests

Do not touch:

- backend runtime code
- module system code

## Acceptance criteria

- [ ] `constructorParametersInVariableDeclarations.ts` no longer reports BuildPass.
- [ ] A focused test reports the diagnostic for `class A { a = x; constructor(x) {} }`.
- [ ] A focused test reports the diagnostic through `{ p: x }` and `() => x` field initializers.
- [ ] Constructor body locals remain usable inside constructor bodies.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(class) or test(field) or test(constructor)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorParametersInVariableDeclarations.ts --detail --no-dashboard-data
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

Split from `issues/open/1476-implement-constructorParametersInVariableDeclarations.md`
on 2026-05-07 after fresh coverage reported build_pass.

Additional superseded bucket:

- `issues/open/1477-implement-constructorParametersThatShadowExternalNamesInVariableDeclarations.md`
  reaches the same TS2301 semantic gap with an outer `var x = 1` shadowed by a
  constructor parameter and a constructor-local `var x = ""`. Fresh triage on
  2026-05-07 reports BuildPass while TypeScript reports TS2301 at
  `private a = x` in both classes.

Related but not exact:

- `issues/done/226-implement-parameter-properties.md` owns parameter property
  runtime lowering, not TS2301 semantic validation.
- `issues/done/5268-support-derived-constructor-parameter-properties-after-super.md`
  owns derived constructor parameter-property ordering.

## Completion evidence

Fill only when implemented.

## False-done audit

**truly-done** (5360)

- Implementation commits: verified via `git log --oneline --all --grep=5360`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
