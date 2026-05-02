---
id: 5009
title: "Remaining static ES module export forms (named list, default import, namespace, re-export, side-effect)"
type: feature
area: ir/compiler
class: done
priority: P1
depends_on: []
blocks: [5010]
status: done
created: 2026-05-02
updated: 2026-05-02
---

## Summary

Issue 5008 implemented `export const x = 1` (ExportDecl) and `export default <expr>` for entry modules without imports. The remaining static ES module export/import forms still hit `issue-055` unsupported diagnostics.

## Remaining forms

- `export { x, y }` / `export { x as y }` (named export lists in entry module)
- `import x from "./mod"` (default import from another module)
- `import x, { y } from "./mod"` (combined default + named import)
- `import * as ns from "./mod"` (namespace import)
- `import "./mod"` (side-effect import)
- `export * from "./mod"` (star re-export)
- `export { x } from "./mod"` (named re-export from)
- `export * as ns from "./mod"` (namespace re-export)

## Scope

- [x] Rewrite `ExportNamed` in `lower_static_named_import_bindings_for_build` for `export { x, y }`
- [x] Rewrite `ImportDefault` for `import x from "./mod"`
- [x] Rewrite `ImportNamespace` for `import * as ns from "./mod"`
- [x] Rewrite `ImportSideEffect` for `import "./mod"`
- Remaining forms (ImportDefaultNamed, re-exports, differential tests, issue-055 narrowing) tracked in issue 5010

## Acceptance criteria

- [x] `export { x, y }` builds to WASM and both names are accessible
- [x] `import x from "./mod"` builds to WASM and reads the default export
- [x] `import * as ns from "./mod"` builds to WASM and `ns.x` accesses named exports
- [x] `import "./side-effect"` triggers module initialization
- Remaining acceptance criteria tracked in issue 5010

## Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli module
cargo nextest run -p ts2wasm-compiler
```
