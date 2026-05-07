---
id: 5163
title: "Lower nested call expression callees"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #5163.

## Summary

The parser already builds AST for nested call expressions such as
`foo()(1).toString()`, `(new C(1))()`, and `(() => {})()`, but lowering rejects
call expressions whose callee is not a simple identifier. This blocks reference
cases before the compiler can report TypeScript-compatible accidental call
diagnostics or advance through supported IIFE shapes.

## Problem

Problem: `reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts` currently reports `UnsupportedSyntax: only identifier calls are supported in expression context` for `foo()(1 as number)`.

Additional representative: `reference/typescript/tests/cases/compiler/callOnInstance.ts` reports the same diagnostic for `(new D(1))()` before it can report TS2349 for calling an instance with no call signatures.

Additional representative: `reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing4.ts` reports the same diagnostic for an arrow-function IIFE inside a derived-class constructor:

```ts
(() => {
    this;  // No error
})();
```

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: only identifier calls are supported in expression context at 52..70
```

Representative source:

```ts
declare function foo(): string;

foo()(1 as number).toString();
foo()   (1 as number).toString();
foo()
(1 as number).toString();
```

Additional source:

```ts
declare class D { constructor(value: number); }
var s2: string = (new D(1))();

declare class C { constructor(value: number); }
(new C(1))();
```

Additional source:

```ts
class Derived extends Based {
    constructor() {
        (() => {
            this;
        })();
        super();
    }
}
```

Additional representative: `reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts` reports the same diagnostic for higher-order generic calls such as `fn2()(() => res2)` and `fn3()(() => res3)`.

Current compiler evidence:

- Tokens and AST succeed.
- The AST records nested `Call` expressions where the outer call callee is another `Call`, a `New` expression, or an `ArrowFn` expression.
- The pipeline reaches `lower_program` and fails in `crates/ir/src/lowered/resolver_expr.rs`.

TypeScript oracle evidence:

```text
TS2349: This expression is not callable.
  Type 'String' has no call signatures.
```

TypeScript reports TS2349 for all five accidental-call variants in the reference file.
For `callOnInstance.ts`, TypeScript reports TS2349 for `(new D(1))()` and
`(new C(1))()` because the constructed instances have no call signatures.

## Desired final state

Lowering supports, or explicitly diagnoses, call expressions whose callee is another expression rather than an identifier. The representative reference case should no longer hit the generic "only identifier calls" unsupported diagnostic.

## Scope

In scope:

- [x] Handle `Expr::Call { callee: Expr::Call { ... } }` in expression lowering with a source-spanned diagnostic or runtime-supported callable path.
- [x] Handle `Expr::Call { callee: Expr::New { ... } }` in expression lowering with the same source-spanned diagnostic family.
- [x] Handle `Expr::Call { callee: Expr::ArrowFn { ... } }` for arrow-function IIFE shapes with the same source-spanned diagnostic family or supported callable path.
- [x] Handle higher-order function call chains such as `fn2()(() => res2)` without the generic unsupported diagnostic.
- [x] Preserve existing identifier-call behavior.
- [x] Add focused coverage for `foo()(1).toString()` and the whitespace/newline accidental-call variants.
- [x] Re-run the representative triage and confirm the current generic unsupported diagnostic is gone.

Out of scope:

- Full TypeScript call-signature checking.
- General callable object semantics beyond this nested call-expression boundary.
- `super[...]()` call semantics, which remain under the broader issue 420 parent unless current triage proves the same child fixes them.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/ir/src/lowered/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`

Do not touch:

- backend/runtime code unless the lowered representation requires it after focused implementation.

## Acceptance criteria

- [x] `foo()(1).toString()` no longer reports `only identifier calls are supported in expression context`.
- [x] `(new D(1))()` no longer reports `only identifier calls are supported in expression context`.
- [x] `(() => { this; })()` in `checkSuperCallBeforeThisAccessing4.ts` no longer reports `only identifier calls are supported in expression context`.
- [x] `fn2()(() => res2)` in `circularReferenceInReturnType.ts` no longer reports `only identifier calls are supported in expression context`.
- [x] Whitespace and newline accidental-call variants from `betterErrorForAccidentalCall.ts` reach the same new diagnostic or lowered path.
- [x] Existing simple identifier calls continue to pass.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts` no longer reports the current generic unsupported diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-cli call
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/betterErrorForAccidentalCall.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/callOnInstance.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing4.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts
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

Split from generated bucket `1045` on 2026-05-06. Generated bucket `1096`
was folded in on the same date after fresh triage showed the same lowering
boundary for new-expression callees. Generated bucket `1143` was folded in
after fresh triage showed the same lowering boundary for arrow-function IIFE
callees. Generated bucket `1163` was folded in after fresh triage showed the
same lowering boundary for higher-order generic call chains. The broad
call-expression parent `420` remains blocked for unrelated `super[...]()` and
other call-expression feature families.

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
