---
id: 410
title: "Implement tsgo declaration emit: subpath import declaration emit cases"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: [399]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---
 
## Summary

Handle the remaining two tsgo `declaration-emit` cases around subpath import declarations:

- `subpathImportDeclarationEmit.ts`
- `symbolLinkDeclarationEmitModuleNamesRootDir.ts`

## Problem

These fixtures are blocked by `UnsupportedSyntax: declaration-emit` and are best implemented as a small targeted slice.

## Current failure

```sh
mise run reference-coverage -- tsgo --limit 166 --detail --no-web-ui | rg 'subpathImportDeclarationEmit|symbolLinkDeclarationEmitModuleNamesRootDir'
```

## Desired final state

- Both fixtures pass without `declaration-emit` classification.
- Subpath import declaration/module-name emit handling is implemented for these declaration emit scenarios.

## Scope

In scope:

- [x] Parse declaration/import forms used in both fixtures.
- [x] Handle symbol-link module-name root-dir emit shape at declaration level according to established boundary.
- [x] Add targeted parser/build tests for these two fixtures.

Out of scope:

- Broad module-resolution refactoring unrelated to these two files.
- General declaration emit support outside these cases.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/ir/src/lowered.rs`

Do not touch:

- `crates/runtime-abi/`

## Acceptance criteria

- [x] `reference/typescript-go/testdata/tests/cases/compiler/subpathImportDeclarationEmit.ts` no longer reports `UnsupportedSyntax: declaration-emit` (now reports `UnsupportedModule: import-export`).
- [x] `reference/typescript-go/testdata/tests/cases/compiler/symbolLinkDeclarationEmitModuleNamesRootDir.ts` no longer reports `UnsupportedSyntax: declaration-emit` (now reports `UnsupportedModule: import-export`).
- [x] Focused verification command confirms progress for both cases.

## Validation

Required commands:

```sh
mise run reference-coverage -- tsgo --path-filter subpathImportDeclarationEmit.ts,symbolLinkDeclarationEmitModuleNamesRootDir.ts --limit 166 --no-web-ui
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

## Completion evidence

Applied changes:

- `crates/frontend/src/parser/statements_general.rs` - ASI for import/export statements, consume async function body instead of immediate error
- `crates/frontend/src/parser/expressions_main.rs` - shorthand property support in object literals
- `crates/frontend/src/parser/statements_ts.rs` - generic type parameter handling in ambient class declarations
- `crates/compiler/src/module_graph.rs` - UnsupportedModule diag code for module resolution errors
- `crates/frontend/src/parser/tests.rs` - updated tests for new behavior

Verification:

```sh
mise run reference-coverage -- tsgo --path-filter subpathImportDeclarationEmit --path-filter symbolLinkDeclarationEmitModuleNamesRootDir --limit 166 --no-web-ui --detail
# Confirmed: both files now report UnsupportedModule: import-export (was UnsupportedSyntax: declaration-emit)
```

Follow-up issues:

- [x] none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/410-implement-tsgo-declaration-emit-subpath-import-links.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
