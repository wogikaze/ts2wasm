---
id: 5257
title: "Parse object type literal construct signatures"
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

Parse and erase TypeScript construct signatures inside object type literals,
such as `{ new(): Object }`, when they appear in annotations.

## Problem

Problem: `classExtendsInterfaceInExpression.ts` fails before class heritage can
be triaged because the function return type annotation contains an object type
literal construct signature:

```ts
function factory(a: any): {new(): Object} {
  return null;
}
```

The parser treats `new()` as an expression and fails at the construct
signature return type colon:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: RightParen, ... }) at 67..68
```

TypeScript accepts the construct signature as `TypeLiteral -> ConstructSignature`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts
```

Current diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 66, end: 67 } }) at 67..68
```

## Scope

In scope:

- [ ] Parse `new (...): Type` members inside object type literals used in
  annotations.
- [ ] Erase the construct signature so runtime parsing continues after the
  annotation.
- [ ] Cover zero-argument construct signatures like `{ new(): Object }`.
- [ ] Preserve existing object type literal property and method signature
  erasure.

Out of scope:

- Runtime construct-signature semantics.
- Type checking construct signatures.
- Interface construct signatures, tracked by
  `issues/done/5245-iterator-protocol-runtime.md`.
- Class heritage call-expression support for `extends factory(A)`, tracked
  separately by `issues/done/5252-support-call-expression-class-heritage.md`
  if it remains after this parser blocker advances.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- runtime construct-signature behavior
- class heritage resolver support unless fresh triage proves this parser slice
  already advanced

## Acceptance criteria

- [ ] `classExtendsInterfaceInExpression.ts` no longer reports the current
  `unsupported expression ... RightParen` diagnostic at `{new(): Object}`.
- [ ] A focused parser or CLI fixture covers
  `function f(): { new(): Object } { return null; }`.
- [ ] Existing object type literal annotation erasure still works.
- [ ] If this exposes the later `extends factory(A)` blocker, record or link it
  to the class heritage call-expression owner.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli -E 'test(parser) | test(type)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/open/1201-implement-classExtendsInterfaceInExpression.md`.
Related parser owner: `issues/done/5245-iterator-protocol-runtime.md`.
