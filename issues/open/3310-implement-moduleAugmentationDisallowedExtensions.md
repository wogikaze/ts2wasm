---
id: 3310
title: "Implement Moduleaugmentationdisallowedextensions"
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
implementation-ready issue 5175.

Fresh smart triage shows the first actionable blocker is the entry-section
`export let a = 1;` declaration, which currently reports the generic issue-055
unsupported variable export boundary before AST construction.

## Problem

`moduleAugmentationDisallowedExtensions.ts` combines several separate concerns:

- entry-section `export let a = 1;`
- namespace `export let x = 1;`
- ambient module variable declarations and imports/re-exports
- `export = N1`
- `export var x = 1;`
- virtual `@filename` imports from `./observable`, `./x`, `./x0`, and `./test`

Problem: the stale generated bucket is too broad. The current first blocker is
already owned by issue 5175, while later blockers are represented by existing
issues such as 5285 and 5229.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDisallowedExtensions --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDisallowedExtensions.ts
```

## Desired final state

This generated bucket is closed. Implement the current first blocker from
`issues/done/5175-support-export-let-destructuring-declarations.md`.

## Scope

In scope:

- [x] Inspect fresh coverage and smart triage.
- [x] Identify `export let a = 1;` as the current first blocker.
- [x] Confirm open issue 5175 owns `export let <identifier> = <expr>;`.
- [x] Record later related owner issues 5285 and 5229.

Out of scope:

- Direct implementation from this generated bucket.
- Ambient module augmentation diagnostics.
- Runtime/module export lowering.
- Virtual `@filename` import resolution.

## Affected paths

Expected:

- `issues/done/5175-support-export-let-destructuring-declarations.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner issue 5175 contains the exact `moduleAugmentationDisallowedExtensions.ts` evidence.
- [x] Later known blockers are linked to issue 5285 and issue 5229.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDisallowedExtensions --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDisallowedExtensions.ts
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

- `reference/typescript/tests/cases/compiler/moduleAugmentationDisallowedExtensions.ts`

## Duplicate detection

- `issues/done/5175-support-export-let-destructuring-declarations.md` owns the
  current first blocker: `export let a = 1;` reports generic issue-055 before
  AST construction.
- `issues/done/5285-support-export-var-initializer-declarations.md` owns the
  later initialized `export var x = 1;` boundary in the virtual
  `observable.ts` section.
- `issues/open/5229-resolve-imports-between-filename-sections.md` owns later
  virtual `@filename` imports such as `./observable`, `./x`, `./x0`, and
  `./test`.
- `issues/open/5346-parse-commonjs-export-assignment-statements.md` may own
  the later `export = N1` shape after the earlier variable-export boundary
  advances.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Fresh smart triage headline:

```text
Feature label: import-export
Diagnostic: UnsupportedModule / unsupported-feature-boundary
message: issue-055: unsupported variable export; module resolution and loading are not implemented at 0..6
line: 1, column: 1
```

Source starts with:

```ts
// @filename: x0.ts
export let a = 1;
```

Later source includes:

```ts
namespace N1 {
    export let x = 1;
}

declare module "./observable" {
    import * as all from "./x0";
    import {a} from "./x0";
    export * from "./x0";
    export {a} from "./x0";
}

declare module "./test" {
    export = N1;
}
export {}

// @filename: observable.ts
export declare class Observable<T> { ... }
export var x = 1;

// @filename: test.ts
export let b = 1;

// @filename: main.ts
import { Observable } from "./observable"
import "./x";
```

AST/resolved output stops before AST construction:

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 84..90
```

TypeScript oracle reports later diagnostics including invalid augmentation
module names, missing local modules, all-exported-or-all-local merged
declarations, and missing side-effect import `./x`.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDisallowedExtensions --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDisallowedExtensions.ts
result: pass; current first blocker is issue-055 unsupported variable export for `export let a = 1;`, owned by issue 5175
date: 2026-05-08
```

Remaining risks:

- Advancing issue 5175 may expose export-var, export-assignment, virtual
  section import, or TypeScript semantic diagnostics listed above.
