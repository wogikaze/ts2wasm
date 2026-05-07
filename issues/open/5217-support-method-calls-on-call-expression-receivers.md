---
id: 5217
title: "Support method calls on call expression receivers"
type: feature
area: ir/lowering
class: implementation-ready
priority: P2
depends_on: [5001]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Support method calls where the receiver is itself a call expression, such as
`factory().foo()`.

## Problem

Problem: `castFunctionExpressionShouldBeParenthesized.ts` parses successfully,
but lowering rejects `.foo()` because the receiver is `(function a() { } as
any)()` instead of an identifier.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: method `foo` requires an identifier receiver
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```text
(function a() { } as any)().foo()
```

Observed failure:

```text
castFunctionExpressionShouldBeParenthesized.ts: issue-211 method `foo` requires an identifier receiver at 20..52
```

Compiler evidence:

```text
tokens: ok
ast: ok; Call(Member(Call(FunctionExpr a, args=[]), property="foo"), args=[])
resolved/lowered: issue-211 at .foo()
```

TypeScript AST evidence:

```text
CallExpression -> PropertyAccessExpression -> CallExpression -> ParenthesizedExpression -> AsExpression -> FunctionExpression
```

## Desired final state

Lowering supports method calls on call-expression receivers by evaluating the
receiver exactly once and binding it as the active receiver for the method call.

## Scope

In scope:

- [ ] Support the representative call-expression receiver `.foo()` shape.
- [ ] Preserve single evaluation of the call-expression receiver.

Out of scope:

- New-expression receivers already handled by issue 5142.
- Arbitrary builtin/prototype method semantics outside the receiver lowering
  shape.
- Optional chaining, computed method names, or extracted method calls.

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- focused compiler fixtures

Do not touch:

- parser grammar unless a regression proves the AST shape changes
- unrelated method-call builtin behavior

## Acceptance criteria

- [ ] `castFunctionExpressionShouldBeParenthesized.ts` no longer reports
  `method \`foo\` requires an identifier receiver`.
- [ ] A focused fixture proves `factory().method()` evaluates the receiver once
  before invoking the method.
- [ ] Existing unsupported diagnostics for extracted methods remain
  source-spanned.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(method) or test(receiver) or test(node_diff)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/castFunctionExpressionShouldBeParenthesized.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
