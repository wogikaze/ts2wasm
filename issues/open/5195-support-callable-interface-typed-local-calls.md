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
updated: 2026-05-06
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

Problem: callable interface-typed locals currently lower to `Undefined` values and calls to them stop with `issue-211`.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-211: function-valued local calls such as extracted method `i(...)` are not supported; call receiver.method(...) directly at 92..97
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

## Desired final state

The compiler no longer reports the generic `issue-211` function-valued local
call diagnostic for callable interface-typed locals. The representative case
either reaches a source-spanned definite-assignment/type diagnostic or supports
the callable-local path when the value is initialized.

## Scope

In scope:

- [ ] Preserve enough callable interface metadata for local variables with call signatures
- [ ] Detect calls to uninitialized callable interface locals before generic lowering rejection
- [ ] Keep existing unsupported diagnostics for arbitrary extracted method calls

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

- [ ] `callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts` no longer reports the generic `issue-211` extracted-method diagnostic for `i("")`
- [ ] A focused fixture covers calling an uninitialized callable interface local and reports a source-spanned diagnostic at `i`
- [ ] Existing issue-211 extracted method fixtures continue to report unsupported diagnostics
- [ ] Valid direct function declarations and arrow/function variable calls keep passing

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
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
