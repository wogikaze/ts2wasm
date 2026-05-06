---
id: 5194
title: "Report empty call type arguments"
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

Handle malformed TypeScript call type-argument lists with empty slots, such as
`Foo<a,,b>()`, without falling back to a generic semicolon parser error.

## Problem

The parser recognizes the start of a call expression with type arguments but
stops at the first comma in an empty type-argument slot. TypeScript keeps the
call-expression shape and reports `TS1110: Type expected` at the second comma.

Problem: `Foo<a,,b>();` currently reports `expected Semicolon, got Some(Comma)` instead of a source-spanned missing type-argument diagnostic.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callExpressionWithMissingTypeArgument1.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Comma) at 24..25
```

Source:

```ts
Foo<a,,b>();
```

Triage evidence:

- Tokens are `Foo`, `<`, `a`, `,`, `,`, `b`, `>`, `(`, `)`, `;`.
- TypeScript AST keeps a top-level `CallExpression` for `Foo<a,,b>()`.
- TypeScript oracle reports `TS1110: Type expected` at the empty type-argument slot, plus unresolved names for `Foo`, `a`, and `b`.

## Desired final state

The parser handles the generic-call boundary well enough to report a missing
type argument at the empty slot. The representative case no longer stops with
`expected Semicolon, got Some(Comma)`.

## Scope

In scope:

- [x] Detect empty entries in call type-argument lists
- [x] Emit a source-spanned missing type diagnostic at the empty slot
- [x] Preserve valid generic call type-argument erasure

Out of scope:

- Full TypeScript type checking for call type arguments
- Resolving `Foo`, `a`, or `b`
- JSX parsing changes

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [x] `Foo<a,,b>();` reports a missing type-argument diagnostic at the empty slot, not `expected Semicolon`
- [x] Existing valid generic call fixtures still parse and erase type arguments
- [x] A focused parser or CLI diagnostic test covers the malformed call type-argument list
- [x] `callExpressionWithMissingTypeArgument1.ts` advances beyond the current comma parser error

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(parser)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callExpressionWithMissingTypeArgument1.ts
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

Existing generic-call work under issue 059 intentionally kept the erasure guard
narrow. This issue only covers malformed type-argument list diagnostics after
the parser has already committed to a generic call.

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
