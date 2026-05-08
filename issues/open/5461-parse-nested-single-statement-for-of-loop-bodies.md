---
id: 5461
title: "Parse nested single-statement for-of loop bodies"
type: feature
area: frontend/parser
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Allow `for..of` loops to use another loop or expression statement as an
unbraced single-statement body, covering nested loop shapes such as
`for (let a1 of []) for (let a2 of a1.someArray) doSomething(() => a2);`.

Split from generated bucket
`issues/done/3482-implement-nestedLoopWithOnlyInnerLetCaptured.md`.

## Problem

Problem: `nestedLoopWithOnlyInnerLetCaptured.ts` is currently blocked before
any closure-capture or TypeScript semantic behavior is reached. The parser
requires a `{ ... }` block after the outer `for..of` and rejects the nested
inner `for` body:

```text
UnsupportedSyntax: expected LeftBrace, got Some(For) at 96..99
```

TypeScript accepts the nested unbraced loop shape and then reports a later
semantic diagnostic for `a1.someArray` because `a1` is inferred as `never`.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts
```

Representative source:

```ts
declare let doSomething;

for (let a1 of [])
    for (let a2 of a1.someArray)
        doSomething(() => a2);
```

Compiler evidence:

```text
tokens: ok; outer For, inner For, call expression, arrow capture
ast: fails before AST construction
resolved: fails with the same parser diagnostic
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: expected LeftBrace, got Some(For) at 96..99
visible symbols: doSomething, a1
```

TypeScript oracle evidence:

```text
TypeScript AST: SourceFile -> ForOfStatement -> ForOfStatement
TypeScript diagnostic after parse: TS2339 for `someArray` on type `never`
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Desired final state

The parser accepts unbraced single-statement bodies for `for..of` when the body
is another loop or an expression statement, preserving existing block-bodied
loop behavior and allowing the representative to advance to its next semantic
blocker.

## Scope

In scope:

- [ ] Parse `for (let x of xs) for (let y of ys) expr;` as nested loop
  statements.
- [ ] Parse `for (let x of xs) call(x);` as a `for..of` with one expression
  statement body.
- [ ] Preserve braced `for..of` behavior and existing loop body parsing.
- [ ] Add focused parser coverage for nested `for..of` bodies and expression
  statement bodies.
- [ ] Re-run the representative triage and record any next blocker.

Out of scope:

- Closure environment lowering for captured `let` variables.
- TypeScript semantic diagnostics for `never.someArray`.
- Single-statement loop bodies already covered for `while break/continue` by
  `issues/done/5133-implement-single-statement-loop-body-break-continue.md`.
- Labeled statement, ASI, or arbitrary statement-body recovery beyond the
  focused loop/expression body forms.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- focused frontend/parser tests

Do not touch:

- resolver or closure-capture lowering unless the representative advances and
  proves a separate blocker
- backend/runtime code

## Acceptance criteria

- [ ] `nestedLoopWithOnlyInnerLetCaptured.ts` no longer reports
  `expected LeftBrace, got Some(For)` at the inner `for`.
- [ ] A focused parser test covers
  `for (let a1 of []) for (let a2 of a1.someArray) doSomething(() => a2);`.
- [ ] A focused parser test covers `for (let x of xs) call(x);`.
- [ ] Existing `while (true) break` / `while (true) continue` parser coverage
  from issue 5133 still passes.
- [ ] If parsing advances to a closure, resolver, or semantic blocker, this
  issue records that blocker before closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedLoopWithOnlyInnerLetCaptured.ts --detail --no-dashboard-data
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

This is a parser-only slice. The reference file name mentions captured `let`,
but current evidence shows the first blocker is loop-body parsing.

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
