---
id: 5484
title: "Resolve import-equals require to virtual node_modules d.ts file"
type: feature
area: compiler/module-graph
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Resolve `import alias = require("pkg")` to a virtual
`node_modules/<pkg>.d.ts` section in the same TypeScript reference file.

## Problem

`nodeResolution5.ts` contains this declaration-file package shape:

```ts
// @filename: node_modules/a.d.ts
declare module "a" {
    var x: number;
}

// @filename: b.ts
import y = require("a");
```

Fresh triage reaches the module graph, then rejects `a` as an unsupported
non-local specifier:

```text
UnsupportedModule: issue-232: unsupported non-local module specifier `a`; package resolution, import maps, and absolute specifiers are not implemented
```

Problem: `import = require("pkg")` does not search virtual
`node_modules/<pkg>.d.ts` sections provided by the same reference file.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution5.ts
```

Observed evidence:

```text
nodeResolution5.ts:
  ast: ok; ImportDefault source "a"
  resolved: issue-232 unsupported non-local module specifier `a`
```

## Desired final state

The compiler resolves bare `import alias = require("a")` from a reference
`@filename` section to the matching virtual `node_modules/a.d.ts` section.

If the declaration contents remain unsupported after resolution, the next
diagnostic must point at that narrower declaration or binding limitation, not
the generic issue-232 non-local specifier boundary.

## Scope

In scope:

- [ ] Register virtual `node_modules/<pkg>.d.ts` declaration sections for package lookup.
- [ ] Resolve `import alias = require("pkg")` to that virtual declaration section.
- [ ] Add one focused module-graph regression for this exact shape.

Out of scope:

- Package `package.json` fields.
- Import maps.
- Runtime CommonJS emit.
- `node_modules/<pkg>/index.d.ts`, tracked by `issues/open/5485-resolve-import-equals-require-to-virtual-node-modules-index-declarations.md`.
- Class-export package shapes already covered by `issues/open/5295-resolve-import-equals-require-to-virtual-node-modules-class-export.md`.
- Static ES import/export virtual-section resolution covered by `issues/open/5229-resolve-imports-between-filename-sections.md`.
- `export var` parser boundaries covered by `issues/done/5283-support-entry-export-var-declarations.md` and `issues/done/5285-support-export-var-initializer-declarations.md`.

## Affected paths

Expected:

- `crates/compiler/src/module_graph.rs`
- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch:

- backend wasm lowering
- package.json resolver semantics
- unrelated import/export parser forms

## Acceptance criteria

- [ ] `nodeResolution5.ts` no longer reports issue-232 unsupported non-local module specifier `a`.
- [ ] A focused regression proves `import alias = require("a")` resolves to virtual `node_modules/a.d.ts`.
- [ ] Unmatched bare package specifiers still report issue-232.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(module) or test(resolution) or test(require)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution5.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeResolution --detail --no-dashboard-data
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

Split from `issues/open/3590-implement-nodeResolution.md`.

Related but not duplicates:

- `issues/open/5295-resolve-import-equals-require-to-virtual-node-modules-class-export.md`
  covers `import = require("pkg")` for virtual `node_modules/pkg/index.ts`
  with exported class values.
- `issues/open/5485-resolve-import-equals-require-to-virtual-node-modules-index-declarations.md`
  covers `node_modules/<pkg>/index.d.ts`.
- `issues/open/5229-resolve-imports-between-filename-sections.md` covers
  local static import/export source specifiers between virtual sections.
- `issues/open/232-resolve-local-relative-es-module-graph.md` intentionally
  rejects unmatched bare specifiers and excludes package/node_modules lookup.

## Completion evidence

Fill when implemented.
