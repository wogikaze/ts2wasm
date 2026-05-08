---
id: 5359
title: "Report multiple constructor implementation diagnostics"
type: feature
area: frontend/diagnostics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report a TS2392-equivalent diagnostic for classes that contain more than one
constructor implementation body, instead of a spanless generic
`DuplicateFunction: duplicate constructor definition`.

## Problem

`constructorOverloads1.ts` and `constructorOverloads8.ts` contain invalid
classes with multiple constructor bodies. The compiler currently reports a
generic duplicate-constructor diagnostic without a source span, while
TypeScript reports TS2392 at each constructor declaration.

Problem: invalid multiple constructor implementations are not reported with
the source-spanned constructor diagnostic expected by the TypeScript oracle.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads8.ts
```

Current compiler diagnostic:

```text
DuplicateFunction: duplicate constructor definition
```

TypeScript oracle evidence:

```text
TS2392: Multiple constructor implementations are not allowed.
```

Representative source:

```ts
class C {
    constructor(x) { }
    constructor(y, x) { }
}
```

## Desired final state

Class validation distinguishes bodyless constructor signatures from constructor
implementations with bodies, and reports TS2392-equivalent diagnostics for
multiple implementation bodies with source spans.

## Scope

In scope:

- [ ] Detect classes with more than one constructor implementation body.
- [ ] Emit a source-spanned TS2392-equivalent diagnostic at constructor declarations.
- [ ] Keep valid bodyless constructor overload signatures delegated to issue 5334.

Out of scope:

- Accepting valid constructor overload signatures, tracked by issue 5334.
- Constructor overload resolution or call argument checking.
- Runtime constructor dispatch changes.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/ir/src/semantic.rs`
- focused frontend/IR diagnostic tests

Do not touch:

- `crates/backend-wasm/`
- broad overload resolution logic

## Acceptance criteria

- [ ] `constructorOverloads1.ts` reports a source-spanned multiple-constructor diagnostic instead of spanless `DuplicateFunction`.
- [ ] `constructorOverloads8.ts` reports a source-spanned multiple-constructor diagnostic for class `C`.
- [ ] A focused negative test covers `class C { constructor() {} constructor(x) {} }`.
- [ ] A valid overload shape remains owned by issue 5334 and is not accepted in this slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(constructor) or test(duplicate)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads8.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads --detail --no-dashboard-data
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

Split from `issues/done/1475-implement-constructorOverloads-parser-syntax.md`
on 2026-05-07.

Related:

- `issues/done/5334-parse-class-constructor-overload-signatures.md` owns valid
  bodyless constructor overload signatures. This issue owns only invalid
  multiple implementation bodies.

## Completion evidence

Fill only when implemented.
