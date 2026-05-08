---
id: 5421a
title: "Resolve classic moduleResolution bare imports to virtual sections"
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

Resolve TypeScript reference `@moduleResolution: classic` bare imports against
virtual `// @Filename:` sections in the same reference file.

## Problem

`moduleResolution_classicPrefersTs.ts` contains virtual files:

```ts
// @Filename: /dir1/dir2/dir3/a.js
export default "dir1/dir2/dir3/a.js";

// @Filename: /dir1/dir2/a.ts
export default "dir1/dir2/a.ts";

// @Filename: /dir1/dir2/dir3/index.ts
import a from "a";
```

Problem: module graph validation reports `issue-232: unsupported non-local
module specifier a` instead of resolving the classic bare specifier through the
virtual section set.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_classicPrefersTs.ts
```

Current evidence:

```text
tokens: ok; export default string sections and import a from "a" are tokenized
ast: ok; two ExportDefault nodes and ImportDefault source "a"
module_graph: issue-232 unsupported non-local module specifier `a`
TypeScript oracle: TS2528 duplicate default exports in raw-source view and TS2307 for `a`
```

## Desired final state

The reference runner's virtual file/module graph support can apply classic
module resolution for bare import `a` from `/dir1/dir2/dir3/index.ts`, including
preferring the virtual `/dir1/dir2/a.ts` section over `/dir1/dir2/dir3/a.js`.

## Scope

In scope:

- [ ] Register virtual `// @Filename:` sections for classic resolution.
- [ ] Resolve bare `import a from "a"` under `@moduleResolution: classic`.
- [ ] Preserve TypeScript classic preference for `.ts` over nearby `.js` in this reference shape.
- [ ] Add one focused module-graph or reference regression for this exact shape.

Out of scope:

- Node/package `node_modules` traversal.
- package.json fields, import maps, or modern NodeNext resolution.
- Full declaration emit or noEmit parity.
- Broad bare-package resolution outside `@moduleResolution: classic`.

## Affected paths

Expected:

- `crates/compiler/src/module_graph.rs`
- `crates/compiler/src/lib.rs`
- focused module-graph tests or fixtures

Do not touch:

- backend wasm lowering
- unrelated parser syntax

## Acceptance criteria

- [ ] `reference-triage` for `moduleResolution_classicPrefersTs.ts` no longer reports `issue-232: unsupported non-local module specifier a`.
- [ ] A focused regression proves `import a from "a"` from `/dir1/dir2/dir3/index.ts` resolves to virtual `/dir1/dir2/a.ts`.
- [ ] Existing unsupported bare package specifiers without classic virtual-section matches still report issue-232.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(module) or test(resolution) or test(filename)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_classicPrefersTs.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_classicPrefersTs.ts --detail --no-dashboard-data
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

Split from `issues/open/3370-implement-moduleResolution-import-export.md`.

## Completion evidence

Fill only when moving to `done`.
