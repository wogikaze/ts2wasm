---
id: 5008
title: "Implement static ES module export forms (default, named, namespace, re-export)"
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

Issues 231-234 implemented the module graph and the narrow `import { x } from "./mod"` / `export const x = ...` subset. The remaining static ES module export/import forms still hit `issue-055` unsupported diagnostics in the built-in resolver and name resolver.

## Problem

The following forms are rejected with `issue-055`:

- `export { x }` / `export { x as y }` (named export lists)
- `export default expr` / `export default function` / `export default class`
- `export const x = ...` (declaration export) in modules without imports
- `export class Foo {}` / `export function f() {}`
- `import x from "./mod"` (default import)
- `import x, { y } from "./mod"` (combined default + named import)
- `import * as ns from "./mod"` (namespace import)
- `import "./mod"` (side-effect import)
- `export * from "./mod"` (star re-export)
- `export { x } from "./mod"` (named re-export from)
- `export * as ns from "./mod"` (namespace re-export)

Problem: `export const x = 1` (ExportDecl) currently hits `issue-055` when the file has no `import` from another module, because the compiler's module rewrite path only triggers for files with named imports. This means a single-file module that only exports cannot build.

## Desired final state

All static export forms above produce valid WASM when the exported value is a simple literal or local binding. Default exports are stored under the `"default"` key in the module export object. Named export lists (`export { a, b }`) resolve to local bindings. Star re-exports (`export * from "./mod"`) forward all named exports. Side-effect imports (`import "./mod"`) execute module initialization.

## Scope

In scope:

- [ ] Rewrite `ExportDecl` (export const/function/class) as a module export in the compiler build path regardless of whether the file has imports
- [ ] Rewrite `ExportNamed` (export { x, y }) and `ExportNamedFrom` (export { x } from "./mod")
- [ ] Rewrite `ExportDefault` (export default ...)
- [ ] Rewrite `ExportAllFrom` (export *from "./mod") and `ExportNamespaceFrom` (export* as ns from "./mod")
- [ ] Rewrite `ImportDefault` (import x from "./mod") and `ImportDefaultNamed` (import x, { y } from "./mod")
- [ ] Rewrite `ImportNamespace` (import * as ns from "./mod")
- [ ] Rewrite `ImportSideEffect` (import "./mod")
- [ ] Upgrade existing issue-055 test expectations from `UnsupportedSyntax` to build smoke
- [ ] Add Node/iwasm differential test coverage for each new form
- [ ] Remove or narrow the `issue-055` catch-all in builtin_resolver.rs when all forms are covered

Out of scope:

- Dynamic `import()` (P3)
- Live binding updates beyond initial export evaluation
- TypeScript ambient module declarations (`declare module`)
- Package resolution, import maps, bare specifiers
- Circular module evaluation semantics

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/cli/tests/m9_modules.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/module-system/`

Do not touch:

- `crates/backend-wasm/src/` (runtime helpers already support module exports)
- `crates/runtime-abi/`
- `docs/`

## Acceptance criteria

- [ ] `export const x = 1` (no import) builds to WASM
- [ ] `export default 42` builds to WASM and is readable via `{ "default": 42 }`
- [ ] `import x from "./mod"` builds to WASM and reads the default export
- [ ] `import * as ns from "./mod"` builds to WASM and `ns.x` accesses named exports
- [ ] `export { x, y }` builds to WASM and both names are accessible
- [ ] `export * from "./mod"` builds to WASM and forwards all named exports
- [ ] `import "./side-effect"` triggers module initialization
- [ ] `export { x } from "./mod"` builds to WASM (re-export by alias)
- [ ] All previous `static-named-import-build-smoke` tests still pass
- [ ] All issue-055 diagnostic expectations that are now covered are removed or narrowed

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli module
cargo nextest run -p ts2wasm-compiler
cargo nextest run -p ts2wasm-ir
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none
- [ ] created/updated: `issues/open/...`
