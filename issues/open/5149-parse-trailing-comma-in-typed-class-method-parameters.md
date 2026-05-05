---
id: 5149
title: "Parse trailing comma in typed class method parameters"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement the narrow parser slice for TypeScript class method parameter lists that end with a trailing comma after typed parameters.

## Problem

The representative reference case reaches a generic private class method with typed parameters and a trailing comma before `)`. The parser consumes the comma and then expects another binding identifier or pattern, failing on the closing parenthesis.

Problem: class method parameter lists with a trailing comma currently fail with `UnsupportedSyntax`, even though TypeScript accepts the source.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-247: expected binding identifier or pattern, got Some(RightParen) at 424..425
```

Source context:

```text
16 |   private handleResolve<TResult>(
17 |     result: NotPromise<TResult> | Thenable<NotPromise<TResult>>,
18 |     resolve: Receiver<TResult>,
19 |   ) {
20 |     if (result instanceof Thenable) {
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none.
TypeScript AST path: ClassDeclaration -> MethodDeclaration `private handleResolve<TResult>(...)`.
```

Current compiler evidence:

```text
tokens include `resolve: Receiver<TResult>, )`.
AST/resolved: parser fails before AST with issue-247 expected binding identifier or pattern at the closing `)`.
```

## Desired final state

The parser accepts a trailing comma immediately before `)` in function/class method parameter lists after TypeScript type annotations, erases the type-only parameter metadata, and continues into the method body.

## Scope

In scope:

- [ ] Accept trailing comma before `)` in class method parameter lists.
- [ ] Preserve existing support for TypeScript parameter type annotations and generic class method type parameters.
- [ ] Add a focused parser test for `private handleResolve<TResult>(result: T, resolve: R,) {}`.
- [ ] Re-run the representative triage and confirm it no longer reports `expected binding identifier or pattern, got Some(RightParen)`.

Out of scope:

- Full `Awaited<T>` type semantics.
- Runtime behavior for private methods beyond the existing supported subset.
- General type checking for `PromiseLike` and `Thenable`.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- async runtime lowering
- TypeScript type checker behavior

## Acceptance criteria

- [ ] A focused parser test accepts a class method parameter list with a trailing comma after typed parameters.
- [ ] The representative triage no longer reports the issue-247 RightParen parameter-list diagnostic.
- [ ] Existing parameter-list parser tests continue to pass.
- [ ] Any next blocker from the representative case is recorded separately if it is outside this trailing-comma slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend typed_class_method_parameter_trailing_comma
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitedTypeNoLib.ts --detail
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

Split from generated bucket `issues/done/1024-implement-awaitedTypeNoLib.md`.

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
