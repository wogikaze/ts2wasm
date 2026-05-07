---
id: 5401
title: "Parse export default interface declarations"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Parse TypeScript `export default interface Name { ... }` declarations instead
of treating `default` as an export-assignment expression followed by a stray
identifier.

## Problem

`allowImportClausesToMergeWithTypes.ts` currently parses the leading exported
const and `export default zzz;`, then fails on the default-exported interface:

```ts
export default interface zzz {
    x: string;
}
```

Problem: `export default interface zzz { ... }` stops with `expected Semicolon` before the frontend can represent the interface declaration.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts
```

Observed result:

```text
UnsupportedSyntax: expected Semicolon, got Some(Ident("zzz")) at 154..157
```

Source context:

```text
5 | export default zzz;
6 |
7 | // @filename: a.ts
8 | export default interface zzz {
9 |     x: string;
```

Compiler evidence:

```text
tokens: ok through Export, Default, Ident("interface"), Ident("zzz"), LeftBrace
ast: fails while parsing the default export before creating InterfaceDeclaration
TypeScript oracle: topLevel includes InterfaceDeclaration "export default interface zzz { ... }"
```

## Desired final state

The frontend represents default-exported interface declarations with enough
span information to advance `allowImportClausesToMergeWithTypes.ts` past the
current parser error to the next narrower duplicate-default-export, import, or
type diagnostic.

## Scope

In scope:

- [ ] Parse `export default interface Name { ... }` as an interface declaration with export/default modifiers.
- [ ] Preserve the interface name span and member spans.
- [ ] Add focused parser or CLI AST coverage for the exact declaration form.
- [ ] Re-triage `allowImportClausesToMergeWithTypes.ts` and record the next diagnostic.

Out of scope:

- Type checking interface merging with imported default names.
- Multiple default export diagnostics.
- Declaration emit fidelity.
- Module resolution for `./a` or `./b`.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/`
- focused parser or CLI AST tests

Do not touch:

- backend/runtime emit unless parsing cannot surface a controlled unsupported diagnostic
- package or virtual file module resolution

## Acceptance criteria

- [ ] `export default interface zzz { x: string; }` no longer reports `expected Semicolon` at the interface name.
- [ ] A focused parser or CLI AST test covers default-exported interface declarations.
- [ ] `env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts` advances past the current `154..157` parser error.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend interface
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from `issues/done/596-implement-allowImportClausesToMergeWithTypes.md`.

Related but not duplicates:

- `issues/open/2033-implement-duplicateDefaultExport.md` is a generated bucket
  for duplicate default export diagnostics, not the parser syntax needed here.
- `issues/open/1718-implement-declarationEmitDefaultExport-import-export.md`
  is a broad generated default export bucket and remains too wide to implement
  directly.

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
