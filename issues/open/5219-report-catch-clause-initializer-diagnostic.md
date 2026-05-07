---
id: 5219
title: "Report catch clause initializer diagnostics"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse or recover from `catch (e = 1)` well enough to report a TypeScript-style
source diagnostic instead of the generic parser `expected RightParen` error.

## Problem

Problem: `catchClauseWithInitializer1.ts` tokenizes successfully, but the parser
expects `)` immediately after the catch binding identifier and fails at `=`.
TypeScript recognizes the catch variable declaration shape and reports TS1197:
catch clause variables cannot have initializers.

Current diagnostic:

```text
UnsupportedSyntax: expected RightParen, got Some(Equal) at 39..40
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```ts
try {
}
catch (e = 1) {
}
```

Compiler evidence:

```text
tokens: ok; Catch, LeftParen, Ident("e"), Equal, Number(1), RightParen
ast: fails with expected RightParen at the Equal token
resolved: fails with the same parser diagnostic
TypeScript oracle: TS1197 "Catch clause variable cannot have an initializer." at line 4, character 12
```

## Desired final state

The compiler reports a source-spanned catch-initializer diagnostic for this
invalid TypeScript syntax, ideally aligned with TS1197, instead of the generic
parser `expected RightParen` error.

## Scope

In scope:

- [ ] Parser: detect `catch (<ident> = <expr>)` after a catch binding identifier.
- [ ] Diagnostics: emit a source-spanned unsupported/TypeScript syntax diagnostic at the initializer expression or `=` token.
- [ ] Tests: add a focused parser/frontend diagnostic test for `catch (e = 1)`.
- [ ] Reference triage: ensure `catchClauseWithInitializer1.ts` no longer reports the generic `expected RightParen` parser error as the first blocker.

Out of scope:

- Runtime behavior for catch bindings.
- Destructuring catch parameters with initializers.
- Full TypeScript error-code compatibility beyond this TS1197-shaped diagnostic.
- Catch without binding or optional catch binding support, unless a regression is found.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/diagnostic.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/cli/tests/`
- focused diagnostic fixture under `fixtures/`

Do not touch:

- backend exception lowering
- runtime exception ABI
- unrelated TypeScript declaration or binding diagnostics

## Acceptance criteria

- [ ] A focused test for `catch (e = 1)` reports a source-spanned diagnostic instead of `expected RightParen`.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/catchClauseWithInitializer1.ts` no longer reports `expected RightParen, got Some(Equal)` as the first blocker.
- [ ] Existing try/catch fixtures still parse and build.
- [ ] The diagnostic span covers the initializer expression or the `=` token.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend catch
cargo nextest run -p ts2wasm-cli -E 'test(catch) or test(exception)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/catchClauseWithInitializer1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/catchClauseWithInitializer1.ts --detail
```

Not run:

- none

## Notes

Split from generated bucket `issues/done/1123-implement-catchClauseWithInitializer.md`.

## Completion evidence

Fill when implemented.
