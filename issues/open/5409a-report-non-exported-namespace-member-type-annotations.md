---
id: 5409a
title: "Report non-exported namespace member type annotations"
type: bug
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TS2694-like diagnostic when a qualified type annotation references a
non-exported member of a namespace.

This issue was split from `issues/open/3328-implement-moduleClassArrayCodeGenTest.md`.

## Problem

`moduleClassArrayCodeGenTest.ts` now build-passes, but TypeScript reports
`TS2694: Namespace 'M' has no exported member 'B'` for `var t2: M.B[] = []`.
The compiler erases the annotation and emits no semantic diagnostic.

Problem: non-exported namespace members in qualified type annotations are erased
without a TS2694-like diagnostic.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleClassArrayCodeGenTest.ts
```

Current evidence:

```text
compiler: BuildPass
tokens: namespace M, export class A, class B, var t: M.A[], var t2: M.B[]
ast/resolved: only runtime `t = []` and `t2 = []` remain
TypeScript oracle: TS2694 Namespace 'M' has no exported member 'B'
```

## Desired final state

The frontend records enough namespace member export metadata during type
annotation erasure to report a source-spanned diagnostic for `M.B` when `B` is
not exported from namespace `M`.

## Scope

In scope:

- [ ] Detect qualified type annotations rooted at a namespace declaration.
- [ ] Distinguish exported namespace members from non-exported namespace locals.
- [ ] Report a TS2694-like diagnostic for `M.B` in `var t2: M.B[] = []`.
- [ ] Preserve successful erasure/build behavior for exported `M.A[]`.

Out of scope:

- Runtime namespace member lowering.
- Class heritage qualified-name checks, tracked by issue 5313.
- Import-equals alias missing-member diagnostics, tracked by issue 5397.
- Full TypeScript namespace checker parity.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/compiler/src/`
- focused frontend/compiler tests or fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `moduleClassArrayCodeGenTest.ts` no longer silently build-passes when `M.B[]` refers to non-exported `B`.
- [ ] The diagnostic is source-spanned at `B` in `M.B`.
- [ ] A focused regression covers accepted exported member `M.A[]` and rejected non-exported member `M.B[]`.
- [ ] The existing issue 5313 and 5397 owner scopes remain unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -p ts2wasm-compiler
env TS2WASM_BINARY=target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleClassArrayCodeGenTest.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleClassArrayCodeGenTest --detail --no-dashboard-data
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

Related but not duplicates:

- `issues/open/5313-report-non-exported-namespace-member-qualified-heritage.md`
  covers class heritage clauses.
- `issues/open/5397-report-missing-namespace-alias-member-diagnostic.md` covers
  import-equals aliases.

Also owns `issues/open/3434-implement-namespacesDeclaration.md`: fresh triage
shows `namespacesDeclaration2.ts` build-passes while TypeScript reports TS2694
for qualified type annotations `N.S`, `M.F`, and `ns.A`. The first two refer
to non-exported namespace functions; the ambient `ns.A` case is the same
qualified namespace-member annotation diagnostic shape with an absent member.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
