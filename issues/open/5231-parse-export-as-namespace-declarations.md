---
id: 5231
title: "Parse export as namespace declarations"
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

Parse TypeScript `export as namespace Name;` declarations as a distinct
module/ambient declaration shape instead of the generic static export boundary.

## Problem

`checkMergedGlobalUMDSymbol.ts` tokenizes `export as namespace THREE;`, but AST
construction stops at the `export` keyword with the generic issue-055 static
export diagnostic.

Problem: `export as namespace THREE;` reports `UnsupportedModule: issue-055: unsupported static export`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts
```

Source shape:

```ts
// @Filename: global.d.ts
import * as _three from './three';

export as namespace THREE;
```

Compiler evidence:

```text
tokens: ok; Export Ident("as") Ident("namespace") Ident("THREE") Semicolon
ast: UnsupportedModule issue-055 at export as namespace
TypeScript AST: NamespaceExportDeclaration
TypeScript oracle: TS1315 Global module exports may only appear in declaration files
```

## Desired final state

The frontend recognizes `export as namespace Name;` and preserves its span for
later module/ambient diagnostics instead of classifying it as a generic static
export.

## Scope

In scope:

- [ ] Parse `export as namespace Name;` as a namespace-export declaration or a precise unsupported module statement.
- [ ] Preserve the declaration span and exported namespace name.
- [ ] Add one focused parser/AST fixture for `export as namespace THREE;`.

Out of scope:

- Full UMD global merge semantics.
- Runtime export namespace emission.
- `declare global` erasure policy changes.
- Namespace body/module ownership semantics.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- `crates/frontend/src/ast.rs`
- focused parser or CLI AST tests

Do not touch:

- `crates/backend-wasm/`
- runtime module loading

## Acceptance criteria

- [ ] `checkMergedGlobalUMDSymbol.ts` no longer reports generic `issue-055: unsupported static export` for `export as namespace THREE;`.
- [ ] `noCrashUMDMergedWithGlobalValue.ts` no longer reports generic
  `issue-055: unsupported static export` for `export as namespace
  SomeInterface;`.
- [ ] A focused parser/AST test proves `export as namespace THREE;` is recognized and span-bearing.
- [ ] Existing unsupported static export diagnostics still apply to unrelated unsupported export forms.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli --test parser_ast_structures
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/open/1140-implement-checkMergedGlobalUMDSymbol.md`.

Also owns `issues/open/3527-implement-noCrashUMDMergedWithGlobalValue.md`:
fresh triage stops before AST at `export as namespace SomeInterface;` with the
same generic `issue-055: unsupported static export` boundary. TypeScript parses
the statement as `NamespaceExportDeclaration` and reports TS1315 in this
reference shape.

## Completion evidence

Fill when implemented.
