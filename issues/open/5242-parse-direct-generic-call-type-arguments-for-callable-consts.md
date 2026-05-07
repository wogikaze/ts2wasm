---
id: 5242
title: "Parse direct generic call type arguments for callable consts"
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

Parse and erase explicit TypeScript type arguments on direct calls to declared
generic callable values, such as `object<Something>()`.

## Problem

`circularReferenceInReturnType2.ts` declares `object` as a const with a generic
call signature:

```ts
declare const object: <Source>() => <Fields>(config: { fields: Fields }) => ObjectType<Source>;
```

The later call `object<Something>()({ ... })` tokenizes successfully, but AST
construction stops at the end of the type argument list and reports:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: RightParen, ... })
```

Problem: the existing generic-call erasure path is too narrow for callable
const declarations, so a valid direct generic call is parsed as runtime
comparison-like syntax and leaves the parser at the wrong token.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 1076, end: 1077 } }) at 1077..1078
```

Representative source:

```ts
declare const object: <Source>() => <
  Fields extends {
    [Key in keyof Fields]: Field<Source, Key & string>;
  }
>(config: {
  name: string;
  fields: Fields | (() => Fields);
}) => ObjectType<Source>;

const A = object<Something>()({
  name: "A",
  fields: () => ({})
});
```

Triage evidence:

- Tokens succeed for `object`, `<`, `Something`, `>`, `(`, `)`, `(`.
- AST construction fails before it can build the outer call argument object.
- Visible symbols include declared generic callable `object`, `field`, and
  partial binding `A`.
- TypeScript AST keeps `object<Something>()({ ... })` as nested `CallExpression`
  nodes and reports no diagnostics for this file.

## Desired final state

The parser recognizes explicit TypeScript type argument lists on direct calls to
declared generic callable values and erases them before ordinary call parsing.
The representative case should advance past AST construction and either compile
further or report the next semantic/lowering diagnostic.

## Scope

In scope:

- [ ] Parse `callable<T>()` where `callable` is a declared const with a generic
  call signature.
- [ ] Reuse the existing direct generic-call type-argument erasure behavior
  where possible.
- [ ] Preserve ambiguity discipline for relational expressions such as
  `a < b > (c)`.
- [ ] Add a focused parser or CLI fixture for `declare const make: <T>() => ...;
  const out = make<number>()(1);`.

Out of scope:

- Full TypeScript generic inference.
- Generic member calls, covered by issue 5202.
- Malformed empty call type-argument diagnostics, covered by issue 5194.
- Lowering nested call-expression callees after parsing, covered by issue 5163.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- runtime semantics beyond parser fixture fallout

## Acceptance criteria

- [ ] `circularReferenceInReturnType2.ts` no longer reports the current
  `unsupported expression: ... RightParen` AST-construction diagnostic at
  `object<Something>()`.
- [ ] A focused parser/CLI test covers a generic callable const direct call.
- [ ] Existing simple generic function-call erasure still passes.
- [ ] Relational/comparison expressions are not reclassified as generic calls.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli -E 'test(parser) | test(call)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1164-implement-circularReferenceInReturnType-parser-syntax.md`.

## Completion evidence

Fill when implemented.
