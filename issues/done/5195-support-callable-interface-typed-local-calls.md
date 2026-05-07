---
id: 5195
title: "Support callable interface-typed local calls"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-07
---

## Summary

Support calling local variables whose TypeScript-only type annotation is a
callable interface, or report a more precise source-spanned diagnostic before
lowering reaches the generic `issue-211` function-valued local call boundary.

## Problem

`var i: I<string>; var y = i("");` parses and resolves `i`, but lowering rejects
the call as a function-valued local call. TypeScript preserves callable
interface information and reports the earlier definite-assignment diagnostic for
`i`, not an unsupported call-form parser or lowering error.

Additional representative
`callSignaturesShouldBeResolvedBeforeSpecialization.ts` parses an
`I1<string>` local and calls `test(...)`; lowering stops at the same generic
`issue-211` boundary before TypeScript's call-signature specialization can
reject the boolean argument.

Additional representative `contextualTypingOfTooShortOverloads.ts` now reaches
the dedicated `issue-5195` boundary for `var use: Overload; use(...)`, where
`Overload` is an interface with multiple call signatures. TypeScript reports
TS2454 definite-assignment diagnostics before any unsupported function
resolution boundary.

Problem: callable interface-typed locals currently lower to `Undefined` values and calls to them stop with `issue-211`.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callSignaturesShouldBeResolvedBeforeSpecialization.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfTooShortOverloads.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-211: function-valued local calls such as extracted method `i(...)` are not supported; call receiver.method(...) directly at 92..97
error: [UnsupportedTypeScriptSyntax] issue-5195: callable interface-typed local `use` is not callable — the variable is never assigned at 96..117
```

Source:

```ts
interface I<T> {
    <U extends T>(u: U): U;
}
var i: I<string>;
var y = i("");
```

Triage evidence:

- AST succeeds with `Let i = Undefined` and `Let y = Call(Ident i, String "")`.
- Visible symbols include local bindings `i` and `y`.
- TypeScript oracle reports TS2454, `Variable 'i' is used before being assigned`, at the call site.
- For `callSignaturesShouldBeResolvedBeforeSpecialization.ts`, AST succeeds
  with `var test!: I1<string>;`, `test("expects boolean instead of string")`,
  and `test(true)`.
- TypeScript oracle reports TS2345 for `test(true)` after resolving the
  callable interface signature against `I1<string>`.
- For `contextualTypingOfTooShortOverloads.ts`, AST succeeds with
  `Let use = Undefined`, `Call(Ident use, ArrowFn req,res)`, and later
  `app.use(...)`; TypeScript oracle reports TS2454 for `use` and `app`.

## Desired final state

The compiler no longer reports the generic `issue-211` function-valued local
call diagnostic for callable interface-typed locals. The representative case
either reaches a source-spanned definite-assignment/type diagnostic or supports
the callable-local path when the value is initialized.

## Scope

In scope:

- [x] Preserve enough callable interface metadata for local variables with call signatures
- [x] Detect calls to uninitialized callable interface locals before generic lowering rejection
- [x] Specialize generic callable interface signatures before classifying local
  calls
- [x] Keep existing unsupported diagnostics for arbitrary extracted method calls

Out of scope:

- Full TypeScript type inference for generic call signatures
- Runtime support for arbitrary function-valued locals beyond this callable-interface shape
- Method receiver semantics for `obj.method()` or extracted class methods

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/` unless lowering already produces a supported callable representation
- unrelated method-call receiver behavior

## Acceptance criteria

- [x] `callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts` no longer reports the generic `issue-211` extracted-method diagnostic for `i("")`
- [x] `callSignaturesShouldBeResolvedBeforeSpecialization.ts` no longer reports
  the generic `issue-211` extracted-method diagnostic for `test(...)`
- [x] `contextualTypingOfTooShortOverloads.ts` no longer reports
  `issue-5195` or an unspanned `UnresolvedFunction` for the local `use(...)`
  call
- [x] A focused fixture covers calling an uninitialized callable interface local and reports a source-spanned diagnostic at `i`
- [x] A focused fixture covers `interface I1<T> { (value: T): void; field1:
  I1<boolean>; }` with a local `I1<string>` call, and the boolean argument path
  reaches a type diagnostic instead of unsupported lowering
- [x] Existing issue-211 extracted method fixtures continue to report unsupported diagnostics
- [x] Valid direct function declarations and arrow/function variable calls keep passing

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callSignaturesShouldBeResolvedBeforeSpecialization.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfTooShortOverloads.ts
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

Issue 211 intentionally keeps arbitrary extracted method calls unsupported. This
slice is narrower: locals annotated with callable interface types should be
classified before they fall into the extracted-method diagnostic path.

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
