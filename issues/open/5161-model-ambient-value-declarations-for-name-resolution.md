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

The parser erases declaration-only ambient variables such as `declare var e: Ellement;`, but the name resolver then rejects later runtime expressions that reference `e`. This blocks `bestCommonTypeWithContextualTyping.ts` before the compiler can reach the contextual typing and ternary checks in the reference case.

## Problem

Problem: `reference/typescript/tests/cases/compiler/bestCommonTypeWithContextualTyping.ts` currently reports `UnresolvedName` for `e` in `[e]`, even though `declare var e: Ellement;` is visible earlier in the source.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestCommonTypeWithContextualTyping.ts
```

Current compiler diagnostic:

```text
UnresolvedName: unresolved name: `e` at 414..415
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
- [ ] Ambient declarations with initializers, such as `declare var e = 1;`, remain rejected.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestCommonTypeWithContextualTyping.ts` no longer reports `UnresolvedName: unresolved name: \`e\``.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestCommonTypeWithContextualTyping.ts
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

Split from generated bucket `1044` on 2026-05-06. Existing ambient-erasure work made declaration-only syntax parseable; this slice is specifically about preserving enough erased metadata for name resolution.

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
