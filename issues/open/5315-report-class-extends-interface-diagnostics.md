---
id: 5315
title: "Report class extends interface diagnostics"
type: feature
area: frontend/name-resolution
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report a TypeScript-compatible diagnostic when a class `extends` an interface
binding instead of implementing it.

## Problem

Problem: `classExtendsInterface.ts` now build-passes, but TypeScript reports
TS2689 for both `class A extends Comparable {}` and generic
`class A2<T> extends Comparable2<T> {}`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterface.ts
```

Observed 2026-05-07:

```text
ts2wasm: BuildPass
source:
interface Comparable {}
class A extends Comparable {}
class B implements Comparable {}

interface Comparable2<T> {}
class A2<T> extends Comparable2<T> {}
class B2<T> implements Comparable2<T> {}
TypeScript oracle:
TS2689 Cannot extend an interface 'Comparable'. Did you mean 'implements'?
TS2689 Cannot extend an interface 'Comparable2'. Did you mean 'implements'?
```

## Desired final state

The resolver/checker detects class heritage clauses whose base resolves to an
interface binding and reports a source-spanned TS2689-equivalent diagnostic at
the heritage name.

## Scope

In scope:

- [ ] Detect `class A extends Comparable {}` where `Comparable` is an interface.
- [ ] Detect generic `class A2<T> extends Comparable2<T> {}` after erasing type arguments.
- [ ] Preserve accepted `implements Comparable` / `implements Comparable2<T>` behavior.

Out of scope:

- Non-constructor value heritage diagnostics for local variables, tracked by issue 5314.
- Member-expression heritage diagnostics, tracked by issue 5256.
- Qualified class heritage implementation, tracked by issue 5225.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused class/name-resolution tests or fixtures

Do not touch:

- backend or runtime lowering unless a focused resolver test proves the diagnostic can only be produced later

## Acceptance criteria

- [ ] `classExtendsInterface.ts` no longer silently build-passes when TypeScript reports TS2689.
- [ ] A focused regression covers `interface I {} class C extends I {}`.
- [ ] A focused regression covers generic `interface I<T> {} class C<T> extends I<T> {}` or records a narrower follow-up if generic erasure blocks this shape.
- [ ] `class C implements I {}` remains accepted or reaches its existing implements-boundary owner.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(interface) or test(name)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterface.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterface.ts --detail --no-dashboard-data
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

Split from stale generated bucket `issues/done/1199-implement-classExtendsInterface-parser-syntax.md`.

Related but not duplicates:

- `issues/open/5314-report-non-constructor-local-class-heritage.md` handles
  non-constructor local value bindings used as heritage.
- `issues/done/5256-report-non-constructor-class-heritage-expressions.md`
  handles member-expression heritage diagnostics.

## Completion Evidence

Fill when implemented.
