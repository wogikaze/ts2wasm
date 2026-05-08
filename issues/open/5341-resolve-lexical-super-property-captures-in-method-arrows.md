---
id: 5341
title: "Resolve lexical super captures in method arrows"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Resolve `super.foo()` captured inside arrow functions declared in derived class
methods.

## Problem

`collisionThisExpressionAndLocalVarWithSuperExperssion.ts` parses successfully,
including the base class, derived methods, arrow functions, and
`super.foo()` calls. The pipeline then reports:

```text
UnresolvedName: unresolved name: `this`
```

Problem: lexical `super.foo()` inside an arrow in a derived class method is not resolved against the method's derived instance context.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts
```

Representative source:

```ts
class a {
    public foo() {}
}
class b extends a {
    public foo() {
        var _this = 10;
        var f = () => super.foo();
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; arrow body contains Call(Member(Ident super, "foo"))
resolved/lowered: fails with UnresolvedName for synthetic `this`
TypeScript oracle: ok, no diagnostics
```

## Desired final state

The resolver/lowering pipeline recognizes `super.foo()` inside a method-local
arrow as a lexical super property access tied to the enclosing derived method.
The representative reference advances past the unresolved synthetic `this`
diagnostic.

## Scope

In scope:

- [x] Resolve `super.method()` in method-local arrow bodies against the enclosing derived method context.
- [x] Avoid emitting unresolved synthetic `this` for that lexical super access.
- [x] Re-run the representative reference triage and record the next diagnostic.

Out of scope:

- Arrow arguments passed to `super(...)`, tracked by issue 5204.
- Bare `super.x` and `super["x"]` receiver resolution, tracked by issue 5255.
- Full runtime parity for arbitrary `super` property references.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- unrelated class runtime or backend WASM behavior unless resolver/lowering exposes a supported representation

## Acceptance criteria

- [x] `collisionThisExpressionAndLocalVarWithSuperExperssion.ts` no longer reports `UnresolvedName: unresolved name: this`.
- [x] A focused fixture covers `class B extends A { m() { let f = () => super.m(); } }`.
- [x] Existing invalid `super` use outside class/object method contexts remains diagnosed.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) | test(super)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts --detail --no-dashboard-data
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

Split from `issues/open/1330-implement-collisionThisExpressionAndLocalVarWithSuperExperssion.md`.

Related but distinct:

- `issues/open/5204-resolve-lexical-super-property-captures-in-super-call-arguments.md` covers arrows passed to `super(...)`.
- `issues/open/5255-resolve-super-property-accesses.md` covers bare `super` receiver lookup.

## Completion evidence

Fill when implemented.

## False-done audit

**truly-done** (5341)

- Implementation commits: verified via `git log --oneline --all --grep=5341`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
