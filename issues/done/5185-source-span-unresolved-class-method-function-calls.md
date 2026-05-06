---
id: 5185
title: "Source-span unresolved class method function calls"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`blockScopedFunctionDeclarationInStrictClass.ts` reaches the intended out-of-block `foo()` error, but the compiler reports `UnresolvedFunction` without a source span while TypeScript reports `TS2304` at the `foo` identifier.

## Problem

The parser and AST correctly preserve the strict class method body:

```ts
class c {
    method() {
        if (true) {
            function foo() { }
            foo(); // ok
        }
        foo(); // not ok
    }
}
```

The first call resolves inside the `if` block, but the second call leaves the block-scoped function declaration scope and fails later as `UnresolvedFunction` without a span. That makes the diagnostic less actionable than the TypeScript oracle and keeps the generated bucket open as a function-resolution blocker.

Problem: unresolved function calls from class method bodies lose source-span information and should report the out-of-scope call site.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictClass.ts
```

Current compiler diagnostic:

```text
UnresolvedFunction: unresolved function: `foo`
```

Compiler evidence:

- Tokens and AST construction succeed.
- AST contains the nested block function `foo`, an in-block `foo()` call, and an out-of-block `foo()` call at byte span `150..155`.
- Lowering fails after resolved/builtin/type IR stages with an unspanned `UnresolvedFunction`.

TypeScript oracle evidence:

```text
TS2304: Cannot find name 'foo'.
```

The oracle reports the diagnostic at the out-of-block `foo` identifier on line 8, character 9.

## Desired final state

The compiler reports a source-spanned unresolved-function/name diagnostic at the out-of-block class method call site. The representative case should no longer emit an unspanned `UnresolvedFunction`.

## Scope

In scope:

- [x] Preserve the call-site span for unresolved direct function calls in class method bodies.
- [x] Report the diagnostic at the out-of-block `foo` identifier or call expression span.
- [x] Keep the in-block `foo()` call resolved.
- [x] Add focused coverage for a strict class method with a block-scoped function declaration and an out-of-block call.

Out of scope:

- Changing block-scoped function declaration semantics.
- Hoisting `foo` outside the `if` block.
- Broader function-resolution builtin coverage.
- Runtime class method dispatch.

## Affected paths

Expected:

- `crates/ir/src/semantic.rs`
- `crates/ir/src/lowered/resolver_extra.rs`
- `crates/ir/src/lowered/resolver.rs`
- `crates/cli/tests/ir_lowering.rs`
- focused fixtures/tests for unresolved class method calls

Do not touch:

- Parser class-method syntax unless a focused test proves the span was already lost before IR.
- Backend/runtime class dispatch.

## Acceptance criteria

- [x] A focused test covers `class c { method() { if (true) { function foo(){} foo(); } foo(); } }`.
- [x] The in-block `foo()` call remains accepted.
- [x] The out-of-block `foo()` diagnostic includes a source span at `foo` or the `foo()` call expression.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictClass.ts` no longer reports an unspanned `UnresolvedFunction`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir unresolved
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictClass.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from generated bucket `1073` on 2026-05-06. The intended semantic outcome already matches TypeScript, but the current compiler diagnostic loses source position in the class-method lowering path.

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

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
