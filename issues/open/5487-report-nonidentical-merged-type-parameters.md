---
id: 5487
title: "Report non-identical type parameters across merged declarations"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: [3593]
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Implement the semantic diagnostic for merged declarations whose type parameter
lists are not identical. The motivating reference is
`nonIdenticalTypeConstraints.ts`, where class/interface declarations for the
same symbol disagree on type parameter constraints or names.

## Problem

The compiler currently build-passes
`reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts`.
TypeScript reports TS2428 for merged declarations of `Foo`, `Qux`, and `Quux`
because all declarations of a merged symbol must have identical type
parameters.

Problem: merged declarations with non-identical type parameter lists are
accepted without the TS2428 semantic diagnostic.

## Current failure

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts
```

Observed result:

```text
headline: BuildPass
oracle: TS2428 All declarations of 'Foo' must have identical type parameters.
oracle: TS2428 All declarations of 'Qux' must have identical type parameters.
oracle: TS2428 All declarations of 'Quux' must have identical type parameters.
```

Representative source:

```ts
class Foo<T extends Function> {
    n: T;
}
interface Foo<T extends Different> {
    y: T;
}

class Quux<T> {
    n: T;
}
interface Quux<U> {
    m: U;
}
```

## Desired final state

The reference fixture emits TS2428-equivalent semantic diagnostics for merged
declarations whose type parameter lists differ by constraint or type parameter
name, while still accepting matching merged declarations such as `Bar` and
`Baz`.

## Scope

In scope:

- [ ] Track type parameter names and constraints for mergeable class/interface
  declarations in the frontend semantic path.
- [ ] Compare merged declarations for identical type parameter lists.
- [ ] Emit a TS2428-equivalent diagnostic for each non-identical declaration
  site in `nonIdenticalTypeConstraints.ts`.
- [ ] Preserve build-pass behavior for matching class/interface declarations.

Out of scope:

- Strict property initialization diagnostics, owned by issue 5356.
- Full structural assignability or generic constraint satisfaction.
- Declaration emit behavior.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`
- unrelated coverage dashboard artifacts

## Acceptance criteria

- [ ] `nonIdenticalTypeConstraints.ts` reports TS2428-equivalent diagnostics
  for `Foo`, `Qux`, and `Quux`.
- [ ] The same fixture does not report TS2428 for matching merged declarations
  `Bar` or `Baz`.
- [ ] A focused regression fixture or reference assertion covers constraint
  mismatch and type parameter name mismatch.
- [ ] Issue 5356 remains the owner for TS2564 uninitialized class fields in the
  same fixture.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts
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

This issue was split from generated bucket
`issues/open/3593-implement-nonIdenticalTypeConstraints.md`. The parser and
resolver already traverse the fixture; the missing work is semantic parity.

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

- TS2564 diagnostics in this fixture are tracked separately by issue 5356.
