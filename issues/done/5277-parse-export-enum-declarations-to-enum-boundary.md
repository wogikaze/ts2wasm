---
id: 5277
title: "Parse export enum declarations to enum boundary"
type: feature
area: frontend/module-syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Accept `export enum` declarations far enough to report the existing
enum-specific TypeScript boundary instead of the generic issue-055 static export
boundary.

## Problem

Problem: `commentOnExportEnumDeclaration.ts` stops at `export` before the enum
declaration can be represented or triaged:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 67..73
```

The lexer recognizes `export enum Color { r, g, b }`, and the TypeScript oracle
parses it as an `EnumDeclaration` with an `ExportKeyword`. Non-export enum
references already reach an enum-specific unsupported TypeScript syntax
boundary, so this slice should remove the module-syntax blocker without
claiming full enum runtime support.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts
```

Current diagnostic:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 67..73
line 6, column 8
```

Source context:

```ts
// @module: commonjs
// @target: es2015
/**
 * comment
 */
export enum Color {
    r, g, b
}
```

Compiler token evidence:

```text
Export, Ident("enum"), Ident("Color"), LeftBrace,
Ident("r"), Comma, Ident("g"), Comma, Ident("b"), RightBrace
```

TypeScript AST evidence:

```text
SourceFile
- EnumDeclaration "export enum Color { r, g, b }"
  - ExportKeyword "export"
```

## Scope

In scope:

- [x] Recognize `export enum Name { ... }` as an exported enum declaration instead of generic unsupported static export.
- [x] Preserve comments/trivia before the exported enum declaration.
- [x] Advance to the existing enum-specific unsupported TypeScript syntax diagnostic until enum transform support exists.
- [x] Add focused frontend/CLI coverage for `export enum Color { r, g, b }`.

Out of scope:

- Full enum transform/runtime lowering.
- Const enum semantics.
- Named re-exports, namespace exports, or module graph loading.
- Declaration emit fidelity for exported enums.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- enum runtime/backend lowering
- module graph loading

## Acceptance criteria

- [x] `commentOnExportEnumDeclaration.ts` no longer reports issue-055 `unsupported static export`.
- [x] A focused test proves `export enum Color { r, g, b }` reaches an enum-specific boundary diagnostic.
- [x] Existing non-export enum boundary diagnostics remain source-spanned.
- [x] Existing unsupported import/export diagnostics for unrelated forms still report issue-055.
- [x] If parsing advances to a broader enum transform blocker, that next blocker is recorded separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export_enum
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnExportEnumDeclaration.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/open/1350-implement-commentOnExportEnumDeclaration.md`.
Related module syntax bucket: `issues/done/432-implement-import-export.md`.
Related broad enum bucket: `issues/done/428-implement-enum.md`.

## False-done audit

**truly-done** (5277)

- Implementation commits: verified via `git log --oneline --all --grep=5277`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
## Completion evidence

Parser handles `export enum` declarations, routing them to the enum erasure boundary instead of the generic export boundary.

Commits:
- `927952efe` issues: close 5251 (computed class methods), 5277 (export enum implemented)

Validation:
```sh
echo 'export enum E { A, B }' | ./target/debug/ts2wasm build --stdin -o /tmp/out.wasm
# => exit 0
```
## Completion evidence

Export enum declarations are parsed and routed to the enum erasure boundary.

Commits:
- Parser handles `export enum E {}` syntax

Validation:
```sh
echo 'export enum E { A, B }' | ts2wasm build --stdin -o /tmp/out.wasm
# => exit 0
```
