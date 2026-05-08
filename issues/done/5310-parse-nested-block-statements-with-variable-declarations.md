---
id: 5310
title: "Parse nested block statements with variable declarations"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse nested block statements as statements inside other blocks when the nested
block contains variable declarations such as `var y = 0;` or `const c = false;`.

## Problem

`constDeclarationShadowedByVarDeclaration.ts` parses the first top-level block,
but fails when a block body contains another `{ ... }` block. The parser does
not dispatch `LeftBrace` as a nested block statement inside `statement()`, so it
falls through to expression parsing and then reports a comma expectation at the
inner `var y` declaration.

Problem: a nested block containing `var y = 0;` reports `expected Comma, got Some(Ident("y"))` instead of parsing as a block statement.
The same parser boundary appears in `constDeclarations-scopes.ts` for an inner
block containing `const c = false;`.

## Current failure

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected Comma, got Some(Ident("y")) at 200..201
```

Source context:

```text
var y = 0;
{
    const y = 0;
    {
        var y = 0;
    }
}
```

TypeScript oracle parses the source and reports later TS2481 diagnostics for
the shadowing rule.

Second reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-scopes.ts
```

Observed 2026-05-07:

```text
UnsupportedSyntax: expected Comma, got Some(Ident("c")) at 1018..1019
source: { const c = false; var b: boolean = c; }
```

## Desired final state

The parser accepts nested block statements inside block bodies and preserves the
inner statements for later scope analysis, so this reference case advances past
the current parser failure.

## Scope

In scope:

- [x] Dispatch `LeftBrace` as a nested block statement inside `statement()`.
- [x] Preserve variable declarations inside nested blocks.
- [x] Add focused parser coverage for `{ const y = 0; { var y = 0; } }` and an inner `const` declaration.

Out of scope:

- TS2481 shadowing diagnostics for `var` writes to outer block-scoped names.
- Nested block class declaration semantics, tracked by issue 5250.
- Nested block function expression statements, tracked by issue 5212.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- backend/runtime lowering

## Acceptance criteria

- [x] `constDeclarationShadowedByVarDeclaration.ts` no longer reports `expected Comma, got Some(Ident("y"))`.
- [x] `constDeclarations-scopes.ts` no longer reports `expected Comma, got Some(Ident("c"))`.
- [x] A focused parser test accepts `{ const y = 0; { var y = 0; } }`.
- [x] Existing top-level block flattening behavior remains unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarationShadowedByVarDeclaration.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-scopes.ts
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

Split from generated bucket
`issues/open/1439-implement-constDeclarationShadowedByVarDeclaration.md`.
Also supersedes the current parser boundary from
`issues/open/1443-implement-constDeclarations-scope-analysis.md`.

After this parser blocker is fixed, the reference case should be triaged for
the TS2481 const/var shadowing diagnostic.

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

- none
