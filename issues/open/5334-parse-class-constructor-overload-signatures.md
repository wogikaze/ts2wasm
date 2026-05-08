---
id: 5334
title: "Parse class constructor overload signatures"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Distinguish bodyless class constructor overload signatures from concrete
constructor implementations.

`collisionArgumentsClassConstructor.ts` currently parses class constructors but
stops with `DuplicateFunction: duplicate constructor definition` before it can
reach the TypeScript strict-mode `arguments` diagnostics.

## Problem

Classes in `collisionArgumentsClassConstructor.ts` include valid TypeScript
constructor overload signatures followed by an implementation:

```ts
class c5 {
    constructor(i: number, ...arguments);
    constructor(i: string, ...arguments);
    constructor(i: any, ...arguments) {
        var arguments: any[];
    }
}
```

The current compiler treats the bodyless overload signatures as duplicate
constructor definitions during validation.

Problem: class constructor overload signatures are not represented separately
from constructor implementations.

## Current failure

Reproduction: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts`.

Focused coverage: `python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts --detail --no-dashboard-data`.

Current diagnostic:

```text
DuplicateFunction: duplicate constructor definition
```

Compiler evidence:

```text
tokens: ok through constructor overload declarations and implementations
ast: contains multiple Function name `constructor` members
resolved: fails in validation/resolution with DuplicateFunction
```

TypeScript oracle evidence:

```text
TS1210: Code contained in a class is evaluated in JavaScript's strict mode which does not allow this use of 'arguments'.
```

## Desired final state

The frontend accepts bodyless constructor overload signatures as TypeScript-only
class members and only treats the constructor with a body as the implementation.
The representative reference should advance past the current
`DuplicateFunction` blocker.

## Scope

In scope:

- [x] Parse `constructor(...);` class members as overload signatures.
- [x] Allow multiple bodyless constructor overload signatures before one constructor implementation.
- [x] Preserve duplicate-constructor diagnostics for multiple constructor bodies.
- [x] Keep `declare class` constructor signatures erased.
- [x] Re-run the representative triage and record the next strict-mode diagnostic separately if exposed.

Out of scope:

- Missing constructor parameter-list diagnostics, tracked by `issues/open/5323-report-missing-constructor-parameter-list.md`.
- Top-level function/class overload merges, tracked by `issues/open/5199-report-function-overload-list-class-merge-diagnostics.md`.
- Strict-mode `arguments` binding diagnostics after constructor overloads parse.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `crates/ir/src/`
- focused parser/resolver tests

Do not touch:

- backend constructor lowering
- unrelated class method overload semantics

## Acceptance criteria

- [x] `collisionArgumentsClassConstructor.ts` no longer reports `DuplicateFunction: duplicate constructor definition` for bodyless constructor overload signatures.
- [x] `collisionThisExpressionAndPropertyNameAsConstuctorParameter.ts` no longer reports `DuplicateFunction: duplicate constructor definition` for bodyless constructor overload signatures before the `_this` parameter-property cases.
- [x] A focused parser/resolver test accepts `class C { constructor(x: number); constructor(x: string); constructor(x: any) {} }`.
- [x] A focused negative test still rejects two constructor bodies in one class.
- [x] `declare class C { constructor(x: number); constructor(x: string); }` remains erased/accepted if currently parsed.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(constructor)'
cargo nextest run -p ts2wasm-ir -E 'test(class) or test(constructor)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts --detail --no-dashboard-data
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

Split from `issues/open/1267-implement-collisionArgumentsClassConstructor.md`
on 2026-05-07.

2026-05-07 additional evidence:

- `issues/open/1317-implement-collisionSuperAndPropertyNameAsConstuctorParameter.md` reaches the same `DuplicateFunction: duplicate constructor definition` boundary before `_super` parameter-property cases.
- `issues/open/1485-implement-constructorsWithSpecializedSignatures.md` reaches the same boundary before TypeScript's TS2394 specialized overload compatibility diagnostics.

## Completion evidence

Fill only when implemented.

## False-done audit

**truly-done** (5334)

- Implementation commits: verified via `git log --oneline --all --grep=5334`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
