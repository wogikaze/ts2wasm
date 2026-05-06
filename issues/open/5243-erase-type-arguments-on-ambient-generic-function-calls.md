---
id: 5243
title: "Erase type arguments on ambient generic function calls"
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

Erase explicit TypeScript type arguments on calls to ambient generic functions,
such as `useState<Data>(...)`.

## Problem

`circularResolvedSignature.ts` declares an ambient generic function:

```ts
declare function useState<S>(initialState: (() => S)): [S, (s: S) => void];
```

The later call `useState<Data>(() => ({ ... }))` tokenizes and builds an AST,
but the parser keeps `<Data>` as runtime binary `<` / `>` expressions. Name
resolution then tries to resolve `Data` as a runtime identifier and reports:

```text
UnresolvedName: unresolved name: `Data` at 280..284
```

Problem: the existing direct generic-call erasure guard covers simple generic
function declarations but does not cover ambient `declare function` bindings.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularResolvedSignature.ts
```

Current diagnostic:

```text
error: [UnresolvedName] unresolved name: `Data` at 280..284
```

Representative source:

```ts
declare function useState<S>(initialState: (() => S)): [S, (s: S) => void];

type Data = Readonly<{
    value: number;
}>;

export function Component() {
    const [state, setState] = useState<Data>(() => ({ value: "string" }));
}
```

Triage evidence:

- Tokens include `useState`, `<`, `Data`, `>`, `(`.
- AST currently represents the initializer as nested `Binary { op: Less }` and
  `Binary { op: Greater }` instead of an erased generic call.
- Resolved dump stops at `UnresolvedName` for `Data`.
- TypeScript oracle accepts the generic call syntax and reports the later
  semantic diagnostic `TS2322` on `value: "string"`.

## Desired final state

The parser registers ambient generic function declarations for the same
type-argument erasure path used by ordinary generic function declarations. The
representative case should no longer report `UnresolvedName` for type-only
`Data` at the call-site type argument.

## Scope

In scope:

- [ ] Track generic parameters from `declare function f<T>(...)` declarations
  for direct generic-call erasure.
- [ ] Parse `f<T>(args)` as an ordinary call when `f` is an ambient generic
  function in the current parser run.
- [ ] Preserve the existing ambiguity guard for relational expressions.
- [ ] Preserve existing generic function declaration / call fixtures.

Out of scope:

- Full TypeScript type checking for `Data`.
- Generic callable const declarations, covered by issue 5242.
- Member-call type arguments, covered by issue 5202.
- Semantic parity for the later circular resolved-signature inference.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- runtime or type-inference semantics beyond parser fallout

## Acceptance criteria

- [ ] `circularResolvedSignature.ts` no longer reports `UnresolvedName` for
  type-only `Data` in `useState<Data>(...)`.
- [ ] A focused parser or CLI fixture covers `declare function f<T>(value: T):
  T; f<number>(1);`.
- [ ] Existing simple generic function-call erasure still passes.
- [ ] Relational/comparison expressions are not reclassified as generic calls.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli -E 'test(parser) | test(call)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularResolvedSignature.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularResolvedSignature.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1165-implement-circularResolvedSignature.md`.

## Completion evidence

Fill when implemented.
