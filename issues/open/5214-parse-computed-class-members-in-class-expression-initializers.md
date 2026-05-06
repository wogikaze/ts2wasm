---
id: 5214
title: "Parse computed class members in class expression initializers"
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

Accept computed class members inside class expressions used as default
parameter initializers.

## Problem

Problem: capturedParametersInInitializers parser coverage rejects `get [x]()` in
a class expression initializer because class member parsing expects `(` after
the getter name.

Current diagnostic:

```text
UnsupportedSyntax: expected LeftParen, got Some(LeftBracket)
```

TypeScript parses the class expression and reports TS2373 parameter capture
diagnostics after the parser succeeds.

## Current failure

Use the validation command below to reproduce the representative failure.

Observed failure:

```text
capturedParametersInInitializers2.ts: expected LeftParen, got Some(LeftBracket) at 92..93
```

Source shape:

```text
function foo(
    y = class {
        static c = x;
        get [x]() {return x;}
        constructor() { x; }
        [z]() { return z; }
    },
    x = 1,
    z = 2
) {
}
```

TypeScript AST evidence:

```text
Parameter -> ClassExpression -> GetAccessor -> ComputedPropertyName "[x]"
```

## Desired final state

The parser accepts computed getter and method names inside class expressions in
parameter default expressions.

## Scope

In scope:

- [ ] Parse computed class member names represented by
  `capturedParametersInInitializers2.ts`.

Out of scope:

- Class field initialization semantics after parsing succeeds.
- Parameter capture diagnostics after parsing succeeds.
- Broader class-expression generated buckets that require separate triage.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser/compiler fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `capturedParametersInInitializers2.ts` no longer reports
  `expected LeftParen, got Some(LeftBracket)` at `get [x]()`.
- [ ] A focused parser fixture proves `class { get [x]() {} [z]() {} }` parses
  inside a parameter default expression.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedParametersInInitializers2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedParametersInInitializers2.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
