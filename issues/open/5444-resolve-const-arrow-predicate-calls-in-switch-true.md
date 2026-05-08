---
id: 5444
title: "Resolve const arrow predicate calls in switch true clauses"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Resolve direct calls to top-level `const` bindings initialized with arrow
functions when they appear as `switch (true)` case expressions, such as
`case isA(x):`.

Split from generated bucket
`issues/done/3438-implement-narrowByClauseExpressionInSwitchTrue-name-resolution.md`.

## Problem

`narrowByClauseExpressionInSwitchTrue1.ts` parses `const isA = (...) => ...`
and `case isA(x):`, and visible-symbol extraction sees `isA`, but the compiler
later reports `UnresolvedFunction` for the direct call.

Problem: const arrow predicate bindings are visible as bindings, but direct
calls to those bindings in switch case expressions are not resolved as callable
local arrow functions.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue1.ts
```

Current diagnostic:

```text
UnresolvedFunction: unresolved function: `isA`
```

Focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue1.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedFunction:1
unsupported_features=function-resolution:1
semantic_enabled=0
```

Source context:

```ts
const isA = (x: AorB): x is A => x.type === "A";
const isB = (x: AorB): x is B => x.type === "B";

function test1(x: AorB) {
  switch (true) {
    case isA(x):
      x;
      break;
    case isB(x):
      x;
      break;
  }
}
```

Compiler evidence:

```text
tokens: ok through const arrow predicates and switch case call expressions
ast: ok; `case isA(x)` and `case isB(x)` are Call expressions
resolved/lowered: reports UnresolvedFunction for `isA`
visible symbols before failure: binding isA, binding isB, function test1, function test2, binding x, function isSomeType, function processInput
```

TypeScript oracle:

```text
ok: true
diagnostics: []
isA type: (x: AorB) => x is A
isB type: (x: AorB) => x is B
```

## Desired final state

The compiler resolves direct calls to local const arrow predicate bindings in
switch case expressions, so `narrowByClauseExpressionInSwitchTrue1.ts` no
longer stops at `UnresolvedFunction: isA`.

## Scope

In scope:

- [ ] Resolve direct calls to const bindings initialized with arrow functions
      when used as switch case expressions.
- [ ] Preserve existing support for assigned-arrow recursion and ordinary
      arrow binding calls.
- [ ] Add a focused resolver/lowering regression for
      `const pred = x => x === 1; switch (true) { case pred(v): break; }`.
- [ ] Re-triage the representative reference path and record the next blocker
      if it advances to narrowing/type-predicate semantics.

Out of scope:

- Full TypeScript type predicate narrowing semantics.
- Arbitrary function-valued local calls or extracted methods.
- Ambient callable const declarations.
- Parser support for the sibling `narrowByClauseExpressionInSwitchTrue`
  parser-syntax bucket.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused resolver/lowering tests or fixtures

Do not touch:

- backend/runtime ABI unless fresh implementation evidence proves it is needed

## Acceptance criteria

- [ ] `narrowByClauseExpressionInSwitchTrue1.ts` no longer reports
      `UnresolvedFunction: unresolved function: \`isA\``.
- [ ] A focused regression covers a const arrow binding called from a
      `switch (true)` case expression.
- [ ] Existing assigned-arrow recursion and function-valued local diagnostics
      remain unchanged.
- [ ] Any later semantic narrowing/type-predicate blocker is recorded here or
      split into a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(arrow) or test(function) or test(call) or test(switch)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue1.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue1.ts --detail --no-dashboard-data
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

Related but distinct issues:

- `issues/done/283-implement-assigned-arrow-recursion.md` records completed
  assigned-arrow recursion support for a narrower recursive closure fixture.
- `issues/open/5440-support-initialized-function-expression-local-calls.md`
  owns initialized `function` expression local calls, not arrow predicates.

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
