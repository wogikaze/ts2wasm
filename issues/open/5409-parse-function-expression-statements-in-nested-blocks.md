---
id: 5387
title: "Parse function expression statements in nested blocks"
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

Accept parenthesized function expression statements after block-scoped
declarations inside nested statement blocks.

## Problem

Problem: capturedLetConstInLoop parser tests reject `(function() { return x })`
inside a nested block after `let x;`.

Current diagnostic:

```text
UnsupportedSyntax: expected Comma, got Some(Ident("x"))
```

TypeScript accepts the representative files with no diagnostics.

## Current failure

Use the validation commands below to reproduce the representative failures.

Observed failures:

```text
capturedLetConstInLoop9.ts: expected Comma, got Some(Ident("x")) at 133..134
capturedLetConstInLoop9_ES6.ts: expected Comma, got Some(Ident("x")) at 132..133
```

Source shape:

```text
for (let x = 0; x < 1; ++x) {
    let x;
    (function() { return x });
    {
        let x;
        (function() { return x });
    }
}
```

## Desired final state

The parser treats the parenthesized `function` expression as a valid expression
statement inside nested blocks and does not misparse the function body return
identifier as an object/member comma site.

## Scope

In scope:

- [ ] Parse the nested-block parenthesized function expression statement after
  `let x;`.

Out of scope:

- Captured binding semantics after parsing succeeds.
- Function expression statement cases outside the represented nested-block
  parser window.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser/compiler fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `capturedLetConstInLoop9.ts` no longer reports `expected Comma, got
  Some(Ident("x"))` at the nested function expression statement.
- [ ] `capturedLetConstInLoop9_ES6.ts` no longer reports `expected Comma, got
  Some(Ident("x"))` at the same statement.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop9.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop9_ES6.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop9.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
