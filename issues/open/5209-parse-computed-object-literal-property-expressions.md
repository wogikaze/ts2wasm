---
id: 5209
title: "Parse computed object literal property expressions"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: [5000]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse object literal properties with computed expression keys such as
`{ [name + ".a"]: () => value }`.

## Problem

Problem: the parser expects a simple dotted/key form inside computed object
literal property names and rejects binary expressions like `name + ".a"`.

Current diagnostic:

```text
UnsupportedSyntax: expected Dot, got Some(Plus)
```

TypeScript accepts the representative file with no diagnostics.

## Current failure

Use the validation command below to reproduce the representative failure.

Observed failure:

```text
capturedLetConstInLoop13.ts: expected Dot, got Some(Plus) at 257..258
```

Source shape:

```text
this.bar({
    [name + ".a"]: () => { this.foo(name); },
});
```

TypeScript AST evidence:

```text
ObjectLiteralExpression
  PropertyAssignment
    ComputedPropertyName
      BinaryExpression `name + ".a"`
```

## Desired final state

The parser accepts computed object literal property names whose key expression
is a binary expression, and preserves the computed key expression for later
resolution/lowering.

## Scope

In scope:

- [ ] Parse the representative object-literal computed property expression.

Out of scope:

- Computed property names in binding patterns; that is issue 5180.
- Class member computed property parser buckets.
- Runtime semantics for computed property evaluation after parsing succeeds.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser/compiler fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `capturedLetConstInLoop13.ts` no longer reports `expected Dot, got
  Some(Plus)` for `[name + ".a"]`.
- [ ] A focused parser fixture proves object literals accept computed property
  names with binary key expressions.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop13.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop13.ts --detail
```

Not run:

- none

## Completion evidence

2026-05-07 additional evidence:

- `issues/done/1487-implement-contextualComputedNonBindablePropertyType.md`
  reaches the same computed object-literal key expression boundary for
  `[testD()]`, failing with `expected Dot, got Some(LeftParen)` before mapped
  type/contextual typing behavior can run.

Fill when implemented.
