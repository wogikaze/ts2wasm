---
id: 3313
title: "Implement Moduleaugmentationdoesnamespacemergeofreexport"
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
implementation-ready issue 5229.

Fresh triage shows AST construction succeeds for the star re-export,
namespace import, namespace augmentation, and use-site expressions. Module
graph validation then fails because `export * from "./file"` in virtual
`reexport.ts` cannot resolve to the sibling virtual `file.ts` section.

## Problem

`moduleAugmentationDoesNamespaceMergeOfReexport.ts` is a multi-section
reference input with virtual `file.ts`, `reexport.ts`, and `augment.ts`
sections. The current compiler treats the re-export source `./file` as an
on-disk path rather than resolving it through the virtual section table.

Problem: the current blocker is virtual `@Filename` section resolution for a
static re-export source specifier, not namespace merge semantics.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDoesNamespaceMergeOfReexport --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDoesNamespaceMergeOfReexport.ts
```

## Desired final state

This generated bucket is closed. Implement virtual `@Filename` import/re-export
specifier resolution from
`issues/open/5229-resolve-imports-between-filename-sections.md` before
re-triaging namespace merge behavior.

## Scope

In scope:

- [x] Inspect fresh coverage and smart triage.
- [x] Confirm AST construction succeeds.
- [x] Confirm module graph reports issue-232 missing local module `./file`.
- [x] Confirm issue 5229 owns virtual `@Filename` sibling specifier resolution.

Out of scope:

- Direct implementation from this generated bucket.
- Namespace merge or module augmentation semantics.
- Runtime module export lowering.

## Affected paths

Expected:

- `issues/open/5229-resolve-imports-between-filename-sections.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner issue 5229 contains the exact `moduleAugmentationDoesNamespaceMergeOfReexport.ts` evidence.
- [x] Closure preserves exact reproduction commands and current diagnostic.
- [x] Later TypeScript oracle diagnostics are recorded as follow-up risk, not mixed into this bucket.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDoesNamespaceMergeOfReexport --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDoesNamespaceMergeOfReexport.ts
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

- `reference/typescript/tests/cases/compiler/moduleAugmentationDoesNamespaceMergeOfReexport.ts`

## Duplicate detection

- `issues/open/5229-resolve-imports-between-filename-sections.md` owns the
  current virtual `./file` re-export source resolution blocker.
- Generic import/export candidates from smart triage do not own this exact
  virtual multi-file module graph boundary.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Source shape:

```ts
// @filename: file.ts
export namespace Root {
    export interface Foo {
        x: number;
    }
}

// @filename: reexport.ts
export * from "./file";

// @filename: augment.ts
import * as ns from "./reexport";
declare module "./reexport" {
    export namespace Root {
        export interface Foo {
            self: Foo;
        }
    }
}
declare const f: ns.Root.Foo;
f.x;
f.self;
f.self.x;
f.self.self;
```

The AST includes:

```text
ExportAllFrom { source: "./file" }
ImportNamespace { source: "./reexport" }
Expr f.x / f.self / f.self.x / f.self.self
```

Resolved/module graph validation reports:

```text
issue-232: missing local module `./file` re-exported from reexport.ts
```

TypeScript oracle reports later diagnostics:

```text
TS2307: Cannot find module './file' or its corresponding type declarations.
TS2307: Cannot find module './reexport' or its corresponding type declarations.
TS2664: Invalid module name in augmentation, module './reexport' cannot be found.
```

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDoesNamespaceMergeOfReexport --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedModule:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDoesNamespaceMergeOfReexport.ts
result: pass; AST succeeds, current blocker is virtual `./file` re-export source resolution owned by issue 5229
date: 2026-05-08
```

Remaining risks:

- After issue 5229 advances, namespace merge, module augmentation, and
  TypeScript oracle diagnostics may need separate focused issues.
