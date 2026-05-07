---
id: 5402
title: "Skip package.json @Filename sections in reference harness"
type: feature
area: compiler/multi-section
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Handle TypeScript reference tests whose `// @Filename:` sections include
`package.json` files before executable `.js`, `.ts`, or `.d.ts` sections.

## Problem

`allowJsCrossMonorepoPackage.ts` contains virtual package metadata sections:

```ts
// @Filename: /packages/shared/package.json
{
  "name": "shared",
  "version": "1.0.0",
  "type": "module",
  "exports": "./index.js"
}
```

The multi-section compiler path currently tokenizes and parses this JSON body
as TypeScript/JavaScript source. Fresh triage reaches the JSON section and
fails at the first property colon.

Problem: reference-style `package.json` virtual sections are treated as module bodies instead of package metadata or non-code sections.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts
```

Observed result:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 157..158
```

Source context:

```text
5 | // @Filename: /packages/shared/package.json
6 | {
7 |   "name": "shared",
8 |   "version": "1.0.0",
```

Compiler evidence:

```text
tokens: ok through the preceding `export declare function pkg(): "pkg";`
tokens then include JSON object tokens from package.json
ast/resolved: fail on the package.json property colon
```

## Desired final state

The multi-section compiler path recognizes `package.json` sections as reference
test package metadata and does not parse them as executable module bodies.
`allowJsCrossMonorepoPackage.ts` should advance past the JSON metadata section;
any later package-resolution or JS syntax limitation must be reported as its
own narrower blocker.

## Scope

In scope:

- [ ] In the multi-section builder, skip `package.json` virtual sections or store them as metadata instead of lowering them as module bodies.
- [ ] Add focused regression coverage with a `package.json` section followed by a code section.
- [ ] Re-triage `allowJsCrossMonorepoPackage.ts` and record the next diagnostic.

Out of scope:

- Full package.json semantics.
- Node package resolution through `exports`, `types`, or `main`.
- JSON module imports or `resolveJsonModule`.
- Package self-name resolution.
- CommonJS or ESM emit fidelity.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch:

- backend/runtime emit
- broad package resolution implementation

## Acceptance criteria

- [ ] `allowJsCrossMonorepoPackage.ts` no longer reports `UnsupportedSyntax` from parsing the `package.json` body as source.
- [ ] A focused compiler test proves a `// @Filename: package.json` section is skipped before a code section.
- [ ] If package resolution remains unsupported after the skip, the diagnostic points at an import/export module specifier and does not mention the JSON property-colon boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(filename) or test(multi) or test(package)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts --detail --no-dashboard-data
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

Split from `issues/done/598-implement-allowJsCrossMonorepoPackage.md`.

Related but not duplicates:

- `issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`
  covers the same non-code-section behavior for `tsconfig.json`.
- `issues/open/436-implement-module-resolution.md` and package-resolution
  buckets cover later module/package lookup behavior after metadata sections
  are skipped.

Also owns `issues/done/3336-implement-moduleExportNonStructured.md`: fresh
triage for `moduleExportNonStructured.ts` stops in its virtual `package.json`
section at the first property colon before `.mts`, `.cjs`, `.d.cts`, or
CommonJS `export =` diagnostics become reachable.
Also owns `issues/done/3346-implement-moduleLocalImportNotIncorrectlyRedirected.md`:
fresh triage stops in `node_modules/troublesome-lib/package.json` at the first
JSON property colon before package resolution or local import redirection
semantics become reachable.
Also owns `issues/done/3354-implement-moduleNodeImportRequireEmit.md`: fresh
triage stops in `// @filename: package.json` at the first JSON property colon
before `declare module "foo";`, `import foo = require("foo");`, or NodeNext
emit diagnostics become reachable.
Also owns `issues/done/3358-implement-modulePreserve.md` for
`modulePreserve2.ts`: fresh triage stops in virtual `package.json` at the first
JSON property colon before package `exports`, ESM/CJS conditional resolution,
or module-preserve import/require diagnostics become reachable.

Also owns the package-json subset of
`issues/done/3371-implement-moduleResolution-module-resolution.md`: fresh
coverage for `moduleResolution_packageJson_*` reports six
UnsupportedSyntax/module-resolution paths, and representative triage for
`moduleResolution_packageJson_scopedPackage.ts` stops at the virtual
`package.json` property colon before scoped package resolution becomes
actionable.

Also owns `issues/done/3374-implement-moduleResolutionAsTypeReferenceDirectiveAmbient.md`:
fresh triage for `moduleResolutionAsTypeReferenceDirectiveAmbient.ts` parses
`declare module "phaser" { export const a2: number; }`, then stops in the
virtual `/typings/phaser/package.json` body at the first JSON property colon
before type-root or package resolution becomes actionable.
Also owns `issues/done/3379-implement-moduleResolutionPackageIdWithRelativeAndAbsolutePath.md`:
fresh triage for `moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts`
stops in `/shared/node_modules/troublesome-lib/package.json` at the first JSON
property colon before package-id relative/absolute path behavior, path mapping,
or virtual node_modules resolution becomes actionable.

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
