---
id: 5427a
title: "Resolve reference types to virtual @types packages"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: [227]
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Resolve `/// <reference types="..."/>` directives to reference-test virtual
`node_modules/@types/<package>/index.d.ts` sections.

## Problem

`reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_referenceTypes.ts`
defines virtual `@types/library-a` and `@types/library-b` sections, then the
app section references both packages. Current triage stops at the existing
issue-227 unsupported diagnostic instead of loading those virtual declaration
sections.

Current diagnostic:

```text
UnsupportedTypeScriptSyntax: issue-227: triple-slash reference types directive for `library-a` requires type package resolution
```

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_referenceTypes.ts
```

Coverage result from 2026-05-08: executed 1 case, build pass 0,
unsupported 1, unsupported feature `module-resolution`.

## Desired final state

The reference runner resolves the `library-a` and `library-b` type-reference
directives to the virtual `@types` sections and preserves issue-227 diagnostics
for missing packages.

## Scope

In scope:

- [ ] Resolve `reference types` package names to virtual `@types` index files.
- [ ] Handle a nested virtual declaration file that references another package.

Out of scope:

- Full package manager integration.
- General static import package resolution.

## Affected paths

Expected:

- `crates/frontend/src/type_reference_directive.rs`
- `crates/compiler/src/lib.rs`
- focused frontend or compiler regression tests

## Acceptance criteria

- [ ] The representative reference path no longer reports the issue-227
      unsupported diagnostic for `library-a`.
- [ ] Missing type packages still report an issue-227 diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend type_reference
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_referenceTypes.ts
```

Impacted command:

```text
reference-coverage for the same path should no longer count this boundary under UnsupportedSyntax/module-resolution.
```

## Notes

Split from `issues/open/3388-implement-moduleResolutionWithSymlinks-parser-syntax.md`.
Issue 227 owns the current diagnostic-only behavior; this issue owns the first
virtual `@types` resolution slice.
