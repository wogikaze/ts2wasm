---
id: 5188
title: "Report block-scoped function call arity diagnostics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`blockScopedSameNameFunctionDeclarationES5.ts` and `blockScopedSameNameFunctionDeclarationES6.ts` now build successfully, but TypeScript reports TS2554 arity diagnostics for calls that resolve to the wrong same-name function signature.

## Problem

The representative pattern is:

```ts
function foo(a: number) {
    if (a === 1) {
        function foo() { }
        foo();
        foo(10); // TS2554: expected 0 arguments
    }
    foo(); // TS2554: expected 1 argument
}
foo(); // TS2554: expected 1 argument
```

Parser, name resolution, and lowering now succeed. The remaining gap is a false-pass: same-name block-scoped function calls are not checked against the resolved function declaration arity in TSC semantic mode.

Problem: user-defined function calls that resolve to block-scoped same-name declarations can build even when TypeScript reports TS2554 arity errors.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES5.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES6.ts
```

Current compiler diagnostic:

```text
BuildPass: ts2wasm build succeeded
```

Compiler evidence:

- Tokens, AST, and resolved IR succeed for both files.
- Resolved IR contains calls to `foo()` and `foo(10)` inside nested branches and after the outer function.
- No compiler diagnostic is emitted for wrong argument counts.

TypeScript oracle evidence:

```text
TS2554: Expected 0 arguments, but got 1.
TS2554: Expected 1 arguments, but got 0.
```

Each representative file has four TS2554 diagnostics at the wrong-arity call sites.

## Desired final state

The compiler reports source-spanned arity diagnostics for wrong-argument calls to resolved user-defined functions in this same-name block-scoped function pattern.

## Scope

In scope:

- [ ] Check resolved direct calls against the resolved user function's required parameter count.
- [ ] Preserve the call-site span for wrong-arity diagnostics.
- [ ] Add focused coverage for ES5 and ES6 same-name block-scoped function declarations.

Out of scope:

- Builtin arity changes already tracked by builtin arity issues.
- Overload resolution or generic call signature selection.
- Optional/rest parameter completeness beyond the existing required-parameter model.
- Runtime argument padding/ignoring behavior.

## Affected paths

Expected:

- `crates/ir/src/semantic.rs`
- `crates/ir/src/lowered/validate.rs`
- `crates/cli/tests/ir_lowering.rs`
- `fixtures/`

Do not touch:

- Builtin resolver arity tables.
- Backend argument lowering.

## Acceptance criteria

- [ ] Focused tests cover `foo(10)` resolving to a zero-parameter block function.
- [ ] Focused tests cover `foo()` resolving to a one-parameter outer function.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES5.ts` no longer reports `BuildPass`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli --test ir_lowering
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES5.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedSameNameFunctionDeclarationES6.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from generated bucket `1077` on 2026-05-06. This is separate from builtin arity issues because the calls resolve to user-defined functions.

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
