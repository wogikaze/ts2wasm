---
id: 5394
title: "Fix object accessor FuncId invariant"
type: feature
area: ir/compiler
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Fix the lowered-IR/WAT validation path that reports an out-of-range `FuncId`
for object-literal get/set accessors in a declaration-only JS reference file.

## Problem

Current diagnostic: `InvariantViolation: FuncId 0 is out of range (program has
0 function(s))` while triaging `accessorDeclarationEmitJs.ts`.

Fresh coverage on 2026-05-08 reports `build_pass=1`, so the old parser
`expected Colon` blocker is gone. `reference-triage` still exposes an invariant
when it dumps the build pipeline for exported object literals with accessors.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts
```

Equivalent repo task:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts
```

Representative source:

```ts
export const t1 = {
    p: 'value',
    get getter() { return 'value'; },
}
```

Observed evidence:

```text
tokens: ok
ast: ok; object literal getter/setter accessors parse as FunctionExpr entries
resolved: ok; exported t1/t2/t3 object literals are present
coverage: build_pass=1, unsupported=0
triage: InvariantViolation FuncId 0 is out of range (program has 0 function(s))
oracle: TypeScript ok=true with no diagnostics
```

## Desired final state

The triage/build pipeline no longer emits invalid lowered IR for declaration-only
object-literal accessor functions when the lowered program has zero functions.

## Scope

In scope:

- [ ] Preserve or erase object accessor function references consistently when no functions are emitted.
- [ ] Add focused validation coverage for exported object literal get/set accessors in a zero-function program.
- [ ] Confirm the representative no longer reports the `FuncId 0` invariant.

Out of scope:

- General declaration emit output.
- Class constructor `FuncId` invariants, tracked by issues 5247 and 5325.
- Runtime semantics for invoking object accessors.

## Affected paths

Expected:

- `crates/ir/`
- `crates/compiler/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`

Do not touch:

- frontend parser syntax unless a focused regression proves AST production changed

## Acceptance criteria

- [ ] `accessorDeclarationEmitJs.ts` triage no longer reports `FuncId 0 is out of range`.
- [ ] Focused coverage remains `build_pass=1` or advances to a non-invariant diagnostic.
- [ ] Existing object-literal accessor parser coverage remains unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(accessor) or test(object) or test(invariant)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/accessorDeclarationEmitJs.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Notes

Split from generated bucket `566` on 2026-05-08. Related class-constructor
`FuncId` invariants are already tracked separately by issues 5247 and 5325.
