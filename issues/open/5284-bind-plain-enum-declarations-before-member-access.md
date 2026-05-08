---
id: 5284
title: "Bind plain enum declarations before member access"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-08
---

## Summary

Accept a plain `enum Name { ... }` declaration far enough that later
`Name.Member` references do not fail as unresolved names.

## Problem

`commentsEnums.ts` tokenizes the plain enum declaration, but the ts2wasm AST
omits it. The later member access then fails in name resolution:

```text
UnresolvedName: unresolved name: `Colors` at 254..260
```

Problem: `enum Colors { Cornflower, FancyPink }` does not create a frontend
binding before `Colors.Cornflower` is resolved.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsEnums.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsEnums.ts --detail --no-dashboard-data
```

Observed result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
error: [UnresolvedName] unresolved name: `Colors` at 254..260
```

Source context:

```ts
enum Colors {
    /** Fancy name for 'blue'*/
    Cornflower /* blue */,
    /** Fancy name for 'pink'*/
    FancyPink
} // trailing comment
var x = Colors.Cornflower;
x = Colors.FancyPink;
```

Compiler evidence:

```text
tokens: ok, with enum spelled as Ident("enum") followed by Ident("Colors")
ast: ok but contains only uses of Colors, not the enum declaration
resolved: UnresolvedName for Colors
TypeScript oracle: ok, AST includes EnumDeclaration, binding x has type Colors
```

## Desired final state

The frontend records a plain enum declaration binding before resolving
`Name.Member`, or advances to the narrower enum transform/runtime boundary.

## Scope

In scope:

- [ ] Parse/bind a plain top-level enum declaration with identifier members before resolving `Name.Member`, then re-run the representative reference triage and confirm the `UnresolvedName` boundary is gone.

Out of scope:

- `const enum`, tracked separately by `issues/open/5184-parse-const-enum-declarations.md`.
- `export enum`, tracked separately by `issues/open/5277-parse-export-enum-declarations-to-enum-boundary.md`.
- Computed enum members, merged enums, declaration emit, and full enum runtime transform.
- Comment emit fidelity.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- focused enum fixtures

Do not touch:

- backend/runtime ABI unless existing enum lowering cannot report a source-spanned enum boundary
- unrelated import/export or const-enum forms

## Acceptance criteria

- [ ] `commentsEnums.ts` no longer reports `UnresolvedName` for `Colors` at `Colors.Cornflower`.
- [ ] `nestedExcessPropertyChecking.ts` no longer reports `UnresolvedName` for
  `E` at `E.A`.
- [ ] A focused fixture covers `enum Colors { Cornflower, FancyPink }` followed by `Colors.Cornflower`.
- [ ] Remaining unsupported enum behavior reports an enum-specific source-spanned diagnostic instead of a generic unresolved-name diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(enum)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsEnums.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsEnums.ts --detail --no-dashboard-data
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

Split from `issues/done/1365-implement-commentsEnums.md`.

Related but not duplicates:

- `issues/open/428-implement-enum.md` is the broad enum generated bucket.
- `issues/open/2121-implement-enumBasics-parser-syntax.md` and
  `issues/open/2143-implement-enumPropertyAccess.md` are generated buckets, not
  implementation-ready slices.
- `issues/open/5184-parse-const-enum-declarations.md` covers `const enum`.
- `issues/open/5277-parse-export-enum-declarations-to-enum-boundary.md` covers
  `export enum`.

2026-05-08 fold-in:

- `issues/done/3477-implement-nestedExcessPropertyChecking.md` reaches the same
  plain enum binding boundary for `enum E { A = "A" }` followed by
  `let x: { nope?: any } = E.A;`.
- Current diagnostic: `UnresolvedName: unresolved name: \`E\` at 363..364`.
- TypeScript oracle reports the later TS2559 excess-property/type
  compatibility diagnostic after resolving `E.A`.
- `issues/done/3542-implement-noImplicitAnyIndexing.md` reaches the same plain
  enum binding boundary for `enum MyEmusEnum { emu }` followed by
  `MyEmusEnum[0]`; detailed indexing follow-up should be re-triaged after this
  enum binding issue advances.
- `issues/done/3543-implement-noImplicitAnyIndexingSuppressed.md` reaches the
  same plain enum binding boundary in the suppressed variant before the fixture
  can expose its narrower noImplicitAny indexing behavior.

## Completion evidence

Fill when implemented.
