---
id: 5316
title: "Report class implements interface private member mismatch"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report a TypeScript-compatible diagnostic when a class `implements` an
interface that inherited a private member from a different class origin.

## Problem

Problem: `classExtendsInterfaceThatExtendsClassWithPrivates1.ts` now
build-passes, but TypeScript reports TS2420 because `D2` declares its own
private `x` instead of satisfying the private `x` inherited by `I extends C`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts
```

Observed 2026-05-07:

```text
ts2wasm: BuildPass
TypeScript oracle:
TS2420 Class 'D2' incorrectly implements interface 'I'.
  Types have separate declarations of a private property 'x'.
```

Representative source:

```ts
class C {
    public foo(x: any) { return x; }
    private x = 1;
}

interface I extends C {
    other(x: any): any;
}

class D2 implements I {
    public foo(x: any) { return x }
    private x = 3;
    other(x: any) { return x }
}
```

Current compiler evidence:

- Tokens include the erased `interface I extends C` and both `private x`
  declarations.
- AST/resolved IR only retain runtime class methods for `C` and `D2`; the
  interface heritage and typed private member metadata are erased.
- The build succeeds, so semantic parity currently depends on a missing
  frontend/resolver diagnostic.

## Desired final state

The frontend preserves enough type-only class/interface metadata to diagnose the
`D2 implements I` private-member-origin mismatch before reporting a build pass.

## Scope

In scope:

- [ ] Track private member origin metadata for TypeScript `private` class
  properties erased from runtime class bodies.
- [ ] Diagnose `class D2 implements I` when `D2` redeclares inherited private
  member `x` from a different class origin.
- [ ] Add a focused regression for the representative `class C` / `interface I`
  / `class D2 implements I` shape.

Out of scope:

- General structural interface implementation checking.
- Interface multiple-base private-member clashes, tracked by issue 5158.
- Class `extends` interface diagnostics, tracked by issue 5315.
- TS2564 definite-assignment diagnostics for uninitialized properties.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/statements_class.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/frontend/src/diagnostic.rs`
- focused frontend/IR tests or fixtures

Do not touch:

- backend/runtime lowering unless triage still reaches backend after the
  frontend diagnostic is added

## Acceptance criteria

- [ ] The representative
  `classExtendsInterfaceThatExtendsClassWithPrivates1.ts` case no longer
  silently build-passes when TypeScript reports TS2420.
- [ ] The diagnostic points at `class D2 implements I` or the offending
  `implements I` heritage span and names private property `x`.
- [ ] Compatible `implements` clauses without private-member-origin mismatch
  remain accepted or continue to be owned by their existing narrower issues.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -p ts2wasm-ir -E 'test(class) or test(interface) or test(name)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts --detail --no-dashboard-data
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

Split from stale generated bucket
`issues/open/1203-implement-classExtendsInterfaceThatExtendsClassWithPrivates.md`.

Related but not duplicates:

- `issues/open/5158-report-interface-private-member-clash.md` handles
  `interface Z extends X, Y` private-member clashes.
- `issues/open/5315-report-class-extends-interface-diagnostics.md` handles
  class `extends` interface diagnostics.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
