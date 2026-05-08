---
id: 5146
title: "Report for-await context errors before async runtime diagnostics"
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

Implement the narrow parse/validation slice that distinguishes `for await...of` in non-async contexts from the broader unsupported async-iteration runtime diagnostic.

## Problem

The representative TypeScript case contains `for await` and `await` in ordinary functions, exported functions, arrow functions, class methods, nested functions, and top-level code. TypeScript reports context errors such as TS1103 and TS1308. The compiler currently stops at the first `for await` with the generic issue-230 async-iteration runtime-subset diagnostic.

Problem: `for await...of` outside async/top-level-allowed contexts currently reports the broad async runtime unsupported message before a TypeScript-aligned context diagnostic can be emitted.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts
```

Current diagnostic:

```text
error: [UnsupportedRuntimeSubset] issue-230: `for await...of` async iteration requires Promise and async iterator runtime semantics, which are not supported in this milestone at 138..147
```

Source context:

```text
3 | // https://github.com/Microsoft/TypeScript/issues/26586
4 |
5 | function normalFunc(p: Promise<number>) {
6 |   for await (const _ of []);
7 |   return await p;
8 | }
```

TypeScript oracle evidence:

```text
TS1103: 'for await' loops are only allowed within async functions and at the top levels of modules.
TS1308: 'await' expressions are only allowed within async functions and at the top levels of modules.
```

Current compiler evidence:

```text
tokens: For at 138..141 followed by Await at 142..147
AST dump: fails before AST with issue-230 generic async-iteration diagnostic
```

## Desired final state

When `for await...of` appears in a non-async function or other disallowed context, the compiler emits a source-spanned context diagnostic before the generic async-runtime unsupported diagnostic. Valid async contexts may continue to report the existing issue-230 runtime-subset diagnostic until async iteration is implemented.

## Scope

In scope:

- [x] Track enough parser statement context to know when `for await...of` is outside an async/top-level-allowed context.
- [x] Emit a clear source-spanned diagnostic for the disallowed non-async context.
- [x] Add a focused parser/diagnostic fixture for `function f() { for await (const x of []); }`.
- [x] Re-run the representative triage and confirm it no longer reports the generic issue-230 message for the first non-async `for await`.

Out of scope:

- Implementing async iteration runtime semantics.
- Top-level await module configuration support.
- General async function lowering.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`

Do not touch:

- async runtime lowering
- backend async iterator support

## Acceptance criteria

- [x] A focused diagnostic test for `for await` inside an ordinary function reports the new context diagnostic.
- [x] The diagnostic span covers the `await` keyword in `for await`.
- [x] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts` no longer reports the generic issue-230 async-iteration message as the first blocker.
- [x] Existing valid-context async/for-await unsupported diagnostics remain source-spanned and issue-linked.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend for_await_non_async_context
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts --detail
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

Split from generated bucket `issues/open/1019-implement-awaitInNonAsyncFunction.md`.

Related follow-up for ordinary `await(...)` call parsing:

- `issues/open/5145-parse-await-as-call-outside-async-context.md`

Also owns `issues/open/3360-implement-modulePreserveTopLevelAwait.md`: fresh
triage for `modulePreserveTopLevelAwait1.ts` stops at top-level
`for await (const x of []) {}` with issue-230 before TypeScript's TS1432
context diagnostic and before the following top-level `await Promise.resolve()`
diagnostic become actionable.

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
