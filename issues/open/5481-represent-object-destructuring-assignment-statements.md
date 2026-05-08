---
id: 5481
title: "Represent object destructuring assignment statements"
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

Represent parenthesized object destructuring assignment statements such as
`({ x } = this);` as destructuring assignments instead of synthesizing an
ordinary assignment target named `"{x}"`.

## Problem

`noUnusedLocals_destructuringAssignment.ts` tokenizes and builds an AST, but
the class method body contains:

```text
Assign { name: "{x}", expr: This }
```

Name resolution then fails with an unspanned unresolved synthetic name.

Problem: object destructuring assignment statements are parsed into a string
assignment target like `"{x}"`, producing `UnresolvedName` instead of assigning
to the existing local binding.

## Current failure

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts --detail --no-dashboard-data
```

Result on 2026-05-08:

```text
reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts: UnresolvedName: name-resolution
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts
```

Representative source:

```ts
class C {
    private x = 0;

    m(): number {
        let x: number;
        ({ x } = this);
        return x;
    }
}

```

Compiler evidence:

```text
tokens: ok through parenthesized object destructuring assignments
ast: ok but `({ x } = this)` becomes Assign { name: "{x}", expr: This }
resolved/lowered: UnresolvedName unresolved name: `{x}`
```

TypeScript oracle:

```text
diagnostics=[]
local binding x has type number
```

## Desired final state

The frontend represents object destructuring assignment statements as
destructuring assignment nodes or lowers this narrow shorthand form to ordinary
assignments to existing bindings. The representative fixture should no longer
produce an unresolved synthetic name such as `"{x}"`.

## Scope

In scope:

- [ ] Detect parenthesized object destructuring assignment statements in method
  and function bodies.
- [ ] Preserve the local binding target for shorthand properties such as
  `({ x } = this);`.
- [ ] Avoid synthesizing assignment names from the pattern text.
- [ ] Add focused coverage for `let x; ({ x } = obj);`.

Out of scope:

- Destructuring assignment used as a call argument; see issue 5224.
- Concise arrow body destructuring assignments; see issue 5208.
- Full object rest/default/computed-property destructuring assignment
  semantics.
- noUnusedLocals diagnostic parity after the destructuring assignment is
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

- [ ] `noUnusedLocals_destructuringAssignment.ts` no longer reports
  `UnresolvedName` for synthetic name `"{x}"`.
- [ ] A focused regression covers `let x; ({ x } = obj);`.
- [ ] Existing destructuring binding declarations continue to pass.
- [ ] Any remaining unsupported destructuring assignment behavior reports a
  source-spanned destructuring-specific diagnostic, not an unspanned
  unresolved synthetic name.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(destructuring) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts --detail --no-dashboard-data
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
`issues/done/3571-implement-noUnusedLocals-destructuring.md` after fresh
triage on 2026-05-08.

Related but not duplicates:

- `issues/done/252-implement-destructuring-assignment-pattern-parser.md`
  completed broad parser support but left runtime/semantic destructuring
  assignment behavior out of scope.
- `issues/open/5224-parse-destructuring-assignment-call-arguments.md` owns
  parenthesized destructuring assignments in call-argument position.
- `issues/open/5208-parse-arrow-body-destructuring-assignments.md` owns
  concise arrow body destructuring assignments.

## Completion evidence

Fill when implemented.
