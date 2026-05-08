---
id: 5437
title: "Report typed class method null return diagnostics"
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

Report a TypeScript-compatible diagnostic when a typed class method returns
`null` under strict null checking.

## Problem

Problem: `mutuallyRecursiveGenericBaseTypes2.ts` now build-passes, but
TypeScript reports TS2322 because `bar(): foo2<T[]>` returns `null`.

Representative source:

```ts
class foo<T> {
    bar(): foo2<T[]> { return null; }
}

class foo2<T> extends foo<T> {
}
```

Current evidence from 2026-05-08:

```text
ts2wasm: build_pass
TypeScript oracle:
TS2322 Type 'null' is not assignable to type 'foo2<T[]>'.
```

## Desired final state

The frontend or semantic checker preserves enough class method return type
information to reject `return null` when the annotated return type is a
non-nullable class or generic class instantiation.

## Scope

In scope:

- [ ] Detect `return null` inside class methods with explicit non-nullable
  return type annotations.
- [ ] Include generic class return types such as `foo2<T[]>`.
- [ ] Add focused coverage for `class C { m(): D { return null; } }`.

Out of scope:

- Getter return annotations, tracked by issue 5183.
- Full TypeScript generic assignability beyond the explicit `null` return case.
- Runtime behavior for methods that pass semantic validation.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused semantic/parser tests

Do not touch:

- backend/runtime ABI
- unrelated class inheritance lowering

## Acceptance criteria

- [ ] `mutuallyRecursiveGenericBaseTypes2.ts` no longer silently build-passes
  when TypeScript reports TS2322 at `return null`.
- [ ] A focused fixture covers `class C { m(): D { return null; } }`.
- [ ] Unannotated methods that return `null` remain accepted.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend class
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveGenericBaseTypes2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveGenericBaseTypes2.ts --detail --no-dashboard-data
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
`issues/open/3418-implement-mutuallyRecursiveGenericBaseTypes.md`.

Related but not duplicates:

- `issues/open/5183-report-typed-getter-null-return-diagnostics.md` owns typed
  getter `return null` diagnostics, not ordinary class methods.

## Completion evidence

Fill when implemented.
