---
id: 5008
title: "Implement static ES module export forms (default, named, namespace, re-export) (audit reopened #5008)"
type: feature
area: ir/compiler
class: done
priority: P1
depends_on: []
blocks: [5009]
status: open
created: 2026-05-02
updated: 2026-05-05
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
- [ ] Rewrite `ExportDefault` (export default ...)
- Remaining forms (ExportNamed, ImportDefault, namespace, re-exports, side-effect) tracked in issue 5009

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
- [ ] All previous `static-named-import-build-smoke` tests still pass
- Remaining acceptance criteria tracked in issue 5009

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

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none
- [ ] created/updated: `issues/open/5010-remaining-es-module-export-forms.md`

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5008-static-es-module-export-default-namespace-reexport.md` before this move
- `issues/open/5008-static-es-module-export-default-namespace-reexport.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
