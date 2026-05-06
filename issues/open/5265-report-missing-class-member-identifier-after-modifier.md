---
id: 5265
title: "Report missing class member identifier after modifier"
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

Report a source-spanned parser diagnostic when a class member modifier such as
`public` is followed by `{` instead of a member name.

## Problem

`classMemberWithMissingIdentifier.ts` and
`classMemberWithMissingIdentifier2.ts` tokenize successfully, but the class
member parser reports a generic property-name failure when it sees `{` after
`public`.

Current diagnostic:

```text
UnsupportedSyntax: expected property name, got LeftBrace at 63..64
```

TypeScript reports `TS1146: Declaration expected.` at the position after
`public`, then continues parsing the following `{...}` as a block outside the
class declaration.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier2.ts
```

Representative sources:

```ts
class C {
    public {};
}
```

```ts
class C {
    public {[name:string]:VariableDeclaration};
}
```

Compiler evidence:

```text
tokens: ok; class C, modifier public, LeftBrace
ast/resolved: expected property name, got LeftBrace
TypeScript oracle: TS1146 at the missing declaration after public
```

## Desired final state

The parser recognizes the missing class member identifier after a modifier and
emits a diagnostic at the modifier boundary or following `{`, rather than a
generic property-name error. Both representative references should no longer
report `expected property name, got LeftBrace`.

## Scope

In scope:

- [ ] Detect `public {` / `private {` / `protected {` in class member parsing.
- [ ] Emit a source-spanned missing-member-identifier diagnostic.
- [ ] Preserve valid access modifier parsing for fields, methods, accessors,
  and index signatures.
- [ ] Keep parser recovery narrow enough that following class members remain
  parseable when present.

Out of scope:

- Full TypeScript error recovery outside class bodies.
- Supporting arbitrary block statements inside class declarations.
- Type-checking errors inside the recovered block expression.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- class runtime lowering

## Acceptance criteria

- [ ] `classMemberWithMissingIdentifier.ts` no longer reports
  `expected property name, got LeftBrace`.
- [ ] `classMemberWithMissingIdentifier2.ts` no longer reports
  `expected property name, got LeftBrace`.
- [ ] A focused fixture covers `class C { public {}; }`.
- [ ] A focused fixture covers `class C { public {[name:string]: T}; }`.
- [ ] Valid `public field`, `public method()`, and public class index
  signatures remain unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(parser) or test(modifier)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter classMemberWithMissingIdentifier --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1225-implement-classMemberWithMissingIdentifier.md`.
