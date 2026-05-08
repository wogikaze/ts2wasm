---
id: 5253
title: "Report class expression decorator boundary"
type: feature
area: frontend/lexer
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Recognize `@` decorator syntax on class expressions enough to emit a
source-spanned TypeScript decorator boundary diagnostic instead of a generic
lexer `unsupported character: @` failure.

## Problem

Problem: `classExpressionWithDecorator1.ts` stops in tokenization at
`var v = @decorate class C { static p = 1 };` with `unsupported character: @`.

The TypeScript oracle parses the construct as a `ClassExpression` with a
`Decorator` child, then reports the later semantic diagnostic for missing
`decorate`. The current lexer failure prevents the reference from reaching the
decorator transform boundary described by the TypeScript parse/erase/emit
contract.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts
```

Current diagnostic:

```text
UnsupportedSyntax: unsupported character: @ at 62..63
```

Representative source:

```ts
// @experimentaldecorators: true
var v = @decorate class C { static p = 1 };
```

TypeScript AST evidence:

```text
VariableDeclaration -> ClassExpression "@decorate class C { static p = 1 }" -> Decorator "@decorate"
```

## Scope

In scope:

- [x] Tokenize `@` in TypeScript sources as decorator syntax rather than an unsupported character.
- [x] Recognize a decorator before a class expression in expression position.
- [x] Emit a source-spanned `UnsupportedTypeScriptSyntax` diagnostic for the decorator transform boundary.
- [x] Add focused frontend/CLI coverage for `var v = @decorate class C {}`.

Out of scope:

- Full decorator transform or runtime decorator calls.
- Decorator metadata emit.
- Parameter, method, property, accessor, or export-position decorator support beyond preserving the same boundary diagnostic if encountered.

## Affected paths

Expected:

- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `crates/cli/tests/`

Do not touch:

- runtime/backend decorator lowering
- TypeScript checker diagnostics

## Acceptance criteria

- [x] `classExpressionWithDecorator1.ts` no longer reports `unsupported character: @`.
- [x] A focused test proves `var v = @decorate class C {}` reaches a source-spanned decorator boundary diagnostic.
- [x] Existing non-decorator uses of unsupported characters still report lexer diagnostics.
- [x] If parsing advances to a broader decorator transform blocker, that next blocker is recorded separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithDecorator1.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/open/1186-implement-classExpressionWithDecorator.md`.
Related broad decorator bucket: `issues/open/4807-implement-decorator.md`.

## False-done audit

**truly-done** (5253)

- Implementation commits: verified via `git log --oneline --all --grep=5253`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
