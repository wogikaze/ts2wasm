---
id: 5474
title: "Parse element-access for-in assignment heads"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Parse `for-in` statement heads whose left-hand side is an element-access
assignment target, such as `for (n[idx++] in m);`.

This is the current blocker from `noImplicitAnyForIn.ts`.

## Problem

`noImplicitAnyForIn.ts` parses ordinary declaration-form `for-in` loops, but
fails on the final assignment-form loop:

```ts
for (n[idx++] in m);
```

Current compiler diagnostic:

```text
UnsupportedSyntax: expected Equal, got Some(LeftBracket) at 578..579
```

Problem: the parser treats `n` as the start of an assignment statement and
expects `=`, so it cannot recognize an element-access expression as the
left-hand side of a `for-in` head.

## Current failure

Fresh focused coverage for
`reference/typescript/tests/cases/compiler/noImplicitAnyForIn.ts` shows:

```text
executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
```

Fresh triage shows:

```text
line 33: for (n[idx++] in m);
diagnostic: UnsupportedSyntax expected Equal, got Some(LeftBracket) at 578..579
```

Compiler evidence:

```text
tokens: ok through earlier declaration-form for-in loops
ast/resolved: fail before AST construction at the `[` in `n[idx++]`
visible symbols include x, i, j, _j, k, k1, k2, a, b, c, idx, m, n
```

TypeScript oracle:

```text
AST path: ForInStatement -> ElementAccessExpression `n[idx++]` -> PostfixUnaryExpression `idx++`
TS2405: The left-hand side of a 'for...in' statement must be of type 'string' or 'any'.
```

## Desired final state

The parser accepts element-access assignment targets in `for-in` heads and
advances this reference case past the current generic `expected Equal`
unsupported syntax boundary. Any later type diagnostic or lowering limitation
is recorded with source-spanned evidence.

## Scope

In scope:

- [ ] Parse `for (<element-access-expression> in <expr>) <stmt>` as a `for-in` statement head without requiring `=`.

Out of scope:

- Full TypeScript noImplicitAny diagnostics for indexed element access.
- Call-expression `for-in` left-hand sides, tracked by broader test262 parser-syntax buckets.
- `for-of` assignment heads and destructuring assignment heads.
- Runtime semantics for assigning each enumerated key back through element access.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/ast.rs`

Do not touch:

- backend-wasm lowering for this slice unless parser changes require an explicit unsupported lowering diagnostic
- unrelated declaration-form `for-in` / `for-of` loops
- reference dashboard artifacts

## Acceptance criteria

- [ ] A focused parser regression covers `for (n[idx++] in m);` and produces a `ForIn` statement instead of `expected Equal`.
- [ ] `noImplicitAnyForIn.ts` no longer reports `UnsupportedSyntax: expected Equal, got Some(LeftBracket)`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend for_in
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyForIn.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyForIn.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/done/3536-implement-noImplicitAnyForIn.md`.

Related but not duplicates:

- `issues/open/442-implement-parser-syntax.md` is a broad generated test262
  parser-syntax bucket and is not an implementation-ready owner for this
  TypeScript fixture.
- `issues/open/438-implement-negative-parse-syntaxerror.md` covers negative
  parse SyntaxError triage, not this valid TypeScript `for-in` head form.

## Completion evidence

Fill when implemented.
