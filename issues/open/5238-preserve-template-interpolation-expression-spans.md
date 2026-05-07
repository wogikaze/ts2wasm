---
id: 5238
title: "Preserve template interpolation expression spans"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Preserve source spans for expressions parsed inside template literal
interpolations, so diagnostics for `` `${a}` `` point at the real `a` token
instead of synthetic `0..1` spans from the temporary interpolation parser.

## Problem

`circularBaseConstraint.ts` currently erases the type aliases and reaches the
runtime expression `` `${a}` as B<T>; ``. Name resolution correctly reports
that `a` is unresolved, matching TypeScript's TS2304 diagnostic, but the span
is wrong.

Problem: `` `${a}` `` reports `UnresolvedName: unresolved name: a at 0..1`
instead of pointing at the `a` token in the original source.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularBaseConstraint.ts
```

Source context:

```ts
function foo<T>() {
    `${a}` as B<T>;
}
```

Compiler evidence:

```text
tokens: ok; includes template literal, `as B<T>`, and erased conditional type aliases
ast: Function foo body contains Binary(String(""), Add, Ident("a" span=0..1))
resolved: UnresolvedName unresolved name `a` at 0..1
TypeScript oracle: TS2304 Cannot find name 'a' at line 18, character 8
```

## Desired final state

Expressions parsed from template interpolation source are offset back into the
original template token span before being attached to the AST. The
representative case may still report `UnresolvedName` for `a`, but the
diagnostic span should point at the real interpolation expression.

## Scope

In scope:

- [ ] Offset spans for identifiers and simple expressions produced by `parse_template_expression`.
- [ ] Add a focused parser or resolver regression for `` `${a}` `` where the `a` expression span is source-backed.
- [ ] Re-run the representative triage and confirm the unresolved `a` diagnostic no longer uses `0..1`.

Out of scope:

- Type-system support for conditional types, `infer`, `keyof`, or indexed access types.
- Changing the fact that genuinely unresolved interpolation identifiers report `UnresolvedName`.
- Nested template literal support inside interpolation expressions.

## Affected paths

Expected:

- `crates/frontend/src/parser/helpers.rs`
- focused parser/resolver test

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] The AST for `` `${a}` `` records `a` with its original source span, not `0..1`.
- [ ] `circularBaseConstraint.ts` no longer reports `UnresolvedName` at `0..1` for the interpolation identifier.
- [ ] Existing template interpolation parsing still accepts simple identifiers and string/number expressions.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(template) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularBaseConstraint.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularBaseConstraint.ts --detail --no-dashboard-data
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
`issues/done/1150-implement-circularBaseConstraint.md`.

Related:

- `issues/done/056-implement-name-resolution.md` owns the general unresolved-name diagnostic contract.
- `issues/done/213-implement-template-literal-interpolation.md` covers basic template literal interpolation support.

## Completion evidence

Fill when implemented.
