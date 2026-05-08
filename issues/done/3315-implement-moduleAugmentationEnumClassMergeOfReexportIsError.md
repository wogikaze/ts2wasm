---
id: 3315
title: "Implement Moduleaugmentationenumclassmergeofreexportiserror"
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

Closed this generated module-augmentation bucket as superseded by the existing
implementation issue for dependency-module `export class` declarations.

## Problem

Fresh triage shows this reference case no longer needs a standalone bucket.
The current first blocker is the dependency virtual file export-class boundary:

```text
issue-5005: dependency module declaration export uses a form outside the current static export slice at 7..40
```

The case contains a later `export * from "./file"` virtual re-export and enum /
class merge assertion, but those are not reachable until dependency
`export class Foo` is accepted.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationEnumClassMergeOfReexportIsError --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationEnumClassMergeOfReexportIsError.ts
```

Source shape:

```ts
// @filename: file.ts
export class Foo {
    member: string;
}
// @filename: reexport.ts
export * from "./file";
// @filename: augment.ts
import * as ns from "./reexport";
declare module "./reexport" {
    export enum Foo {
        A, B, C
    }
}
declare const f: ns.Foo; //is this the enum or the class? should be an error.
```

Parser evidence:

```text
ExportDecl { ClassDecl { name: "Foo" } }
ExportAllFrom { source: "./file" }
ImportNamespace { source: "./reexport" }
```

Resolved dump also exposes a later blocker after the first failure boundary:

```text
issue-232: missing local module './file'
```

TypeScript oracle diagnostics:

```text
TS2564: Property 'member' has no initializer and is not definitely assigned in the constructor.
TS2307: Cannot find module './file' or its corresponding type declarations.
TS2307: Cannot find module './reexport' or its corresponding type declarations.
TS2664: Invalid module name in augmentation, module './reexport' cannot be found.
```

## Desired final state

Implement the first blocker in
`issues/open/5324-support-dependency-export-class-declarations.md`. After that
lands, rerun this case to expose the next actionable blocker. The later virtual
re-export source resolution shape is already tracked by
`issues/open/5229-resolve-imports-between-filename-sections.md`.

## Scope

In scope:

- [x] Confirm fresh smart-triage evidence for this generated bucket.
- [x] Match the current first blocker to an existing implementation-ready issue.
- [x] Preserve later blocker evidence for the owner issue.

Out of scope:

- Direct implementation from this generated bucket.
- Dependency `export class` implementation.
- Full virtual re-export resolution.
- Enum/class merge diagnostics.

## Affected paths

Expected implementation owner:

- `crates/compiler/src/lib.rs`
- `crates/cli/tests/m9_modules.rs`
- `fixtures/module-system/`

Do not touch from this bucket:

- unrelated runtime/backend code

## Acceptance criteria

- [x] Duplicate/superseding issue identified: `issues/open/5324-support-dependency-export-class-declarations.md`.
- [x] Exact reproduction commands and observed diagnostics are recorded.
- [x] Later virtual re-export blocker is noted as `issues/open/5229-resolve-imports-between-filename-sections.md`.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
git diff --cached --check
```

Reference commands already run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationEnumClassMergeOfReexportIsError --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationEnumClassMergeOfReexportIsError.ts
```

Not run:

- `cargo fmt --all --check` (issue lifecycle only; no Rust changes)
- `cargo nextest run` (issue lifecycle only; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing issue 5324 owns the first blocker
- [x] existing issue 5229 owns the later virtual re-export source resolution blocker

## Notes

Superseded by `issues/open/5324-support-dependency-export-class-declarations.md`.

## Completion evidence

Commits:

- filled by commit

Validation result:

```text
command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-08

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-08

command: git diff --check
result: pass
date: 2026-05-08

command: git diff --cached --check
result: pass
date: 2026-05-08
```

Remaining risks:

- The reference case can still fail after 5324 for the virtual `export * from`
  source path, tracked by issue 5229.
- Enum/class merge diagnostics are not reachable until the module graph blockers
  are removed.
