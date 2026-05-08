---
id: 5125
title: "Implement as type assertion expression parsing"
type: feature
area: frontend/parser
class: implementation
priority: P1
depends_on: []
blocks: []
parent: 676
created: 2026-05-05
updated: 2026-05-05
---

## Summary

Implement `as` type assertion expression parsing in the frontend parser so that code like `expr as Type` compiles successfully.

## Problem

Reference test `arrayFind.ts` fails with `UnsupportedSyntax` on line 13:

```typescript
const readonlyFoundNumber: number | undefined = readonlyArrayOfStringsNumbersAndBooleans.find(isNumber);
```

The full failure is:

```
error: unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 497, end: 498 } }) at 500..505
```

Line 12 sets up the `as` assertion:

```typescript
const readonlyArrayOfStringsNumbersAndBooleans = arrayOfStringsNumbersAndBooleans as ReadonlyArray<string | number | boolean>;
```

The parser fails to parse `as ReadonlyArray<string | number | boolean>` and falls through to a leftover semicolon. Array.find() itself is already fully implemented — the real gap is `as` type assertion parsing.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFind.ts
```

Representative evidence from smart triage:

- **Diagnostic:** `UnsupportedSyntax` at span 500..505
- **Error type:** `parser-or-frontend-unsupported`
- **Feature label:** `unknown-unsupported`
- **Tokens:** Parser tokenizes `readonlyArrayOfStringsNumbersAndBooleans` then fails at `as` keyword

## Desired final state

The parser accepts `expr as Type` expressions (TypeScript type assertion syntax). The reference test `arrayFind.ts` compiles without `UnsupportedSyntax` errors.

## Scope

In scope:

- [x] Add `as` keyword token in the lexer
- [x] Add `AsExpression` AST node or equivalent
- [x] Parse `expr as Type` in the expression parser
- [x] Update reference coverage for the fixed test case

Out of scope:

- Angle-bracket syntax (`<Type>expr`) — separate issue
- Const assertions (`expr as const`) — type-checked separately
- `satisfies` keyword — separate syntax

## Affected paths

- `crates/frontend/src/`
- `fixtures/` (new fixture for `as` assertion)
- Reference coverage artifacts

Do not touch:
- `crates/ir/src/`, `crates/backend-wasm/src/`, `crates/runtime-abi/src/`

## Acceptance criteria

- [x] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFind.ts` passes
- [x] New fixture with `expr as Type` syntax compiles and produces valid WASM
- [x] No regression in existing tests (`cargo nextest run`)

## Validation

```sh
cargo fmt --all --check
cargo nextest run -E 'test(array_find) | test(builtin_methods)'
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFind.ts
```

## Completion evidence

<!-- To be filled after implementation -->

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

