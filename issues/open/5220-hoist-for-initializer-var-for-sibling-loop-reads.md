---
id: 5220
title: "Hoist for-initializer var declarations for sibling loop reads"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Resolve `var k` declared in one `for` initializer when a later sibling `for`
initializer in the same function reads and assigns `k`.

## Problem

Problem: `cf.ts` parses control-flow syntax successfully, but name resolution
does not register `var k` from `for (var k = 0; ...)` in the enclosing function
var environment. The later sibling loop `for (k = 0; ...)` then fails with
`UnresolvedName`.

Current diagnostic:

```text
UnresolvedName: unresolved name: `k` at 0..0
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```ts
for (var k=0;k<10;k++) {
    z;
    break;
}
for (k=0;k<10;k++) {
    if (k==6) {
        continue;
    }
    break;
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; first For initializer is Let k, second For initializer reads/assigns k
resolved: UnresolvedName for k during resolve_names
TypeScript oracle: ok, diagnostics: []; binding k has type number at line 48
```

## Desired final state

The resolver registers `var` declarations from `for` initializers in the nearest
function var environment so later sibling statements in the same function can
read the binding.

## Scope

In scope:

- [ ] Resolver: hoist `var` declarations from `for (var k = ...)` initializers to the enclosing function var environment.
- [ ] Resolver: preserve lookup of `k` inside the declaring loop condition, update, and body.
- [ ] Tests: add a focused fixture for `for (var k = 0; ...); for (k = 0; ...)`.
- [ ] Diagnostics: preserve duplicate-local behavior for true same-scope `let` / `const` duplicates.

Out of scope:

- Loop-body `var` hoisting already tracked by issue 5206.
- Full unreachable-code analysis for `cf.ts`.
- Labelled break/continue semantics beyond whatever triage exposes after this resolver fix.
- Runtime/backend changes unless the focused fixture advances past name resolution.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- focused resolver/compiler tests
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- parser syntax handlers
- broad control-flow lowering

## Acceptance criteria

- [ ] `cf.ts` no longer reports `UnresolvedName` for sibling-loop `k`.
- [ ] A focused fixture proves `for (var k = 0; ...); for (k = 0; ...)` resolves `k` in the same function var scope.
- [ ] Existing loop-local and duplicate-local regression tests still pass.
- [ ] `var k` remains visible in the declaring loop condition, update, and body.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir name_resolver
cargo nextest run -p ts2wasm-cli -E 'test(loop) or test(duplicate)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cf.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cf.ts --detail
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Notes

Split from generated bucket `issues/done/1124-implement-cf.md`. Related issue
`issues/open/5206-hoist-loop-body-var-declarations-for-post-loop-reads.md`
tracks loop-body `var` hoisting; this issue is limited to `for` initializer
bindings and sibling-loop reads.

## Completion evidence

Fill when implemented.
