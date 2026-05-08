---
id: 5430
title: "Parse exported import-equals require declarations"
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

Parse TypeScript `export import Name = require("...")` declarations instead of
stopping at the generic issue-055 static export boundary.

## Problem

`multiImportExport.ts` tokenizes CommonJS import-equals declarations and the
following uses, then stops at an exported import-equals declaration whose target
is an external `require(...)` reference.

Problem: `export import Math = require("./Math/Math")` currently reports a
generic unsupported static export before the parser can preserve the
ImportEqualsDeclaration shape.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiImportExport.ts
```

Observed result:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 198..204
```

Source context:

```ts
// @Filename: Drawing.ts
export import Math = require('./Math/Math')
```

Compiler evidence:

```text
tokens: ok through Export, Import, Ident("Math"), Equal, require("./Math/Math")
ast/resolved: fails at the export keyword before representing the exported import-equals declaration
TypeScript oracle: topLevel includes ImportEqualsDeclaration "export import Math = require('./Math/Math')"
```

## Desired final state

The frontend represents exported import-equals declarations with external
`require(...)` targets and advances `multiImportExport.ts` past the current
generic static export boundary to the next narrower module loading, export
assignment, or semantic diagnostic.

## Scope

In scope:

- [ ] Parse `export import name = require("specifier")` as an exported
      import-equals declaration.
- [ ] Preserve the exported flag, alias name span, module specifier span, and
      external-module-reference shape.
- [ ] Cover the top-level form from `multiImportExport.ts`.
- [ ] Re-triage `multiImportExport.ts` and record the next diagnostic.

Out of scope:

- Resolving the `require(...)` module specifier.
- CommonJS module loading or runtime execution.
- Parsing `export import name = qualified.name`, tracked by issue 5400.
- CommonJS `export = expr` parsing, tracked by issue 5346.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/`
- focused parser or CLI AST tests

Do not touch:

- package or node module resolution
- backend/runtime CommonJS emit
- unrelated static ES module forms

## Acceptance criteria

- [ ] `export import Math = require("./Math/Math")` no longer reports
      `issue-055: unsupported static export` as its first blocker.
- [ ] A focused parser or CLI AST test covers exported import-equals with a
      `require("...")` target.
- [ ] `multiImportExport.ts` advances past the current `198..204` static export
      boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend import
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiImportExport.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiImportExport.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/open/3403-implement-multiImportExport.md`.

Related but distinct:

- `issues/open/5400-parse-exported-import-equals-declarations.md` covers the
  qualified target form `export import name = alias.Name`.
- `issues/open/5346-parse-commonjs-export-assignment-statements.md` owns
  `export = expr`.

## Completion evidence

Fill when implemented.
