---
id: 5158
title: "Report interface private member clashes"
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

`baseTypePrivateMemberClash.ts` defines two classes with private property `m` from different class origins, then declares `interface Z extends X, Y`. TypeScript reports TS2320 because the inherited private members are not identical, but the compiler erases the interface and private typed fields, then falls through to backend WAT generation.

## Problem

Problem: `reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts` currently reports `BackendIo` instead of a source-spanned diagnostic for an interface extending classes with incompatible private members.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts
```

Current compiler diagnostic:

```text
BackendIo: wat2wasm failed
```

Representative source:

```ts
class X {
    private m: number;
}
class Y {
    private m: string;
}

interface Z extends X, Y { }
```

Current compiler evidence:

- Tokens succeed and include `private m: number`, `private m: string`, and `interface Z extends X, Y`.
- AST succeeds but contains only empty `ClassDecl` entries for `X` and `Y`; the interface is erased.
- Resolved IR succeeds with empty classes and no private fields.
- Backend WAT validation fails before any frontend diagnostic is reported.

TypeScript oracle evidence:

```text
TS2320: Interface 'Z' cannot simultaneously extend types 'X' and 'Y'.
  Named property 'm' of types 'X' and 'Y' are not identical.
```

The oracle also reports TS2564 definite-assignment diagnostics for both private properties; those broader property-initialization checks are out of scope for this slice.

## Desired final state

The frontend reports a source-spanned diagnostic for the `interface Z extends X, Y` private-member clash before backend emission. The representative case should no longer reach `BackendIo`.

## Scope

In scope:

- [x] Track enough erased TypeScript class/interface metadata to detect `interface ... extends A, B` when `A` and `B` declare same-named private members from different classes.
- [x] Report a source-spanned diagnostic at the interface declaration or offending heritage name.
- [x] Add a focused regression for `class X { private m: number } class Y { private m: string } interface Z extends X, Y {}`.
- [x] Re-run representative triage and confirm it no longer reports `BackendIo`.

Out of scope:

- General TypeScript structural type checking.
- TS2564 definite-assignment diagnostics.
- Runtime private class field semantics; runtime private elements are tracked by issue 255/351 families.
- The shared `$exception_pending` runtime-link bug, tracked by issue 5155.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/frontend/src/diagnostic.rs`

Do not touch:

- `crates/backend-wasm/src/` unless triage still reaches backend after the frontend diagnostic is added.

## Acceptance criteria

- [x] The representative `interface Z extends X, Y {}` case reports a source-spanned frontend/resolver diagnostic for incompatible private member `m`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts` no longer reports `BackendIo`.
- [x] Private member metadata from erased TypeScript declarations is covered by a focused parser/resolver regression.
- [x] Definite-assignment diagnostics remain out of scope and are not required to close this issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

This issue is intentionally narrower than full type checking: it only needs enough erased TypeScript metadata to avoid compiling a known invalid private-member interface heritage shape as runtime code.

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


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/open/. Implementation commits confirmed.
