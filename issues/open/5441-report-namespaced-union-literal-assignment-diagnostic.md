---
id: 5441
title: "Report namespaced union literal assignment diagnostic"
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

Report a source-spanned semantic diagnostic when an object literal with an
inferred widened string property is assigned through a local binding to a
namespace-qualified union of exported object type aliases.

## Problem

`namespaceDisambiguationInUnion.ts` now builds successfully, but TypeScript
reports TS2322 for the first typed assignment:

```ts
namespace Foo {
  export type Yep = { type: "foo.yep" };
}

namespace Bar {
  export type Yep = { type: "bar.yep" };
}

const x = { type: "wat.nup" };
const val1: Foo.Yep | Bar.Yep = x;
```

Current compiler evidence:

```text
BuildPass: ts2wasm build succeeded
ast/resolved retain only x object literal and val1 = x; namespace type aliases and the union annotation are erased
```

TypeScript oracle evidence:

```text
TS2322: Type '{ type: string; }' is not assignable to type 'Foo.Yep | Bar.Yep'.
  Type '{ type: string; }' is not assignable to type 'Yep'.
    Types of property 'type' are incompatible.
      Type 'string' is not assignable to type '"bar.yep"'.
```

Problem: erased namespace type aliases and union annotations hide the invalid
assignment, so the reference reports `BuildPass` while TypeScript reports a
semantic diagnostic at `val1`.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts
```

Current result:

```text
BuildPass: ts2wasm build succeeded
TypeScript oracle reports TS2322 at line 11, character 7, on `val1`.
```

## Desired final state

The compiler preserves enough namespaced type-alias and union annotation
information to reject the representative `val1` assignment before reporting a
build pass.

## Scope

In scope:

- [ ] Preserve exported namespace type aliases for the focused object-literal
  alias shape `{ type: "literal" }`.
- [ ] Preserve or classify the focused annotation shape
  `Foo.Yep | Bar.Yep` on a `const` local.
- [ ] Track the inferred widened type `{ type: string }` for a local object
  literal binding used by the annotated assignment.
- [ ] Report a source-spanned diagnostic for
  `const val1: Foo.Yep | Bar.Yep = x;`.
- [ ] Add focused coverage for two namespaces with distinct exported object
  type aliases and an incompatible local object binding.

Out of scope:

- Full TypeScript assignability.
- General namespace type checker parity beyond exported object type aliases.
- Literal narrowing rules beyond the object-property widening needed here.
- The later fixed tuple assignment diagnostic for
  `const val2: [Foo.Yep, Bar.Yep] = y;`.
- Runtime preservation of type-only namespace declarations.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- focused fixtures or reference assertions

Do not touch:

- backend emit unless a focused diagnostic path proves it is necessary
- broad module/import resolution

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts` no longer reports plain `BuildPass` while TypeScript reports the first TS2322.
- [ ] The representative `val1` assignment is diagnosed at `val1`, `x`, or the assignment expression with a namespaced union/object-literal incompatibility message.
- [ ] A focused test covers `namespace Foo { export type Yep = { type: "foo.yep" } }`, `namespace Bar { export type Yep = { type: "bar.yep" } }`, `const x = { type: "wat.nup" }`, and `const val1: Foo.Yep | Bar.Yep = x`.
- [ ] Existing namespace-erasure and object-literal lowering tests still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend namespace
cargo nextest run -p ts2wasm-ir type
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts --detail --no-dashboard-data
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

Split from `issues/open/3429-implement-namespaceDisambiguationInUnion.md` on
2026-05-08 after fresh triage showed the original import/export blocker was
stale and the current mismatch is a false build-pass.

Related but distinct:

- `issues/open/5409a-report-non-exported-namespace-member-type-annotations.md`
  covers a missing/non-exported namespace member in a type annotation, not
  assignment compatibility for exported namespace type aliases.

## Completion evidence

Fill when implemented.
