---
id: 5244
title: "Support namespace-merged function static properties"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Handle TypeScript namespace/function merging for static properties such as
`maker.Bar`, where a `namespace maker { export class Bar ... }` augments a
function value.

## Problem

`circularTypeofWithFunctionModule.ts` parses the function body and namespace
declaration enough to reach lowering, but `return maker.Bar` is treated as an
unsupported function metadata property:

```text
UnsupportedSyntax: issue-062f: function `maker` metadata property `Bar` is not supported
```

Problem: TypeScript namespace merging can attach exported namespace members as
static properties of a function symbol, but the lowering path currently only
recognizes the narrow function metadata subset from issue 062f.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-062f: function `maker` metadata property `Bar` is not supported at 122..131
```

Representative source:

```ts
class Foo {}

function maker(value: string): typeof maker.Bar {
    return maker.Bar;
}

namespace maker {
    export class Bar extends Foo {}
}
```

Triage evidence:

- Tokens succeed for class `Foo`, function `maker`, `typeof maker.Bar`,
  `return maker.Bar`, and `namespace maker`.
- AST contains `ClassDecl Foo` and `Function maker` with a return expression
  `Member { object: Ident("maker"), property: "Bar" }`.
- Resolved/lowered dump stops at `issue-062f` for unsupported function metadata
  property `Bar`.
- TypeScript oracle accepts the source with no diagnostics and infers
  `maker` as returning `typeof Bar`.

## Desired final state

The compiler represents or explicitly lowers namespace-merged function static
properties so `maker.Bar` no longer falls through to the generic unsupported
function metadata diagnostic.

## Scope

In scope:

- [ ] Recognize a function declaration merged with a same-name namespace.
- [ ] Expose exported namespace class/value members as supported static
  properties on the function symbol for read access.
- [ ] Keep ordinary function metadata (`name`, `length`, unsupported
  `prototype`) behavior from issue 062f unchanged.
- [ ] Add focused coverage for `function maker() { return maker.Bar; }`
  merged with `namespace maker { export class Bar {} }`.

Out of scope:

- Full TypeScript namespace emit.
- Arbitrary namespace merging across files or modules.
- Static writes or dynamic property names on merged function objects.
- Type-only `typeof maker.Bar` checking beyond whatever is needed to pass the
  current parser/lowering boundary.

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/` unless lowering requires a reviewed runtime shape.

## Acceptance criteria

- [ ] `circularTypeofWithFunctionModule.ts` no longer reports
  `issue-062f: function maker metadata property Bar is not supported`.
- [ ] A focused fixture proves read access to a namespace-merged function
  static member.
- [ ] Existing function `name` / `length` metadata tests still pass.
- [ ] Unsupported unrelated function metadata properties still report an
  issue-linked diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(function) or test(metadata) or test(namespace)'
cargo nextest run -p ts2wasm-cli -E 'test(function) or test(metadata) or test(namespace)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1167-implement-circularTypeofWithFunctionModule.md`.

## Completion evidence

Fill when implemented.
