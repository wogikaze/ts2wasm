---
id: 5419
title: "Parse parenthesized self-closing JSX expressions"
type: feature
area: frontend/jsx
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Represent a minimal parenthesized self-closing JSX expression such as
`(<div />);` instead of leaving the parser at `RightParen`.

## Problem

`modulePreserve3.ts` tokenizes the JSX-looking expression as
`LeftParen, Less, Ident("div"), Slash, Greater, RightParen, Semicolon`, but the
parser does not represent the JSX element and later reports an unsupported
expression at the closing parenthesis.

Problem: self-closing JSX expressions in expression position are not parsed or
rejected at the JSX element boundary.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve3.ts
```

Current diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 276, end: 277 } }) at 277..278
```

Source context:

```tsx
export namespace JSX {}

// @Filename: /index.tsx
export {};
(<div />);
```

Compiler evidence:

```text
tokens: ok; `(<div />);` is tokenized as LeftParen, Less, Ident("div"), Slash, Greater, RightParen, Semicolon.
visible symbols: []
ast/resolved: fail at RightParen after the JSX-looking token sequence.
```

TypeScript oracle:

```text
Top level includes ModuleDeclaration `export namespace JSX {}`, ExportDeclaration
`export {};`, and ExpressionStatement `(<div />);`.
```

## Desired final state

The frontend reports a source-spanned JSX unsupported diagnostic at the
`<div />` boundary or represents a minimal self-closing JSX expression node
well enough for the reference to advance to the next diagnostic.

## Scope

In scope:

- [ ] Handle `(<div />);` as a JSX expression boundary in the parser.
- [ ] Add one focused frontend parser test for a parenthesized self-closing JSX element.
- [ ] Re-run `modulePreserve3.ts` triage and record the next diagnostic.

Out of scope:

- JSX attributes, children, fragments, spread tags, and emit/runtime behavior.
- JSX factory and React namespace semantics.
- Full JSX type checking.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs` if a minimal JSX AST representation is needed
- focused frontend JSX/parser tests

Do not touch:

- backend/runtime code
- React or JSX emit lowering
- module graph resolution

## Acceptance criteria

- [ ] `(<div />);` no longer reports unsupported expression at `RightParen`.
- [ ] A focused test proves the diagnostic or AST span points at the JSX element boundary.
- [ ] Existing non-JSX comparison and less-than expressions still parse normally.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend jsx
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve3.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserve3.ts --detail --no-dashboard-data
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

Split from `issues/done/3358-implement-modulePreserve.md`.

Related but not duplicates:

- `issues/open/5230-tokenize-jsx-elements-before-regexp-fallback.md` covers a
  lexer fallback where JSX closing tags become unterminated RegExp literals.
  This issue starts after tokenization succeeds and targets the parser's
  parenthesized self-closing JSX expression boundary.

## Completion evidence

Fill when implemented.
