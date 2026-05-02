---
id: 5009
title: "Remaining static ES module export forms (named list, default import, namespace, re-export, side-effect)"
type: feature
area: ir/compiler
class: implementation-ready
priority: P1
depends_on: []
blocks: []
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
- [ ] Rewrite `ImportDefaultNamed`, `ImportNamespace`, `ImportSideEffect` forms
- [ ] Rewrite `ExportNamedFrom`, `ExportAllFrom`, `ExportNamespaceFrom` re-export forms
- [ ] Add Node/iwasm differential test coverage for each new form
- [ ] Narrow the `issue-055` catch-all in builtin_resolver.rs

## Acceptance criteria

- [x] `export { x, y }` builds to WASM and both names are accessible
- [x] `import x from "./mod"` builds to WASM and reads the default export
- [ ] `import * as ns from "./mod"` builds to WASM and `ns.x` accesses named exports
- [ ] `import "./side-effect"` triggers module initialization
- [ ] `export * from "./mod"` builds to WASM and forwards all named exports
- [ ] `export { x } from "./mod"` builds to WASM (re-export by alias)
- [ ] All previous `static-*-build-smoke` tests still pass
- [ ] issue-055 diagnostic expectations for now-handled forms are removed/narrowed

## Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli module
cargo nextest run -p ts2wasm-compiler
```
