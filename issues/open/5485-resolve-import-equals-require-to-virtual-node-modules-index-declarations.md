---
id: 5485
title: "Resolve import-equals require to virtual node_modules index d.ts"
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
`node_modules/<pkg>/index.d.ts` section in the same TypeScript reference file.

## Problem

`nodeResolution7.ts` contains:

```ts
// @filename: node_modules/a/index.d.ts
declare module "a" {
    var x: number;
}

// @filename: b.ts
import y = require("a");
```

Current module graph validation rejects `a` as an issue-232 non-local module
specifier instead of resolving the virtual index declaration file.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution7.ts
```

Expected current evidence:

```text
resolved: issue-232 unsupported non-local module specifier `a`
```

## Desired final state

The compiler resolves bare `import alias = require("a")` to the matching
virtual `node_modules/a/index.d.ts` section, or advances to a narrower
declaration/binding limitation.

## Scope

In scope:

- [ ] Register virtual `node_modules/<pkg>/index.d.ts` declaration sections.
- [ ] Resolve `import alias = require("pkg")` to that virtual declaration section.
- [ ] Add one focused module-graph regression for this exact shape.

Out of scope:

- `node_modules/<pkg>.d.ts`, tracked by `issues/open/5484-resolve-import-equals-require-to-virtual-node-modules-declarations.md`.
- Package `package.json` fields.
- Import maps.
- Runtime CommonJS emit.

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

- [ ] `nodeResolution7.ts` no longer reports issue-232 unsupported non-local module specifier `a`.
- [ ] A focused regression proves `import alias = require("a")` resolves to virtual `node_modules/a/index.d.ts`.
- [ ] Unmatched bare package specifiers still report issue-232.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(module) or test(resolution) or test(require)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution7.ts
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

- `issues/open/5484-resolve-import-equals-require-to-virtual-node-modules-declarations.md`
  covers direct `node_modules/<pkg>.d.ts` files.
- `issues/open/5295-resolve-import-equals-require-to-virtual-node-modules-class-export.md`
  covers virtual `node_modules/pkg/index.ts` class-export values.
- `issues/open/232-resolve-local-relative-es-module-graph.md` intentionally
  rejects unmatched bare specifiers and excludes package/node_modules lookup.

## Completion evidence

Fill when implemented.
