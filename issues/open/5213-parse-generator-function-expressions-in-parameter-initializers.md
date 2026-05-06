---
id: 5213
title: "Parse generator function expressions in parameter initializers"
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

Accept `function*` expressions when they appear inside default parameter
initializers.

## Problem

Problem: capturedParametersInInitializers parser coverage rejects
`(function*() { yield z })()` inside a parameter initializer at the `*`.

Current diagnostic:

```text
UnsupportedSyntax: expected LeftParen, got Some(Star)
```

TypeScript parses the construct and then reports later TS2373 parameter
capture diagnostics.

## Current failure

Use the validation command below to reproduce the representative failure.

Observed failure:

```text
capturedParametersInInitializers1.ts: expected LeftParen, got Some(Star) at 654..655
```

Source shape:

```text
function foo7(y = (function*() {yield z})(), z = 1) {
}
```

Parser evidence:

```text
tokens: ok
ast: UnsupportedSyntax expected LeftParen, got Some(Star)
resolved: same parser failure
```

## Desired final state

The parser treats `function*` as a valid generator function expression in
expression position, including nested inside parameter initializer expressions.

## Scope

In scope:

- [ ] Parse the exact parenthesized generator function expression initializer
  represented by `capturedParametersInInitializers1.ts`.

Out of scope:

- Generator runtime semantics.
- Async generator declarations or generic async generator declarations.
- Parameter capture diagnostics after parsing succeeds.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser/compiler fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `capturedParametersInInitializers1.ts` no longer reports
  `expected LeftParen, got Some(Star)` at the `function*` initializer.
- [ ] A focused parser fixture proves `(function*() { yield z })()` parses as a
  parameter default expression.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedParametersInInitializers1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedParametersInInitializers1.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
