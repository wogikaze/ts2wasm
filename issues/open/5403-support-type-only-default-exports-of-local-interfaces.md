---
id: 5403
title: "Support type-only default exports of local interfaces"
type: feature
area: frontend/name-resolution
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Allow `export default Name;` to reference a local interface declaration in
declaration-oriented TypeScript reference inputs without reporting an
unresolved value name.

## Problem

`allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts` now parses its
module declarations, but fresh triage stops on the local interface name used by
the default export:

```ts
interface Color {
    c: string;
}
export default Color;
```

Problem: the resolver treats `export default Color;` as a value export
expression, but `Color` is a local type-only interface binding in this
declaration-oriented reference input.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts
```

Observed result:

```text
error: [UnresolvedName] unresolved name: `Color` at 50..55
```

Compiler evidence:

```text
tokens: ok for `interface Color`, `export default Color`, default import, named import, and `export const A = styled()`
ast: contains ExportDefault Ident("Color") but does not preserve an interface binding for name resolution
resolved: later module_graph output reaches missing virtual local module `./color`
```

## Desired final state

This representative no longer reports `UnresolvedName: Color` and advances to
the next narrower virtual-file module-resolution or declaration emit blocker.

## Scope

In scope:

- [ ] Preserve local interface declarations enough for default-export resolution.
- [ ] Allow `export default InterfaceName;` when the name is a local type-only interface binding.
- [ ] Add focused resolver coverage for `interface Color { ... } export default Color;`.
- [ ] Re-triage `allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts` and record the next diagnostic.

Out of scope:

- `export default interface Name { ... }`, tracked by `issues/open/5401-parse-export-default-interface-declarations.md`.
- Resolving imports between virtual `@filename` sections, tracked by `issues/open/5229a-resolve-imports-between-filename-sections.md`.
- General unresolved-name cleanup unrelated to type-only default exports.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/`
- `crates/frontend/src/resolver.rs`
- focused parser/resolver/reference tests

Do not touch:

- backend/runtime lowering unless the resolver advances to a reviewed runtime shape
- package or non-reference module resolution

## Acceptance criteria

- [ ] `interface Color { c: string; } export default Color;` no longer reports `UnresolvedName: Color`.
- [ ] A focused test covers default-exporting a local interface name.
- [ ] `allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts` advances past the current `UnresolvedName: Color` blocker or records the next narrower diagnostic in this issue.
- [ ] The implementation does not treat arbitrary type-only names as runtime value exports.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/open/601-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md`.

Related: issue 5401 covers `export default interface Name { ... }`; issue 5229
covers the later virtual-file imports `./color` and `./file1`.
Also owns `issues/open/3324-implement-moduleAugmentationOfAlias.md`: fresh
triage for `moduleAugmentationOfAlias.ts` stops at `export default I;` after a
local `interface I {}` declaration, before the later issue 5401
`export default interface I { x: number; }` and issue 5229 `./a`
virtual-section import blockers.

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
