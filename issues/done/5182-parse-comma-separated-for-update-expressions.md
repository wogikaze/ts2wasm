---
id: 5182
title: "Parse comma-separated for update expressions"
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

`blockScopedBindingsReassignedInLoop2.ts` stops at the comma in a for-loop update list: `for (...; ...; ++x, --y)`.

## Problem

The parser accepts a single for-loop update expression such as `++x`, then expects `)` immediately. TypeScript parses `++x, --y` as a comma expression in the update slot, and the representative reference file has no TypeScript diagnostics.

Problem: comma-separated for-loop update expressions are parser-unsupported, blocking the block-scoped loop reassignment reference window.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop2.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: expected RightParen, got Some(Comma) at 53..54
```

Representative source:

```ts
for (let x = 1, y = 2; x < y; ++x, --y) {
    let a = () => x++ + y++;
    if (x == 1) {
        break;
    }
    else {
        a();
    }
}
```

Compiler evidence:

- Token dump includes `Increment`, `Ident("x")`, `Comma`, `Decrement`, and `Ident("y")` in the update slot.
- AST construction fails before representing the `ForStatement`.
- Visible symbol extraction sees only `x`; the parser has not yet modeled the full `let x = 1, y = 2` declaration list.

TypeScript oracle evidence:

```text
TypeScript reports no diagnostics for the representative file.
```

TypeScript AST evidence at the failing span:

```text
ForStatement -> BinaryExpression -> PrefixUnaryExpression -> CommaToken
```

## Desired final state

The frontend accepts comma-separated update expressions in `for` statements and preserves their left-to-right execution order for the focused `++x, --y` pattern. The representative case should no longer fail with `expected RightParen, got Some(Comma)`.

## Scope

In scope:

- [x] Parse `for (...; ...; ++x, --y)` without treating the comma as a syntax error.
- [x] Represent the update slot as an ordered expression sequence or equivalent AST shape.
- [x] Preserve existing single-update support from issue `268`.
- [x] Add focused parser/frontend coverage for comma-separated prefix update expressions in a for-loop update slot.
- [x] Re-run representative triage and confirm the current comma parser blocker is gone.

Out of scope:

- Multi-declarator `let x = 1, y = 2` semantics beyond what is needed to keep this parser slice focused.
- Postfix update expressions in arbitrary value positions.
- Closure/lifetime behavior for loop variables captured by arrows.
- General comma expression support outside the `for` update slot.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/compiler/src/dump.rs`
- `crates/ir/src/builtin_resolver.rs`

Do not touch:

- Backend closure object ABI.
- General scope-analysis diagnostics.
- Broad comma-expression support outside this update-slot slice.

## Acceptance criteria

- [x] `for (let x = 1; x < 3; ++x, --y) {}` parses without `expected RightParen, got Some(Comma)`.
- [x] Single-update loops such as `for (let x = 1; x < 3; ++x) {}` remain accepted.
- [x] The update expressions preserve left-to-right order in the frontend/dump or resolved representation.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop2.ts` no longer reports the current comma parser diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend for
cargo nextest run -p ts2wasm-ir update
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop2.ts
```

Impacted commands:

```sh
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

Split from generated bucket `1069` on 2026-05-06. This issue only removes the first parser blocker in the five-file `blockScopedBindingsReassignedInLoop` scope-analysis bucket.

## Completion evidence

Commits:

- Combined with the current commit.

### Changes

1. **`statements_general.rs`**: For-loop update expression parsing now consumes comma-separated expressions via a `while self.consume(Comma)` loop.

Validation:
- cargo nextest run -p ts2wasm-frontend: 200 passed
- blockScopedBindingsReassignedInLoop2.ts: no longer `expected RightParen, got Comma` (now UnresolvedName)

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- Later triage may expose multi-declarator `let` handling, postfix update expression values, or captured loop binding semantics after the comma update list parses.
