---
id: 5281
title: "Resolve arrow rest parameter bindings"
type: feature
area: ir/name-resolution
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Fix name resolution for arrow function rest parameters, including cases where
comments/trivia appear between `...` and the parameter name.

## Problem

`commentsAfterSpread.ts` now tokenizes and parses object spread, function rest
parameters, and arrow function expressions, but name resolution fails inside
the first arrow rest parameter body:

```text
UnresolvedName: unresolved name: `args` at 725..729
```

Problem: arrow rest parameters are not made visible under their identifier name
for body resolution in the current `Expr::ArrowFn` path.

## Current Failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsAfterSpread.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsAfterSpread.ts --detail --no-dashboard-data
```

Observed result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
error: [UnresolvedName] unresolved name: `args` at 725..729
```

Source context:

```ts
const h = (.../* comment h */args) => args.length;

const i = (
  first, .../* comment i */rest
) => rest.length;
```

Compiler evidence:

```text
tokens: ok; comments are skipped and spread/rest tokens are present
ast: ok; earlier object spreads and arrow function expressions parse
resolved: fails in resolve_names with UnresolvedName for `args`
TypeScript oracle: ok, parameter `args` has type `any[]`
```

## Desired Final State

The resolver treats arrow rest parameters as ordinary local parameters under
the identifier name after stripping/rest-normalizing the `...` marker, so body
references like `args.length` and `rest.length` resolve.

## Scope

In scope:

- [ ] Normalize arrow function rest parameter names for resolver parameter binding.
- [ ] Add focused coverage for `(.../* comment */args) => args.length`.
- [ ] Re-run `commentsAfterSpread.ts` and confirm the failure advances past
      `UnresolvedName: args`.

Out of scope:

- Object rest/spread runtime semantics after this resolver blocker.
- Function declaration rest parameter semantics, already parsed/resolved before
  this first arrow blocker.
- Full comment emit preservation.
- Broad arrow-function or name-resolution epics.

## Affected paths

Expected:

- `crates/frontend/src/parser/expressions_main.rs`
- `crates/ir/src/lowered/program_captures.rs`
- `crates/ir/src/lowered/resolver.rs`
- focused parser/lowered resolver tests

Do not touch:

- module graph or backend emit unless a focused test proves this cannot be
  represented in existing function metadata
- unrelated object spread lowering

## Acceptance criteria

- [ ] `(.../* comment */args) => args.length` and `(first, .../* comment */rest) => rest.length` resolve their rest parameters.
- [ ] `contextualTupleTypeParameterReadonly.ts` no longer reports
  `UnresolvedName` for `args` inside `eacher((...args) => { ... })`.
- [ ] Existing non-rest arrow parameter tests still pass.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsAfterSpread.ts` no longer reports `UnresolvedName: unresolved name: \`args\``.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend arrow
cargo nextest run -p ts2wasm-ir arrow
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsAfterSpread.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsAfterSpread.ts --detail --no-dashboard-data
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

Split from `issues/done/1357-implement-commentsAfterSpread.md`.

Related but not duplicates:

- `issues/done/1300-implement-collisionRestParameterArrowFunctions.md` is a
  generated semantic collision bucket and lacks current smart-triage evidence
  for this resolver failure.
- `issues/done/038-implement-rest-parameters.md` and
  `issues/done/212-implement-rest-parameter-argument-collection.md` cover
  general rest parameter support, but this reference still shows an arrow
  resolver binding gap.
- `issues/done/5064-implement-arrow-function.md` is the broad arrow-function
  generated bucket and was superseded by narrower children.

2026-05-07 fold-in:

- `issues/done/1505-implement-contextualTupleTypeParameterReadonly.md` reaches
  the same resolver gap for an uncommented arrow rest parameter:
  `eacher((...args) => { const [a, b] = args; })`.
- Current diagnostic: `UnresolvedName: unresolved name: \`args\`` inside the
  arrow body. TypeScript oracle binds `args` and reports an earlier TS2345
  readonly tuple assignability diagnostic.

## Completion Evidence

Fill when implemented.
