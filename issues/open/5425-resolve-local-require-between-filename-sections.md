---
id: 5425
title: "Resolve local require between @Filename sections"
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

Resolve local CommonJS `require("./...")` calls to sibling virtual files from
TypeScript reference `// @filename:` sections.

## Problem

`moduleResolutionWithRequire.ts` contains:

```ts
// @filename: /other.ts
export const other = 123;

// @filename: /index.ts
declare const require: any;
function foo() {
    const { other }: { other: string } = require('./other');
}
```

The AST and module graph advance past parsing, but lowered IR validation fails
because the `require("./other")` module load references a module id that is not
present in the program module list.

Problem: local CommonJS require calls can create dangling `ModuleLoad`
references for virtual `@Filename` sections.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithRequire.ts
```

Observed result:

```text
UnsupportedModule: ModuleLoad references module_id 1 which is not in the program's module list
```

Evidence:

```text
tokens: ok
ast: ok; ExportDecl `other`, Function `foo`, Let `a = require("../outside-of-rootdir/foo")`, Let `{other} = require("./other")`
resolved/lowered: validate_lowered rejects a dangling ModuleLoad module_id
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

The compiler resolves `require("./other")` from `/index.ts` to the virtual
`/other.ts` section or reports a narrow supported diagnostic before lowering.
It must not emit invalid lowered IR with a dangling `ModuleLoad`.

## Scope

In scope:

- [ ] Register local `require("./other")` targets from virtual `@Filename` sections.
- [ ] Keep lowered `ModuleLoad` ids consistent with the program module list.
- [ ] Add one focused regression for `require("./other")` across two virtual sections.

Out of scope:

- Static `import` / `export ... from` virtual section resolution, covered by issue 5229.
- Bare package `require("pkg")` and node_modules resolution, covered by issue 5295.
- CommonJS `module.exports` execution semantics.
- Out-of-root `require("../outside-of-rootdir/foo")` behavior.

## Affected paths

Expected:

- `crates/compiler/src/module_graph.rs`
- `crates/ir/src/lowered/`
- focused compiler tests or fixtures

Do not touch:

- backend/runtime emit unless validation requires an error-shaping hook
- broad package resolution

## Acceptance criteria

- [ ] `moduleResolutionWithRequire.ts` no longer reports `ModuleLoad references module_id 1 which is not in the program's module list`.
- [ ] A focused test proves `require("./other")` resolves to a sibling virtual `@Filename` section without dangling lowered module ids.
- [ ] Existing static import virtual-section behavior remains owned by issue 5229 and is not broadened here.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(module) or test(filename) or test(require)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithRequire.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithRequire.ts --detail --no-dashboard-data
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

Split from `issues/done/3383-implement-moduleResolutionWithRequire.md`.
Also owns `issues/open/3384-implement-moduleResolutionWithRequireAndImport.md`:
fresh triage for `moduleResolutionWithRequireAndImport.ts` parses the
`typeof import("./other")` type annotation as an erased `null` initializer,
then reaches `require("./other")` and fails lowered validation with
`ModuleLoad references module_id 1 which is not in the program's module list`.

Related but not duplicates:

- `issues/open/5229-resolve-imports-between-filename-sections.md` covers
  static import/export source specifiers.
- `issues/open/5295-resolve-import-equals-require-to-virtual-node-modules-class-export.md`
  covers `import alias = require("pkg")` through virtual node_modules.
- `issues/open/5414-classify-non-builtin-require-result-method-calls.md`
  covers method calls on non-builtin require results after binding, not
  dangling ModuleLoad ids.

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
