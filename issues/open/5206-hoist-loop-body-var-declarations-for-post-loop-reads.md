---
id: 5206
title: "Hoist loop-body var declarations for post-loop reads"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P2
depends_on: [5006]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Resolve `var v` declared inside the `capturedLetConstInLoop3` loop body when
the same function later reads `use(v)`.

## Problem

Problem: loop-body `var v` is not registered in the enclosing function var
environment, so post-loop `use(v)` fails with `UnresolvedName`.

Both `capturedLetConstInLoop3` variants parse and produce AST successfully, but
resolver lookup fails at the post-loop `use(v)`.

Current compiler diagnostic:

```text
UnresolvedName: unresolved name: `v`
```

TypeScript resolves `v` and reports TS2454 use-before-assigned instead, so this
slice only needs to remove the false missing-symbol blocker.

## Current Failure

Reproductions:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop3.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop3_ES6.ts
```

Observed failures:

```text
capturedLetConstInLoop3.ts: UnresolvedName `v` at 233..234
capturedLetConstInLoop3_ES6.ts: UnresolvedName `v` at 232..233
```

Source shape:

```text
function foo0_1(x) {
    for (let x in []) {
        var v = x;
        (function() { return x + v });
        (() => x + v);
    }
    use(v);
}
```

AST evidence:

```text
Function foo0_1:
- ForOf/ForIn loop body contains var binding `v = x`
- Later function-body statement calls `use(v)`
- resolved dump fails during resolve_names
```

TypeScript oracle:

```text
TS2454: Variable 'v' is used before being assigned.
```

## Desired final state

The resolver registers `var` declarations from this loop-body pattern in the
nearest function var environment, while keeping `let x` loop shadowing lexical.

## Scope

In scope:

- [ ] Resolve the exact `capturedLetConstInLoop3` loop-body `var` hoisting
  pattern while preserving lexical `let x` loop shadowing.

Out of scope:

- General definite-assignment analysis.
- Other capturedLetConstInLoop parser-syntax buckets.
- Backend/runtime behavior unless this resolver fix exposes it in the focused
  fixture.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `fixtures/` or focused resolver/compiler tests

Do not touch:

- `crates/backend-wasm/`
- unrelated parser syntax handlers

## Acceptance criteria

- [ ] The two affected reference triage commands no longer report
  `UnresolvedName` for the post-loop `use(v)` read.
- [ ] A focused fixture proves loop-body `var v` is visible after the loop in
  the same function, with lexical `let x` shadowing preserved.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop3.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop3_ES6.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop3.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
