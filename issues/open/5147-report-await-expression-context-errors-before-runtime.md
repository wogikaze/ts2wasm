---
id: 5147
title: "Report await expression context errors before runtime diagnostics"
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

Implement the narrow validation slice that reports `await` expression context errors in ordinary functions before the generic async runtime-subset diagnostic.

## Problem

The parser correctly builds `Expr::Await` for `await 'literal'`, `await 1`, and other literal await expressions. In ordinary non-async functions, TypeScript reports TS1308 because await expressions are not allowed there. The compiler currently reaches `resolve_builtins` and reports the broader issue-294 runtime-subset message.

Problem: `await <literal>` in a non-async function currently fails with `UnsupportedRuntimeSubset` instead of a TypeScript-aligned source-spanned context diagnostic.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitLiteralValues.ts
```

Current compiler diagnostic:

```text
error: [UnsupportedRuntimeSubset] issue-294: await is only supported for Bun.file("/dev/stdin").text() stdin lowering in this slice at 50..65
```

Source context:

```text
1 | // @target: es2015
2 | function awaitString() {
3 |     await 'literal';
4 | }
5 |
6 | function awaitNumber() {
```

TypeScript oracle evidence:

```text
TS1308: 'await' expressions are only allowed within async functions and at the top levels of modules.
```

Current compiler evidence:

```text
AST: Function awaitString body contains Expr::Await(String("literal")) at 50..65.
Resolved pipeline: resolve_builtins reports issue-294 async runtime subset.
The same file repeats the pattern for number, true, false, null, and undefined literals.
```

## Desired final state

When `Expr::Await` appears in an ordinary non-async function, the compiler emits a source-spanned context diagnostic before async runtime lowering. Valid async/top-level contexts may continue to report existing issue-linked async runtime diagnostics until async semantics are implemented.

## Scope

In scope:

- [x] Track enough frontend or resolver context to know whether `Expr::Await` is inside an ordinary function.
- [x] Emit a clear source-spanned diagnostic for disallowed `await` expressions in non-async functions.
- [x] Add a focused diagnostic fixture for `function f() { await 1; }`.
- [x] Re-run the representative triage and confirm it no longer reports issue-294 as the first blocker.

Out of scope:

- Implementing async function runtime semantics.
- Top-level await module configuration support.
- Treating `await(...)` as identifier call outside async contexts; tracked by issue 5145.
- `for await...of` context diagnostics; tracked by issue 5146.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/ir/src/builtin_resolver.rs`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `fixtures/`

Do not touch:

- Bun stdin await lowering covered by issue 294
- backend async runtime support

## Acceptance criteria

- [x] A focused diagnostic test for `function f() { await 1; }` reports the new context diagnostic.
- [x] The diagnostic span covers the `await` keyword or full await expression.
- [x] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitLiteralValues.ts` no longer reports `issue-294: await is only supported for Bun.file` as the first blocker.
- [x] Existing Bun stdin await lowering still works.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend await_literal_values_context
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitLiteralValues.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitLiteralValues.ts --detail
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

Split from generated bucket `issues/open/1020-implement-awaitLiteralValues.md`.

Related await-context issues:

- `issues/open/5145-parse-await-as-call-outside-async-context.md`
- `issues/open/5146-report-for-await-context-errors-before-async-runtime.md`

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

Audit result: retained in issues/open/. Implementation commits confirmed.
