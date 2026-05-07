---
id: 5406
title: "Report ambient enum nonconstant initializers"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a source diagnostic for ambient enum members whose initializer is not a
constant expression, matching the TypeScript TS1066 boundary exposed by
`ambientEnum1.ts`.

## Problem

Problem: `ambientEnum1.ts` currently records a ts2wasm `BuildPass`, but
TypeScript reports TS1066 for `declare enum E2 { x = 'foo'.length }`.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientEnum1.ts
```

```text
ts2wasm: BuildPass
oracle: TS1066 In ambient enum declarations member initializer must be constant expression.
source: declare enum E2 { x = 'foo'.length }
```

## Desired final state

The compiler reports a source-spanned diagnostic for non-constant ambient enum
member initializers instead of silently treating the reference as a build pass.

## Scope

In scope:

- [ ] Detect property/member expressions in ambient enum member initializers.
- [ ] Report the non-constant initializer at the initializer span.
- [ ] Preserve erasure for valid ambient enum declarations.

Out of scope:

- Runtime enum lowering.
- Full TypeScript constant-expression evaluation.
- Non-ambient enum semantics.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused parser or resolver tests

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`
- runtime enum lowering outside the diagnostic boundary

## Acceptance criteria

- [ ] `ambientEnum1.ts` no longer records `BuildPass` when TS1066 applies.
- [ ] A focused regression covers `declare enum E { x = 'foo'.length }`.
- [ ] `declare enum E { x = 1 }` remains erased without runtime bindings.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(ambient) or test(enum)'
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientEnum1.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientEnum1.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs: not affected.
Current state: not affected.
Follow-up issues: none.

## Notes

Split from `issues/done/145-implement-ambientEnum.md` on 2026-05-08 after the
build blocker was resolved by issue 400.

## Completion evidence

Fill only when implemented.
