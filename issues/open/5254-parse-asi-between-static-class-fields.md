---
id: 5254
title: "Parse ASI between static class fields"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Accept automatic semicolon insertion between static class field declarations
when a field without an initializer is followed by another `static` field on a
later line.

## Problem

Problem: `classExpressionWithStaticProperties2.ts` parses `static b` as the
start of a class element, then fails at the following `static c = ...` with
`expected LeftParen, got Some(Static)`.

TypeScript accepts the boundary as two static property declarations:

```ts
var v = class C {
    static b
    static c = {
        x: "hi"
    }
}
```

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties2.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected LeftParen, got Some(Static) at 99..105
```

TypeScript AST evidence:

```text
ClassExpression -> PropertyDeclaration "static c = { x: \"hi\" }" -> StaticKeyword "static"
```

Related generated bucket `autoAsiForStaticsInClassDeclaration.ts` shows the
same boundary in a class declaration:

```ts
class C {
    static x
    static y
}
```

## Scope

In scope:

- [ ] Accept ASI after `static name` before a later-line `static` class element.
- [ ] Cover class declarations and class expressions with the same parser rule.
- [ ] Preserve method parsing for `static name()` and field initializer parsing for `static name = expr`.
- [ ] Add focused parser coverage for `static b` followed by `static c = ...`.

Out of scope:

- Runtime semantics for static public fields.
- Full class expression lowering.
- ASI policy for every class element form beyond this static-field boundary.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/cli/tests/`

Do not touch:

- runtime/backend static field lowering
- TypeScript type checking

## Acceptance criteria

- [ ] `classExpressionWithStaticProperties2.ts` no longer reports `expected LeftParen, got Some(Static)`.
- [ ] `autoAsiForStaticsInClassDeclaration.ts` no longer reports `expected LeftParen, got Some(Static)`.
- [ ] A focused parser test covers `class C { static b\nstatic c = 1 }`.
- [ ] Existing static method parsing remains unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/autoAsiForStaticsInClassDeclaration.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties2.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/open/1188-implement-classExpressionWithStaticProperties-parser-syntax.md`.
Related generated bucket: `issues/open/773-implement-autoAsiForStaticsInClassDeclaration.md`.

Additional superseded bucket:

- `issues/open/1190-implement-classExpressionWithStaticPropertiesES-parser-syntax.md`
  contributes `classExpressionWithStaticPropertiesES62.ts`, the same class
  expression boundary with `static b` followed by later-line `static c = ...`.
  Fresh triage on 2026-05-06 shows ES61 is already build-pass.
