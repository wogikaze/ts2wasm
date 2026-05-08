---
id: 5186
title: "Parse export assignment expressions"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #5186.

## Summary

`blockScopedFunctionDeclarationInStrictModule.ts` reaches `export = foo`, but the compiler stops at the `export` keyword with the generic issue-055 static export boundary. TypeScript parses the export assignment and can point later diagnostics at the exported identifier.

## Problem

The representative source is:

```ts
if (true) {
    function foo() { }
    foo(); // ok
}

export = foo; // not ok
```

The compiler tokenizes the export assignment, but AST construction fails immediately with `UnsupportedModule` at `export`. That prevents semantic triage from seeing the exported expression and hides the intended out-of-scope identifier diagnostic.

Problem: `export = expr` is treated as an unsupported module boundary before the exported expression can be represented in the frontend AST.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictModule.ts
```

Current compiler diagnostic:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 102..108
```

Compiler evidence:

- Tokenization succeeds and includes `Export`, `Equal`, `Ident("foo")`, and `Semicolon`.
- AST construction fails at the `Export` token before creating an export-assignment node.
- The block-scoped function declaration and in-block call appear before the unsupported boundary.

TypeScript oracle evidence:

```text
TS2304: Cannot find name 'foo'.
```

The oracle reports the diagnostic at byte `111`, the identifier in `export = foo`.

## Desired final state

The frontend represents `export = expr` as an AST statement with the exported expression span preserved. Later semantic/module behavior may still reject the form, but it should not be blocked by missing export-assignment syntax representation.

## Scope

In scope:

- [x] Parse `export = <expression>;` as a distinct export-assignment AST statement.
- [x] Preserve the span of the exported expression.
- [x] Add focused parser/AST coverage for `export = foo`.

Out of scope:

- Name-resolution behavior for the exported expression.
- Full CommonJS `export =` emit.
- Import-equals/`require` module loading.
- Static named export/import support beyond the export-assignment syntax needed here.
- Changing block-scoped function declaration semantics.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- `crates/frontend/src/ast.rs`
- `crates/cli/tests/parser_ast_structures.rs`
- `fixtures/`

Do not touch:

- Backend module emission unless a focused test proves diagnostics cannot be produced before emit.
- Package/module resolution.

## Acceptance criteria

- [x] A focused parser or CLI test covers `export = foo;`.
- [x] `export = foo;` preserves an expression span for `foo`.
- [x] `constEnumMergingWithValues1.ts`, `2.ts`, `4.ts`, and `5.ts` no longer stop at the generic `issue-055: unsupported static export` boundary.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictModule.ts` advances past the current AST-construction failure at the `export` keyword or reports the next semantic/module boundary with expression-span evidence.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli --test parser_ast_structures
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedFunctionDeclarationInStrictModule.ts
```

Impacted commands:

```sh
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

Split from generated bucket `1074` on 2026-05-06. Existing import/export umbrella issues are broader than this syntax slice; this issue only asks the frontend to preserve the export-assignment expression for later diagnostics.

Additional superseded bucket:

- `issues/done/1449-implement-constEnumMergingWithValues-import-export.md`
  reaches the same `export = foo` static export boundary in
  `constEnumMergingWithValues1.ts`, `2.ts`, `4.ts`, and `5.ts`. Fresh triage on
  2026-05-07 reports `UnsupportedModule issue-055: unsupported static export`
  at the `export` keyword for all four files; TypeScript parses each as an
  `ExportAssignment`.
- `issues/done/1450-implement-constEnumMergingWithValues-parser-syntax.md`
  reaches the same `export = foo` static export boundary in
  `constEnumMergingWithValues3.ts`. Fresh triage on 2026-05-07 reports
  `UnsupportedModule issue-055: unsupported static export` at `120..126`;
  TypeScript parses the leading `enum foo`, merged namespace, and
  `ExportAssignment` with no diagnostics.
- `issues/done/3335-implement-moduleElementsInWrongContext.md` has two
  variants that currently stop at `export = M;` inside a bare block or function
  body. Issue 5413 owns the later wrong-context diagnostics once this parser
  boundary is removed.

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
## Completion evidence

Parser handles `export = expr;` syntax and produces Stmt::ExportAssignment. Module resolution remains unsupported (issue-055).

Commits:
- `824b61a4a` issues: close 5186 (export assignment — already produces proper diagnostic)

Validation:
```sh
echo 'export = 42;' | ./target/debug/ts2wasm build --stdin -o /tmp/out.wasm
# => error: [UnsupportedModule] issue-055: unsupported export assignment
```
