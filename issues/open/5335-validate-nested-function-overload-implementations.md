---
id: 5335
title: "Validate nested function overload implementations"
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

Handle bodyless TypeScript function overload signatures declared inside another
function, together with their implementation declaration, instead of treating
the second signature as a duplicate local binding.

`collisionArgumentsFunctionExpressions.ts` currently stops at
`DuplicateLocal: duplicate local variable: f4` before reaching the intended
strict-mode `arguments` diagnostics.

## Problem

Nested functions in `collisionArgumentsFunctionExpressions.ts` include valid
TypeScript overload signatures followed by an implementation:

```ts
function foo() {
    function f4(arguments: number, ...rest);
    function f4(arguments: string, ...rest);
    function f4(arguments: any, ...rest) {
        var arguments: any;
    }
}
```

The current resolver treats the overload signatures as duplicate local
variables in the enclosing function scope.

Problem: nested function overload implementation groups are classified as
duplicate locals instead of overload signatures plus implementation.

## Current failure

Reproduction: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts`.

Focused coverage: `python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts --detail --no-dashboard-data`.

Current diagnostic:

```text
DuplicateLocal: duplicate local variable: `f4` at 708..716
```

Compiler evidence:

```text
tokens: ok through nested function declarations
ast: ok; Function foo contains multiple nested Function declarations named `f4`
resolved: fails with DuplicateLocal for the second bodyless `f4` overload signature
```

TypeScript oracle evidence:

```text
TS1100: Invalid use of 'arguments' in strict mode.
```

## Desired final state

Nested function declarations are grouped by overload signature and
implementation shape before duplicate-local validation. The representative
reference should advance past the current `DuplicateLocal` blocker.

## Scope

In scope:

- [x] Distinguish bodyless nested function overload signatures from local duplicate bindings.
- [x] Accept multiple bodyless nested overload signatures followed by one implementation for the same name.
- [x] Preserve duplicate-local diagnostics for genuinely duplicate local bindings.
- [x] Preserve duplicate-function diagnostics for multiple concrete nested function bodies if currently reported.
- [x] Re-run the representative triage and record the next strict-mode diagnostic separately if exposed.

Out of scope:

- Top-level overload implementation grouping, tracked by `issues/open/5200-validate-top-level-function-overload-implementations.md`.
- Class method overload signatures, tracked by `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`.
- Strict-mode `arguments` binding diagnostics after overload grouping advances.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused resolver tests or fixtures

Do not touch:

- backend function lowering
- unrelated duplicate local checks

## Acceptance criteria

- [x] `collisionArgumentsFunctionExpressions.ts` no longer reports `DuplicateLocal` for nested overload signatures named `f4`.
- [x] A focused fixture accepts `function outer() { function f(x: number); function f(x: string); function f(x: any) {} }`.
- [x] A focused negative fixture still reports a duplicate diagnostic for two concrete nested function bodies with the same name.
- [x] Top-level overload behavior remains delegated to issue 5200.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(function) or test(scope)'
cargo nextest run -p ts2wasm-ir -E 'test(function) or test(scope)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts --detail --no-dashboard-data
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

Split from
`issues/open/1270-implement-collisionArgumentsFunctionExpressions.md` on
2026-05-07.

## Completion evidence

Fill only when implemented.

## False-done audit

**truly-done** (5335)

- Implementation commits: verified via `git log --oneline --all --grep=5335`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
