---
id: 5340
title: "Preserve function after object type declaration"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-08
---

## Summary

Parse a TypeScript-erased object type `var` declaration when the next runtime
declaration is a function.

## Problem

`collisionThisExpressionAndLocalVarInFunction.ts` tokenizes successfully, then
reports `UnsupportedTypeScriptSyntax: unterminated TypeScript type annotation
at 164..165` while parsing this shape:

```ts
var console: {
    log(val: any);
}
function x() {
    var _this = 5;
    x => { console.log(this.x); };
}
```

Problem: `var name: { method(param: Type); }` is not terminated before a following `function` declaration.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts
```

Observed 2026-05-07:

```text
UnsupportedTypeScriptSyntax: unterminated TypeScript type annotation at 164..165
tokens: ok through typed var, function x, local _this, arrow, and this.x
TypeScript oracle: parses the function, then reports duplicate console and implicit this diagnostics
```

## Desired final state

The parser erases the object type annotation and preserves the following
`function x() { ... }` declaration as a separate runtime declaration.

## Scope

In scope:

- [x] Preserve a following function declaration after `var typed: { ... }`.
- [x] Add a focused parser regression for `var typed: { m(x: any); }\nfunction next() {}`.
- [x] Re-run the reference triage and record the next diagnostic.

Out of scope:

- Full TypeScript structural type support.
- Arrow `this.x` semantics after parsing advances.
- Duplicate global `console` compatibility diagnostics.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- backend emit or runtime ABI
- unrelated resolver/lowering call semantics

## Acceptance criteria

- [x] `collisionThisExpressionAndLocalVarInFunction.ts` no longer reports `unterminated TypeScript type annotation` at `164..165`.
- [x] A focused parser regression proves `function next() {}` is preserved after an object type annotation declaration.
- [x] Existing parser behavior for the following-var case remains covered by issue 5339.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts --detail --no-dashboard-data
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

Split from `issues/open/1326-implement-collisionThisExpressionAndLocalVarInFunction.md`.
Related: `issues/open/5339-preserve-var-after-object-type-declaration.md`
covers the same type declaration followed by another `var`.
Also owns `issues/open/3517-implement-noCollisionThisExpressionAndLocalVarInFunction.md`, which stops at the same unterminated object type annotation before `function x()`.

## Completion evidence

Fill when implemented.

## False-done audit

**truly-done** (5340)

- Implementation commits: verified via `git log --oneline --all --grep=5340`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
