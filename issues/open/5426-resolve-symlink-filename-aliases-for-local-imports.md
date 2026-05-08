---
id: 5426
title: "Resolve @symlink filename aliases for local imports"
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

Register TypeScript reference `// @symlink:` aliases as virtual module paths
for local imports.

## Problem

`moduleResolutionWithSymlinks_notInNodeModules.ts` contains:

```ts
// @filename: /shared/abc.ts
// @symlink: /src/shared/abc.ts,/src/shared2/abc.ts
export const x = 0;

// @filename: /src/app.ts
import { x } from "./shared/abc";
import { x as x2 } from "./shared2/abc";
x + x2;
```

The compiler does not resolve those imports through the `@symlink` aliases, so
the runner currently reports unresolved imported names and does not reach the
intended symlink module-resolution behavior.

Problem: `@symlink` aliases are not registered as virtual module paths for
local relative imports.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_notInNodeModules.ts
```

Observed result:

```text
UnresolvedName: unresolved name `x`
```

Evidence:

```text
tokens: ok; export const x, two imports from ./shared/abc and ./shared2/abc
TypeScript oracle: reports missing modules for ./shared/abc and ./shared2/abc in raw-source view
runner dumps: later virtual /src/tsconfig.json parsing is a separate boundary owned by issue 5292
```

## Desired final state

The module graph registers `/src/shared/abc.ts` and `/src/shared2/abc.ts` as
aliases for the virtual `/shared/abc.ts` section, so local imports from
`/src/app.ts` do not produce unresolved imported names.

## Scope

In scope:

- [ ] Parse `@symlink` directives attached to a virtual `@filename` section.
- [ ] Register each symlink path as a virtual module path alias for local imports.
- [ ] Add one focused regression with two symlink aliases pointing at one source section.

Out of scope:

- Node_modules realpath semantics for package symlinks.
- `preserveSymlinks` behavior.
- Virtual `tsconfig.json` section skipping, covered by issue 5292.
- Dependency `export class` handling, covered by issue 5324.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/compiler/src/module_graph.rs`
- focused compiler tests or fixtures

Do not touch:

- backend/runtime emit
- broad package resolution

## Acceptance criteria

- [ ] `moduleResolutionWithSymlinks_notInNodeModules.ts` no longer reports `UnresolvedName` for imported `x`.
- [ ] A focused regression proves two `@symlink` aliases can resolve to one virtual source section.
- [ ] If the next blocker is the virtual `/src/tsconfig.json` section, it is reported separately and remains owned by issue 5292.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(module) or test(filename) or test(symlink)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_notInNodeModules.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_notInNodeModules.ts --detail --no-dashboard-data
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

Split from `issues/done/3387-implement-moduleResolutionWithSymlinks-import-export.md`.

Related but not duplicates:

- `issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`
  covers the later virtual `/src/tsconfig.json` body.
- `issues/open/5324-support-dependency-export-class-declarations.md` covers
  the in-node_modules symlink cases' current dependency `export class` boundary.
- `issues/open/5229-resolve-imports-between-filename-sections.md` covers
  ordinary `@Filename` sibling imports, not `@symlink` aliases.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
