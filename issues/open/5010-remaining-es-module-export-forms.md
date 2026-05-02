---
id: 5010
title: "Combined import and re-export forms (ImportDefaultNamed, ExportNamedFrom, ExportAllFrom, ExportNamespaceFrom)"
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

Issues 5008-5009 implemented the simpler export forms (ExportDecl, ExportDefault, ExportNamed, ImportDefault, ImportNamespace, ImportSideEffect). The remaining combined import and re-export forms still need implementation.

## Remaining forms

- `import x, { y } from "./mod"` (combined default + named import)
- `export * from "./mod"` (star re-export)
- `export { x } from "./mod"` (named re-export from)
- `export * as ns from "./mod"` (namespace re-export)

## Scope

- [x] Rewrite `ImportDefaultNamed` for `import x, { y } from "./mod"`
- [x] Rewrite `ExportAllFrom` for `export * from "./mod"`
- [ ] Rewrite `ExportNamedFrom` for `export { x } from "./mod"`
- [ ] Rewrite `ExportNamespaceFrom` for `export * as ns from "./mod"`
- [ ] Add Node/iwasm differential test coverage for new forms
- [ ] Narrow the `issue-055` catch-all in builtin_resolver.rs

## Acceptance criteria

- [x] `import x, { y } from "./mod"` builds to WASM
- [x] `export * from "./mod"` builds to WASM and forwards named exports
- [ ] `export { x } from "./mod"` builds to WASM (re-export by alias)
- [ ] `export * as ns from "./mod"` builds to WASM
- [ ] All previous `static-*-build-smoke` tests still pass

## Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli module
cargo nextest run -p ts2wasm-compiler
```
