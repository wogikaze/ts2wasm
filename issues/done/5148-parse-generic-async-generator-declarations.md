---
id: 5148
title: "Parse generic async generator declarations"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement the narrow parser slice for `async function* f<T extends ...>(): AsyncGenerator<...> { }` so generic async generator declarations do not fail at the `<` after the function name.

## Problem

The representative TypeScript case tokenizes correctly but the parser expects `(` immediately after the generator function name. TypeScript accepts the generic async generator declaration and reports no diagnostics. The compiler should erase the type parameters and return type before handing off to the existing async/generator runtime boundary.

Problem: generic async generator declarations currently fail with `UnsupportedSyntax` at the type parameter list.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Less) at 115..116
```

Source context:

```text
2 | // @target: esnext
3 |
4 | // https://github.com/microsoft/TypeScript/issues/51984
5 | async function* f<T extends Promise<never>>(): AsyncGenerator<T, void, void> { }
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none.
TypeScript AST: FunctionDeclaration `async function* f<T extends Promise<never>>(): AsyncGenerator<T, void, void> { }`.
```

Current compiler evidence:

```text
tokens: Async Function Star Ident("f") Less Ident("T") Extends Ident("Promise") Less Ident("never") RightShift ...
AST: parser fails before AST with expected LeftParen at the type-parameter `<`.
```

## Desired final state

The parser erases TypeScript type parameters and return annotations on async generator declarations, so this source no longer fails at the `<` token. If async generator runtime semantics remain unsupported, the next diagnostic should be the existing source-spanned async/generator runtime boundary, not parser syntax.

## Scope

In scope:

- [ ] Allow a balanced TypeScript generic parameter list after `async function* f`.
- [ ] Allow and erase the return type annotation after the parameter list.
- [ ] Add a focused parser test for `async function* f<T extends Promise<never>>(): AsyncGenerator<T, void, void> { }`.
- [ ] Re-run the representative triage and confirm it no longer reports `expected LeftParen`.

Out of scope:

- Implementing async generator runtime semantics.
- General async-function lowering.
- Test262 harness feature admission, tracked by issue 5134.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- backend async runtime support
- Python test262 harness feature whitelist

## Acceptance criteria

- [ ] A focused parser test shows the generic async generator declaration parses or reaches the existing async-generator boundary without the `<` parser error.
- [ ] The representative triage no longer reports `expected LeftParen, got Some(Less)`.
- [ ] Return type `AsyncGenerator<T, void, void>` is erased without corrupting the following function body.
- [ ] Existing non-generic async/generator diagnostics remain source-spanned.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend generic_async_generator_declaration
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket `issues/done/1023-implement-awaitedTypeCrash.md`.

Related broader issues:

- `issues/open/059-implement-parser-syntax-extensions.md`
- `issues/open/5134-admit-generators-and-async-functions-through-python-harness.md`

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
