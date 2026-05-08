---
id: 5337
title: "Parse rest parameter constructor overload signatures"
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

Handle TypeScript class constructor overload signatures that include rest
parameters, without treating bodyless signatures as duplicate constructor
implementations.

## Problem

`collisionRestParameterClassConstructor.ts` parses class constructors and rest
parameters, then fails during validation:

```text
DuplicateFunction: duplicate constructor definition
```

Problem: bodyless constructor overload signatures with rest parameters are
classified as duplicate constructor implementations.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts
```

Representative source:

```ts
class c5 {
    constructor(_i: number, ...rest);
    constructor(_i: string, ...rest);
    constructor(_i: any, ...rest) {
        var _i: any;
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; ClassDecl contains bodyless constructor signatures and one implementation
resolved: DuplicateFunction duplicate constructor definition
TypeScript oracle: ok, diagnostics []
```

## Desired final state

The compiler accepts bodyless rest-parameter constructor overload signatures as
TypeScript-only class members and keeps the constructor with a body as the
implementation.

## Scope

In scope:

- [x] Accept multiple bodyless constructor overload signatures with rest parameters before one constructor implementation.
- [x] Preserve duplicate-constructor diagnostics for multiple constructor bodies.
- [x] Keep `declare class` constructor signatures with rest parameters accepted.

Out of scope:

- Non-rest constructor overload signatures, tracked by `issues/open/5334-parse-class-constructor-overload-signatures.md`.
- Top-level function and class method overload work.
- Later collision diagnostics after this blocker advances.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `crates/ir/src/`
- focused parser/resolver tests

Do not touch:

- backend constructor lowering
- unrelated overload semantics

## Acceptance criteria

- [x] `collisionRestParameterClassConstructor.ts` no longer reports `DuplicateFunction: duplicate constructor definition`.
- [x] A focused parser/resolver test accepts `class C { constructor(x: number, ...rest); constructor(x: string, ...rest); constructor(x: any, ...rest) {} }`.
- [x] A focused negative test still rejects two constructor bodies in one class.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(constructor)'
cargo nextest run -p ts2wasm-ir -E 'test(constructor)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts
```

Impacted commands:

```sh
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

Split from `issues/open/1301-implement-collisionRestParameterClassConstructor.md`
on 2026-05-07.

## Completion evidence

Fill only when implemented.

## False-done audit

**truly-done** (5337)

- Implementation commits: verified via `git log --oneline --all --grep=5337`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
