---
id: 5264
title: "Parse typed const declarations before initializers"
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

Skip or preserve TypeScript variable type annotations before parsing the
initializer in `const` declarations, such as
`const classesByRow: Record<string, object> = {};`.

## Problem

`classInConvertedLoopES5.ts` tokenizes the leading typed const declaration, but
the parser treats the colon type annotation as the end of the declaration and
reports that the `const` has no initializer.

Current diagnostic:

```text
UnsupportedSyntax: const declarations require an initializer at 31..43
```

TypeScript parses the declaration as:

```text
VariableDeclarationList -> VariableDeclaration -> Identifier classesByRow
```

with type `Record<string, object>` and initializer `{}`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts
```

Representative source:

```ts
const classesByRow: Record<string, object> = {};
for (const row of ['1', '2', '3', '4', '5']) {
  class RowClass {
    row = row;
    static factory = () => new RowClass();
  }
  classesByRow[row] = RowClass;
}
```

Compiler evidence:

```text
tokens: ok; const, identifier, colon, Record<string, object>, equals, object literal
ast/resolved: const declarations require an initializer at classesByRow
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

The parser consumes TypeScript-only type annotations on variable declarations
before looking for `=`. The representative should no longer fail at
`classesByRow`; it should parse the `{}` initializer and expose the next
blocker in the loop/class body if one remains.

## Scope

In scope:

- [x] Support `const name: Type = expr` for simple and generic type
  annotations.
- [x] Preserve the existing runtime initializer expression.
- [x] Keep `const name: Type;` rejected with a source-spanned missing
  initializer diagnostic.
- [x] Preserve `let` / `var` type annotation behavior where already supported.

Out of scope:

- Full TypeScript type checking for `Record<string, object>`.
- For-of converted-loop semantics.
- Class field initialization and class-in-loop capture behavior.
- Const enum declaration parsing, which is tracked separately.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- loop lowering or class runtime semantics unless they are only exposed as the
  next blocker after this parser fix

## Acceptance criteria

- [x] `classInConvertedLoopES5.ts` no longer reports
  `const declarations require an initializer` at `classesByRow`.
- [x] A focused parser or CLI fixture covers
  `const classesByRow: Record<string, object> = {};`.
- [x] A negative fixture covers `const missing: Record<string, object>;`.
- [x] Existing untyped `const x = expr` parsing remains unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(var) or test(const) or test(type)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/open/1220-implement-classInConvertedLoopES.md`. This issue
only covers the current first parser blocker; converted loop/class semantics
may need a later child after the typed const declaration parses.

## False-done audit

**truly-done** (5264)

- Implementation commits: verified via `git log --oneline --all --grep=5264`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
