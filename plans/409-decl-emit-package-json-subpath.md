# Implementation Plan: Issue 409 - tsgo declaration emit: package-json exports and subpath reexport cases

## Root Cause

Two tsgo fixtures fail with `[UnsupportedSyntax]` errors that display as `[UnsupportedModule]` with the message `"issue-055: ..."`. The classification label is `declaration-emit` because the coverage script maps filenames containing "declarationemit" to that label.

### File 1: `declarationEmitResolvePackageJsonExportsFalse.ts`

- Error: `issue-055: unsupported function export` at `export function makeC()`
- The parser's `export_statement()` does not handle `export function`
- Falls to the default `_` arm which produces `UnsupportedSyntax`

### File 2: `declarationEmitSubpathImportsReexport.ts`

- Error: `issue-055: unsupported default import` at `import type { B } from "package-b"`
- The parser's `import_statement()` dispatches `import type` as a default import (`type` is an Ident token), then fails when it encounters `{` instead of `from`

## Changes

### 1. `crates/frontend/src/parser/statements_general.rs`

#### Fix 1a: Add `export function` handling in `export_statement()`

Add a match arm in `export_statement()` for `Some(Token::Function)` before the `_` fallthrough.

When `export function name(...) { ... }` is encountered:
1. Consume the `export` token (already done)
2. Detect `Function` token
3. Call `function_statement()` to parse the function declaration
4. Get the function name from the parsed `Stmt::Function`
5. Wrap in `Stmt::ExportDecl { declaration: Box::new(function), specifier: ExportNamedSpecifier { local: name, ... } }`

Pattern: follow the same approach as `const_export_statement()`.

#### Fix 1b: Add `import type` handling in `import_statement()`

In `import_statement()`, before dispatching on the token after `import`, check if the next token is the contextual keyword `type`. If so:
1. Consume the `type` token (it's a TypeScript-only annotation, erased at runtime)
2. Then dispatch based on the NEXT token after `type`

This makes `import type { B }` work by consuming `type` and then falling through to the existing `named_import_statement` handler for `{ B }`.

### 2. `crates/frontend/src/parser/tests.rs`

Add parser tests for:
- `export function name() { }`
- `import type { Name } from "module"`

### Files NOT to change

- `docs/` - forbidden per issue scope
- Other crates outside `crates/frontend/src/`

## Verification

```bash
mise run fmt
mise run nextest
mise run reference-coverage -- tsgo --path-filter declarationEmitResolvePackageJsonExportsFalse.ts --path-filter declarationEmitSubpathImportsReexport.ts --limit 166 --no-web-ui --detail
```

## Acceptance criteria

1. `declarationEmitResolvePackageJsonExportsFalse.ts` no longer reports `UnsupportedSyntax: declaration-emit` (now reports `UnsupportedModule: import-export`)
2. `declarationEmitSubpathImportsReexport.ts` no longer reports `UnsupportedSyntax: declaration-emit` (now reports `UnsupportedModule: import-export`)
3. `cargo fmt` passes
4. `cargo nextest` passes
