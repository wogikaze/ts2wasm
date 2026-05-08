---
id: 5276
title: "Report class declaration decorator boundary"
type: feature
area: frontend/lexer
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Recognize `@` decorator syntax before class declarations enough to emit a
source-spanned TypeScript decorator boundary diagnostic instead of a generic
lexer `unsupported character: @` failure.

## Problem

Problem: `commentOnDecoratedClassDeclaration.ts` stops in tokenization at the
first decorated class declaration:

```text
UnsupportedSyntax: unsupported character: @ at 123..124
```

The TypeScript oracle accepts the file under `experimentalDecorators` and parses
both decorated declarations as `ClassDeclaration` nodes with `Decorator`
children. The current lexer failure hides the intended frontend decorator
boundary and blocks comment/trivia coverage around decorated declarations.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts
```

Current diagnostic:

```text
UnsupportedSyntax: unsupported character: @ at 123..124
line 8, column 1
```

Source context:

```ts
declare function decorator(x: string): any;

/**
 * Leading trivia
 */
@decorator("hello")
class Remote { }

/**
 * Floating Comment
 */

@decorator("hi")
class AnotherRomote {
    constructor() {}
}
```

TypeScript AST evidence:

```text
SourceFile
- FunctionDeclaration "declare function decorator(x: string): any;"
- ClassDeclaration "@decorator(\"hello\")\nclass Remote { }"
  - Decorator "@decorator(\"hello\")"
- ClassDeclaration "@decorator(\"hi\")\nclass AnotherRomote { ... }"
```

Visible symbols before failure:

```text
function decorator(x: string)
```

## Scope

In scope:

- [ ] Tokenize `@` before class declarations in TypeScript sources as decorator syntax rather than an unsupported character.
- [ ] Recognize one or more decorators before a `class` declaration in statement/exportable declaration position.
- [ ] Preserve comments/trivia around decorated class declarations while advancing to a source-spanned decorator boundary diagnostic.
- [ ] Add focused frontend/CLI coverage for decorated class declarations.

Out of scope:

- Full decorator transform or runtime decorator calls.
- Decorator metadata emit.
- Class expression decorator handling, which is tracked separately by `issues/open/5253-report-class-expression-decorator-boundary.md`.
- Parameter, method, property, accessor, or export-position decorator semantics beyond preserving the same boundary diagnostic if encountered.

## Affected paths

Expected:

- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- runtime/backend decorator lowering
- TypeScript checker diagnostics

## Acceptance criteria

- [ ] `commentOnDecoratedClassDeclaration.ts` no longer reports `unsupported character: @`.
- [ ] `noEmitHelpers2.ts` no longer reports `unsupported character: @` at the
  class declaration decorator before `class A`.
- [ ] A focused test proves `@decorator("hello") class Remote {}` reaches a source-spanned decorator boundary diagnostic.
- [ ] Leading/floating comments around the decorated declarations do not regress tokenization or parser recovery.
- [ ] Existing non-decorator unsupported characters still report lexer diagnostics.
- [ ] If parsing advances to a broader decorator transform blocker, that next blocker is recorded separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnDecoratedClassDeclaration.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1348-implement-commentOnDecoratedClassDeclaration.md`.
Related broad decorator bucket: `issues/open/4807-implement-decorator.md`.
Related class-expression decorator boundary: `issues/open/5253-report-class-expression-decorator-boundary.md`.
Also owns `issues/done/3359-implement-modulePreserveImportHelpers.md`: fresh
triage for `modulePreserveImportHelpers.ts` stops at `@dec()` before exported
and non-exported class declarations before import helpers, package metadata, or
tslib declarations become actionable.
Also owns `issues/done/3529-implement-noEmitHelpers.md`: fresh triage for
`noEmitHelpers2.ts` stops at the class declaration decorator `@decorator`
before `class A`. The same file also contains a constructor parameter decorator
that TypeScript reports as TS1206; parameter decorator semantics remain outside
this issue and should be recorded separately after the class-decorator lexer
boundary advances.
