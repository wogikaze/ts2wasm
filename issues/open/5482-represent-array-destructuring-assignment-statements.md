---
id: 5482
title: "Represent array destructuring assignment statements"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Represent parenthesized array destructuring assignment statements such as
`([x] = [1]);` as destructuring assignments instead of synthesizing an ordinary
assignment target named `"[x]"`.

## Problem

`noUnusedLocals_writeOnly.ts` tokenizes and builds an AST, but the first
destructuring assignment in `function f` contains:

```text
Assign { name: "[x]", expr: Array([1]) }
```

Problem: array destructuring assignment statements are parsed into a string
assignment target like `"[x]"`, producing `UnresolvedName` instead of assigning
to the existing parameter binding.

## Current failure

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts --detail --no-dashboard-data
```

Result on 2026-05-08:

```text
reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts: UnresolvedName: name-resolution
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts
```

Representative source:

```ts
function f(x = 0) {
    x = 1;
    ([x] = [1]);
}
```

Compiler evidence:

```text
tokens: ok through parenthesized array destructuring assignment
ast: ok but `([x] = [1])` becomes Assign { name: "[x]", expr: Array([1]) }
resolved: UnresolvedName unresolved name: `[x]` at 214..224
```

TypeScript oracle:

```text
diagnostics=[]
parameter x has type number
```

## Desired final state

The frontend represents array destructuring assignment statements as
destructuring assignment nodes or lowers this narrow single-name form to an
ordinary assignment to the existing binding. The representative fixture should
no longer produce unresolved synthetic name `"[x]"`.

## Scope

In scope:

- [ ] Detect parenthesized array destructuring assignment statements in
  function bodies.
- [ ] Preserve the local binding target for single-element patterns such as
  `([x] = [1]);`.
- [ ] Add focused coverage for `let x; ([x] = arr);`.

Out of scope:

- Object destructuring assignment statements; see issue 5481.
- Nested array/object/default destructuring assignment semantics.
- noUnusedLocals write-only semantic diagnostics after the assignment is
  represented correctly.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/ir/src/`
- focused parser or CLI tests

Do not touch:

- backend/runtime ABI unless a later lowering boundary proves necessary
- unrelated destructuring binding declarations

## Acceptance criteria

- [ ] `noUnusedLocals_writeOnly.ts` no longer reports `UnresolvedName` for
  synthetic name `"[x]"`.
- [ ] A focused regression covers `let x; ([x] = arr);`.
- [ ] Existing destructuring binding declarations continue to pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(destructuring) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnly.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from generated bucket
`issues/done/3573-implement-noUnusedLocals-name-resolution.md` after fresh
triage on 2026-05-08.

Related but not duplicates:

- `issues/open/5481-represent-object-destructuring-assignment-statements.md`
  owns object destructuring assignment statements such as `({ x } = this);`.
- `issues/open/5224-handle-package-json-virtual-sections-in-multifile-references.md` owns
  parenthesized destructuring assignments in call-argument position.
- `issues/open/5208-support-regexp-match-fallback-array-map-receiver.md` owns
  concise arrow body destructuring assignments.

## Completion evidence

Fill when implemented.
