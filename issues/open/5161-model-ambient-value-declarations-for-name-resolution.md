---
id: 5161
title: "Model ambient value declarations for name resolution"
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

The parser erases declaration-only ambient variables such as `declare var e: Ellement;`, `declare var b2: boolean;`, and `declare let anys: Ari<any>;`, but the name resolver then rejects later runtime expressions that reference those names. This blocks references before the compiler can reach later contextual typing, assignment, or array filter diagnostics.

## Problem

Problem: `reference/typescript/tests/cases/compiler/bestCommonTypeWithContextualTyping.ts` currently reports `UnresolvedName` for `e` in `[e]`, even though `declare var e: Ellement;` is visible earlier in the source.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestCommonTypeWithContextualTyping.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/booleanAssignment.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/booleanFilterAnyArray.ts
```

Current compiler diagnostic:

```text
UnresolvedName: unresolved name: `e` at 414..415
UnresolvedName: unresolved name: `b2` at 177..179
UnresolvedName: unresolved name: `anys` at 388..392
```

Representative source:

```ts
interface Ellement {
    dummy;
    p: any;
}

declare var e: Ellement;
var arr: Contextual[] = [e];
var obj: { [s: string]: Contextual } = { s: e };
```

Current compiler evidence:

- Tokens include `declare var e: Ellement;`.
- AST erases the ambient variable declaration and keeps `arr`, `obj`, `conditional`, and `contextualOr`.
- Smart triage visible symbols list `e` at line 13, but `resolve_names` reports `UnresolvedName` for the `e` inside `[e]`.
- `booleanAssignment.ts` has the same shape: visible-symbol extraction lists `b2` from `declare var b2:boolean;`, but `resolve_names` reports `UnresolvedName` for `b2` in `b = b2`.
- `booleanFilterAnyArray.ts` has the same shape with `declare let anys: Ari<any>;`: tokens and AST succeed, visible-symbol extraction lists `anys`, and `resolve_names` reports `UnresolvedName` for `anys` in `anys.filter(Bullean)`.

TypeScript oracle evidence:

```text
e: Ellement
arr: Contextual[]
obj: { [s: string]: Contextual; }
```

The oracle also reports TS2873 for `null ? e : e`; that later always-falsy diagnostic is out of scope for this name-resolution slice.

## Desired final state

Declaration-only ambient value declarations (`declare var`, `declare let`, `declare const`) are visible to name resolution as ambient value symbols without emitting runtime declarations. The representative case should no longer fail at the first reference to `e`.

## Scope

In scope:

- [ ] Preserve resolver-visible metadata for declaration-only ambient value declarations erased by the parser.
- [ ] Resolve references to ambient `declare var` / `declare let` / `declare const` names in expressions such as `[e]` and `{ s: e }`.
- [ ] Keep ambient declarations with initializers rejected at the existing unsupported boundary.
- [ ] Add focused coverage for a declaration-only ambient value referenced in an array literal and object literal.
- [ ] Re-run the representative triage and confirm the current `UnresolvedName: e` blocker is gone.

Out of scope:

- Runtime implementation or initialization of ambient values.
- TypeScript structural/contextual type checking for `Contextual` and `Ellement`.
- The later ternary expression lowering blocker, tracked by issue 5160.
- The TypeScript TS2873 always-falsy diagnostic for `null ? e : e`.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/ast.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- `crates/backend-wasm/src/` unless triage proves the unresolved-name blocker has advanced to backend emission.

## Acceptance criteria

- [ ] `declare var e: Ellement; var arr = [e];` no longer reports `UnresolvedName` for `e`.
- [ ] `declare const c: number; var obj = { c };` resolves the ambient value name without emitting a runtime declaration.
- [ ] `declare var b2: boolean; b = b2;` resolves the ambient value name without emitting a runtime declaration.
- [ ] `declare let anys: Ari<any>; var xs = anys.filter(Bullean);` resolves the ambient value name before later filter/type-predicate behavior is evaluated.
- [ ] Ambient declarations with initializers, such as `declare var e = 1;`, remain rejected.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestCommonTypeWithContextualTyping.ts` no longer reports `UnresolvedName: unresolved name: \`e\``.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestCommonTypeWithContextualTyping.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/booleanAssignment.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/booleanFilterAnyArray.ts
```

Impacted commands:

```sh
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

Split from generated bucket `1044` on 2026-05-06. Generated buckets `1081` and `1082` were folded in on the same date after fresh triage showed the same ambient value declaration name-resolution gap for `declare var b2:boolean;` and `declare let anys: Ari<any>;`. Existing ambient-erasure work made declaration-only syntax parseable; this slice is specifically about preserving enough erased metadata for name resolution.

Additional superseded bucket:

- `issues/done/1463-implement-constWithNonNull.md` reaches the same ambient
  value name-resolution boundary for `declare const x: number | undefined;`.
  Fresh triage on 2026-05-07 reports
  `UnresolvedName: unresolved name: \`x\` at 73..74` for the later `x!++`
  expression; TypeScript parses the use as
  `PostfixUnaryExpression -> NonNullExpression -> Identifier` and then reports
  TS2588 because assignment to the ambient const is illegal.
- `issues/done/1466-implement-constraints.md` reaches the same ambient value
  name-resolution boundary for `declare var v1: C<A>;` and
  `declare var v2: C<B>;`. Fresh triage on 2026-05-07 reports
  `UnresolvedName: unresolved name: \`v1\` at 204..206` for `var y = v1.x.a;`;
  TypeScript parses the declarations and later reports TS2344 because `B` does
  not satisfy the generic constraint `A`.
- `issues/done/1508-implement-contextualTypeBasedOnIntersectionWithAnyInTheMix-unknown-unsupported.md`
  first hit an interface generic-default parser-erasure bug. After that parser
  boundary was fixed, fresh triage reports
  `UnresolvedName: unresolved name: \`styled\` at 806..812` for
  `declare const styled: StyledInterface; export const StyledSelect = styled(Flex).attrs(...)`.
  This is the same ambient `declare const` value reference boundary covered by
  this issue.
- `issues/done/582-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md`
  reaches the same ambient value name-resolution boundary after parser support
  for generic class declarations and instantiation-expression type aliases.
  Fresh triage on 2026-05-08 reports `UnresolvedName` for
  `declare const e: ErrAlias<number>; e as ErrAlias<string>;` and
  `declare const wat: Wat<number>; wat as Wat<string>;`. TypeScript later
  reports TS2352 conversion diagnostics, but the current compiler blocker is
  still resolver visibility for declaration-only ambient const values.
- `issues/done/3372-implement-moduleResolution-name-resolution.md` reaches the
  same ambient value name-resolution boundary in
  `moduleResolution_automaticTypeDirectiveNames.ts`. Fresh triage on
  2026-05-08 tokenizes two `declare const a` declarations from reference
  sections, erases them from the executable AST, and then reports
  `UnresolvedName: unresolved name: \`a\`` for the later `a;` expression.
  TypeScript later reports duplicate block-scoped variable diagnostics for the
  two ambient declarations, but the current compiler blocker is still
  resolver visibility for declaration-only ambient const values.
- `issues/done/3436-implement-nanEquality.md` reaches the same ambient value
  name-resolution boundary for `declare const x: number;` and later
  comparisons such as `if (x === NaN) {}`. Fresh triage on 2026-05-08 reports
  `UnresolvedName: unresolved name: \`x\` at 49..50`; TypeScript later reports
  TS2845 always-true/false diagnostics for comparisons with the global `NaN`,
  but the current compiler blocker is still resolver visibility for the
  declaration-only ambient const.

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
