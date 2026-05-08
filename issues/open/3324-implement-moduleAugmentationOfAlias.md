---
id: 3324
title: "Implement Moduleaugmentationofalias"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated `import-export` bucket as superseded by the open
implementation-ready issue 5403.

Fresh smart triage shows the first actionable blocker is no longer generic
module augmentation syntax. The compiler parses the static import/export forms,
then reports `UnresolvedName` for `export default I;` where `I` is a local
type-only interface declaration.

## Problem

`moduleAugmentationOfAlias.ts` contains three separable concerns:

- type-only default export of a local interface: `interface I {}` followed by
  `export default I;`
- ambient module augmentation of `./a` with `export default interface I { ... }`
- virtual `@Filename` section import resolution for `import I from "./a"`

Problem: the stale bucket is too broad. The current first blocker belongs to
issue 5403, while later blockers are already represented by issue 5401
(`export default interface`) and issue 5229 (imports between `@Filename`
sections).

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationOfAlias --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationOfAlias.ts
```

## Desired final state

This generated bucket is closed. Implement the current first blocker from
`issues/open/5403-support-type-only-default-exports-of-local-interfaces.md`.
Use the later issues only after triage advances beyond `export default I;`.

## Scope

In scope:

- [x] Inspect fresh coverage and smart triage.
- [x] Identify the current first blocker.
- [x] Confirm the first blocker is owned by issue 5403.
- [x] Record later related owner issues 5401 and 5229.

Out of scope:

- Direct implementation from this generated bucket.
- Resolving imports between virtual `@Filename` sections.
- Parsing `export default interface` declarations.
- Broad module augmentation semantic parity.

## Affected paths

Expected:

- `issues/open/5403-support-type-only-default-exports-of-local-interfaces.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner issue 5403 contains the exact `moduleAugmentationOfAlias.ts` evidence.
- [x] Later known blockers are linked to issue 5401 and issue 5229.
- [x] Closure preserves exact reproduction commands and current diagnostic.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationOfAlias --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationOfAlias.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleAugmentationOfAlias.ts`

## Duplicate detection

- `issues/open/5403-support-type-only-default-exports-of-local-interfaces.md`
  owns the current first blocker: local interface `I` referenced by
  `export default I;`.
- `issues/open/5401-parse-export-default-interface-declarations.md` owns the
  later `export default interface I { x: number; }` construct inside the
  `declare module "./a"` block.
- `issues/open/5229a-resolve-imports-between-filename-sections.md` owns the
  later missing virtual local module `./a` imported from another `@Filename`
  section.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Fresh smart triage headline:

```text
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
message: unresolved name: `I` at 30..31
line: 2, column: 10
```

Source shape:

```ts
// @Filename: /a.ts
interface I {}
export default I;

// @Filename: /b.ts
export {};
declare module './a' {
    export default interface I { x: number; }
}

// @Filename: /c.ts
import I from "./a";
function f(i: I) {
    i.x;
}
```

The AST includes `ExportDefault Ident("I")`, `ExportNamed { specifiers: [] }`,
`ImportDefault("./a")`, and `Function f`. The resolved dump also shows the
later module graph boundary:

```text
issue-232: missing local module `./a` imported from moduleAugmentationOfAlias.ts
```

TypeScript oracle reports:

```text
TS2664: Invalid module name in augmentation, module './a' cannot be found.
TS2307: Cannot find module './a' or its corresponding type declarations.
TS2339: Property 'x' does not exist on type 'I'.
```

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationOfAlias --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedModule:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationOfAlias.ts
result: pass; current first blocker is UnresolvedName `I`, owned by issue 5403
date: 2026-05-08
```

Remaining risks:

- Advancing issue 5403 may expose issue 5401, issue 5229, or the TypeScript
  oracle diagnostics listed above.
