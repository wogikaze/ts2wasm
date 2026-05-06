---
id: 5314
title: "Report non-constructor local class heritage"
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

Report a TypeScript-compatible diagnostic when a class extends a local binding
whose value is not a constructor, starting with a namespace-local `var A = 1`.

## Problem

Problem: `classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts`
now build-passes, but TypeScript reports TS2507 for `class B extends A` because
the nearest `A` inside `namespace Foo` is `var A = 1`, not the outer class `A`
constructor.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts
```

Observed 2026-05-07:

```text
ts2wasm: BuildPass
source:
class A {
    a: number;
}
namespace A {
    export var v: string;
}

namespace Foo {
    var A = 1;
    class B extends A {
        b: string;
    }
}
TypeScript oracle: TS2507 Type 'number' is not a constructor function type.
```

## Desired final state

The resolver/checker resolves `extends A` inside `namespace Foo` to the
namespace-local variable `A`, detects that it is not constructor-valued, and
reports a source-spanned TS2507-equivalent diagnostic at the heritage name.

## Scope

In scope:

- [ ] Detect identifier heritage that resolves to a non-constructor local binding.
- [ ] Report a source-spanned diagnostic for `class B extends A` where `A` is `var A = 1`.
- [ ] Preserve supported `class B extends A` when `A` resolves to a class binding.

Out of scope:

- Strict property initialization diagnostics TS2564 for `a` and `b`.
- Member-expression heritage diagnostics such as `extends "".bogus`; issue 5256 owns that slice.
- Qualified class heritage names such as `extends Foo.Object`; issue 5225 owns that slice.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused class/name-resolution tests or fixtures

Do not touch:

- backend or runtime lowering unless a focused resolver test proves the diagnostic can only be produced later

## Acceptance criteria

- [ ] `classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts` no longer silently build-passes when TypeScript reports TS2507 for `extends A`.
- [ ] A focused regression covers `var A = 1; class B extends A {}` in a namespace or block scope.
- [ ] Existing `class A {} class B extends A {}` coverage remains green.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(name) or test(namespace)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsClauseClassMergedWithModuleNotReferingConstructor.ts --detail --no-dashboard-data
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

Split from stale generated bucket `issues/done/1197-implement-classExtendsClauseClassMergedWithModuleNotReferingConstructor.md`.
Also owns the matching first residual semantic gap folded from
`issues/done/1198-implement-classExtendsClauseClassNotReferringConstructor.md`.

Related but not duplicates:

- `issues/open/5256-report-non-constructor-class-heritage-expressions.md`
  handles member-expression heritage diagnostics.
- `issues/open/5225-support-qualified-class-heritage-names.md` handles
  qualified class heritage implementation.

## Completion Evidence

Fill when implemented.
