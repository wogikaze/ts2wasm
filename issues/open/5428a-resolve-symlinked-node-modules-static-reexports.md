---
id: 5428a
title: "Resolve symlinked node_modules static re-exports"
type: feature
area: compiler/module-graph
class: implementation-ready
priority: P1
depends_on: [5426]
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Resolve `export { name } from "pkg"` from a symlinked virtual dependency to a
matching virtual `node_modules/pkg/index.d.ts` section.

## Problem

`reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_preserveSymlinks.ts`
defines `/linked/index.d.ts` with symlinks under `/app/node_modules/linked` and
`linked2`. That dependency re-exports from bare package `"real"`, and the same
reference file provides `/app/node_modules/real/index.d.ts`.

Current diagnostic:

```text
UnsupportedModule: issue-232: unsupported non-local module specifier `real` in static re-export
```

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_preserveSymlinks.ts
```

Coverage result from 2026-05-08: executed 1 case, build pass 0,
unsupported 1, unsupported feature `module-resolution`.

## Desired final state

The module graph resolves the bare static re-export from the symlinked
dependency through the virtual `/app/node_modules/real/index.d.ts` package
section, while unmatched bare packages still report issue-232.

## Scope

In scope:

- [ ] Use registered `@symlink` aliases as package-resolution base paths.
- [ ] Resolve this static re-export form to a virtual package index file.

Out of scope:

- CommonJS `require("pkg")`, tracked by issue 5295.
- Classic module resolution, tracked by issue 5421.
- Triple-slash type-reference package resolution, tracked by issue 5427.

## Affected paths

Expected:

- `crates/compiler/src/module_graph.rs`
- `crates/compiler/src/lib.rs`
- focused compiler regression tests

## Acceptance criteria

- [ ] The representative reference path no longer reports issue-232 for
      static re-export specifier `real`.
- [ ] Unmatched bare package specifiers still report issue-232.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(module) or test(symlink)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_preserveSymlinks.ts
```

Impacted command:

```text
reference-coverage for the same path should no longer count this boundary under UnsupportedSyntax/module-resolution.
```

## Notes

Split from `issues/open/3388-implement-moduleResolutionWithSymlinks-parser-syntax.md`.
Issue 232 owns local-relative graph diagnostics; this issue owns the focused
virtual package re-export slice.
