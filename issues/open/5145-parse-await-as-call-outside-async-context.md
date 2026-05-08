---
id: 5145
title: "Parse await as an identifier call outside async contexts"
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

Implement the narrow context-sensitive parsing rule for `await(...)` inside a non-async function, matching TypeScript's treatment of `await` as an identifier call rather than an await expression in that context.

## Problem

The lexer emits `Token::Await`, and the expression parser currently turns it into `Expr::Await` unconditionally. In a non-async function, TypeScript treats `await(Promise.resolve(1))` as a call expression whose callee is the identifier `await`, then reports TS2311 because the name is not available there. The compiler instead reaches the async-runtime subset diagnostic.

Problem: `await(...)` in a sync function currently fails with `UnsupportedRuntimeSubset` instead of preserving the TypeScript AST shape and producing a name-resolution style diagnostic.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitCallExpressionInSyncFunction.ts
```

Current compiler diagnostic:

```text
error: [UnsupportedRuntimeSubset] issue-294: await is only supported for Bun.file("/dev/stdin").text() stdin lowering in this slice at 52..76
```

Source context:

```text
1 | // @target: esnext
2 |
3 | function foo() {
4 |    const foo = await(Promise.resolve(1));
5 |    return foo;
6 | }
```

TypeScript oracle evidence:

```text
TS2311: Cannot find name 'await'. Did you mean to write this in an async function?
TypeScript AST path: CallExpression `await(Promise.resolve(1))` with Identifier `await` at line 4, column 16.
```

Current compiler evidence:

```text
tokens: Token::Await at 52..57
AST: Let foo = Await(Call(Member(Ident("Promise"), "resolve"), [Number(1)]))
resolved: resolve_builtins reports issue-294 async runtime subset
```

## Desired final state

Inside ordinary non-async functions, `await(...)` is parsed as a call expression with callee identifier `await`, so the compiler advances to a source-spanned unresolved-name diagnostic or an equivalent TypeScript-aligned unsupported diagnostic instead of async-runtime lowering.

## Scope

In scope:

- [x] Track enough parser function context to distinguish ordinary functions from async functions for unary `await`.
- [x] Parse `await(...)` in ordinary functions as `Call(Ident("await"), ...)`.
- [x] Preserve existing `Expr::Await` parsing where async contexts are explicitly supported or intentionally diagnosed.
- [x] Add a focused parser/triage regression for `function foo() { const x = await(Promise.resolve(1)); }`.

Out of scope:

- Implementing async function runtime semantics.
- Changing `for await...of` diagnostics.
- Top-level await module semantics.

## Affected paths

Expected:

- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- async runtime lowering
- Bun stdin await lowering covered by issue 294

## Acceptance criteria

- [x] A focused parser test shows `await(Promise.resolve(1))` inside `function foo()` parses as a call whose callee is identifier `await`.
- [x] The representative triage no longer reports `issue-294: await is only supported for Bun.file`.
- [x] The new diagnostic remains source-spanned at the `await` call site.
- [x] Existing async/for-await unsupported diagnostics still point to issue 230 or the current async-runtime issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend await_call_expression_in_sync_function
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitCallExpressionInSyncFunction.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitCallExpressionInSyncFunction.ts --detail
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

Split from generated bucket `issues/open/1016-implement-awaitCallExpressionInSyncFunction.md`.

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


## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
