---
id: 5454
title: "Parse while statements with non-block bodies"
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

Support `while` statement bodies that are valid statements but not block
statements, starting with empty-statement bodies and expression-statement
bodies.

Split from generated bucket
`issues/done/3463-implement-narrowingPlainJsNoCrash.md`.

## Problem

Problem: `narrowingPlainJsNoCrash1.ts` tokenizes `while (d !== a$b);`, but
the parser requires a `{` after every `while` condition and reports
`expected LeftBrace, got Some(Semicolon)`.

The same reference also contains an expression-statement while body:

```ts
while ((c = a$b != a$b)) c.e;
```

Both forms are valid TypeScript/JavaScript statements and must parse before the
plain-JS narrowing/no-crash behavior can be triaged.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts
```

Representative source:

```ts
var a$b = {};
var c, d;
d = a$b;
while (d !== a$b);
while ((c = a$b != a$b)) c.e;
```

Compiler evidence:

```text
tokens: ok; While, LeftParen, Ident("d"), NotEqEq, Ident("a$b"), RightParen, Semicolon
ast: fails because parser expects LeftBrace after while condition
resolved: fails for the same parser boundary
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: expected LeftBrace, got Some(Semicolon)
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

The frontend accepts any currently representable statement body after a
`while` condition, including `;`, expression statements, and existing block
statements. The representative path should advance past the
`expected LeftBrace` diagnostic to the next semantic or runtime blocker.

## Scope

In scope:

- [ ] Parse `while (condition);` as a while loop with an empty-statement body.
- [ ] Parse `while (condition) expression;` as a while loop with an
  expression-statement body.
- [ ] Preserve current block-bodied `while (condition) { ... }` behavior.
- [ ] Add focused parser/frontend regression coverage for empty and expression
  while bodies.
- [ ] Re-run the representative TypeScript reference triage and record any
  later blocker.

Out of scope:

- `do ... while` ASI behavior, tracked by
  `issues/open/5210-parse-do-while-asi-before-block-end-or-expression.md`.
- General `for`/`if` non-block body support unless shared parser code already
  handles it as part of statement parsing.
- JavaScript control-flow narrowing semantics after parsing succeeds.
- Backend/runtime behavior for while loops beyond preserving existing emit
  behavior for block-bodied loops.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- focused parser/frontend tests

Do not touch:

- `scripts/run/reference-triage.py`
- backend/runtime ABI unless the existing while AST cannot represent
  non-block bodies

## Acceptance criteria

- [ ] `while (value);` parses without `expected LeftBrace, got Some(Semicolon)`.
- [ ] `while (value) value.prop;` parses as a while loop with an
  expression-statement body.
- [ ] Existing block-bodied while parser tests still pass.
- [ ] `narrowingPlainJsNoCrash1.ts` no longer reports the semicolon after
  `while (d !== a$b)` as the first unsupported syntax diagnostic.
- [ ] Any later assignment-expression, property-access, loop-lowering, or
  narrowing blocker is recorded here or split into a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend while
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingPlainJsNoCrash1.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Related but distinct:

- Issue 5210 owns `do ... while` ASI cases and does not cover ordinary
  `while` statement bodies.
- Issue 5154 owns angle-bracket type assertions in statement position and is
  not related to this parser boundary.
- Issue 059 is the broad parser epic and should not be selected directly when
  this narrow issue exists.

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
