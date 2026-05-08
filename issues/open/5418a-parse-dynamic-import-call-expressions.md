---
id: 5418a
title: "Parse dynamic import call expressions"
type: feature
area: frontend/module-syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Accept `import("./b")` as an expression form in the frontend parser and AST,
starting with `moduleNoneDynamicImport.ts`.

## Problem

`moduleNoneDynamicImport.ts` tokenizes the dynamic import call correctly, but
the expression parser treats the `Import` token as unsupported when it appears
in expression position.

Problem: dynamic import calls such as `const foo = import("./b");` are rejected
before module resolution or export-default diagnostics become reachable.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts --detail --no-dashboard-data
```

Observed coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts: UnsupportedSyntax: import-export
```

Current diagnostic:

```text
UnsupportedModule: unsupported expression: Some(SpannedToken { kind: Import, span: Span { start: 112, end: 118 } }) at 118..119
```

Source context:

```ts
// @filename: /a.ts
const foo = import("./b");

// @filename: /b.js
export default 1;
```

Compiler evidence:

```text
tokens: ok; Import, LeftParen, String("./b"), RightParen, Semicolon are present.
visible symbols: []
ast/resolved: fail before representing the initializer expression.
```

TypeScript oracle:

```text
Top level includes `const foo = import("./b");` and `export default 1;`.
The binding hint for `foo` is `Promise<any>`, and the current TypeScript
diagnostic is TS2307 for missing `./b`.
```

## Desired final state

The parser represents `import("./b")` as a dynamic import call expression,
preserving the callee/import token span, string specifier expression, call
arguments, and enclosing variable initializer.

## Scope

In scope:

- [ ] Parse `import("specifier")` in expression position as a dynamic import
      call expression.
- [ ] Add focused frontend parser or module syntax tests for a string-literal
      dynamic import initializer.
- [ ] Re-run the representative triage and record the next diagnostic.

Out of scope:

- Promise runtime semantics for dynamic import.
- Resolving the imported module specifier.
- Loading or executing dynamically imported modules.
- `import()` with nested `await`, `yield`, trailing commas, or non-string
  specifier evaluation.
- `export default` lowering in the imported `.js` section.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- focused frontend parser/module syntax tests

Do not touch:

- backend/runtime dynamic module loading
- package or virtual-section module resolution
- Node/module emit behavior

## Acceptance criteria

- [ ] `const foo = import("./b");` parses into an initializer expression that
      preserves the specifier string and source spans.
- [ ] `moduleNoneDynamicImport.ts` advances past the current dynamic-import
      parser blocker or records the next narrower diagnostic without regressing
      ordinary static `import` declarations.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend import
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoneDynamicImport.ts --detail --no-dashboard-data
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

Split from `issues/open/3355-implement-moduleNoneDynamicImport.md`.

Related but not duplicates:

- Existing generated dynamic import buckets such as
  `issues/open/2068-implement-dynamicImportEvaluateSpecifier.md`,
  `issues/open/2069-implement-dynamicImportInDefaultExportExpression.md`, and
  `issues/open/2070-implement-dynamicImportTrailingComma.md` are broader or
  untriaged buckets. This issue owns the first shared expression-position
  parser blocker.

## Completion evidence

Fill when implemented.
