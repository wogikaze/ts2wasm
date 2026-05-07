---
id: 5411
title: "Report TS2709 for namespace variable annotation"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TS2709-style diagnostic for the representative `var a: A;` case where
`A` is a same-file namespace.

## Problem

`moduleAssignmentCompat1.ts` currently build-passes after erasing namespace
declarations and type annotations:

```ts
namespace A {
    export class C { }
}
var a: A;
```

TypeScript reports `TS2709: Cannot use namespace 'A' as a type.` at the `A`
annotation.

Problem: `var a: A;` silently build-passes when `A` is a namespace.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAssignmentCompat1.ts
```

Observed result:

```text
BuildPass: ts2wasm build succeeded
oracle: TS2709 Cannot use namespace 'A' as a type.
```

Compiler evidence:

```text
tokens: ok through namespace A and var a: A
ast/resolved: Let a is retained, namespace A and annotation A are erased
```

## Desired final state

The representative file reports a source-spanned TS2709-style diagnostic at the
namespace identifier used in `var a: A;`.

## Scope

In scope:

- [ ] Detect a same-file namespace root used as a variable type annotation.
- [ ] Report `Cannot use namespace 'A' as a type` for `var a: A;`.
- [ ] Add one focused regression for this exact shape.

Out of scope:

- Class/interface heritage diagnostics, tracked by issue 5410.
- Qualified namespace member annotation diagnostics.
- Assignment compatibility after the diagnostic advances.
- Exhaustively validating the other `moduleAssignmentCompat` variants.

## Affected paths

Expected:

- `crates/frontend/src/`
- focused frontend/resolver tests or fixtures

Do not touch:

- backend namespace emit
- module/package resolution

## Acceptance criteria

- [ ] `moduleAssignmentCompat1.ts` no longer reports `BuildPass`; it reports TS2709-style `Cannot use namespace 'A' as a type`.
- [ ] A focused test covers `namespace A { export class C {} } var a: A;`.
- [ ] Non-namespace type annotations still erase/build as before.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(annotation)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAssignmentCompat1.ts
```

Impacted commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/done/3307-implement-moduleAssignmentCompat.md`.

Sibling cases `moduleAssignmentCompat2.ts` through
`moduleAssignmentCompat4.ts` show the same TS2709 oracle shape and should be
rechecked after this representative diagnostic lands.

## Completion evidence

Fill when implemented.
