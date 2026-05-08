---
id: 3308
title: "Implement Moduleaugmentationcollidingnamesinaugmentation"
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

Fresh triage shows the current first blocker is the same virtual
`@filename`-section module resolution gap already owned by issue 5229: imports
from `map1.ts`, `map2.ts`, and `main.ts` refer to sibling virtual section
`./observable`, but module graph resolution looks on disk and misses it.

## Problem

`moduleAugmentationCollidingNamesInAugmentation1.ts` is a multi-section
reference input with virtual files:

- `map1.ts`
- `map2.ts`
- `observable.ts`
- `main.ts`

The source parses into named imports, prototype assignments, ambient module
declarations, side-effect imports, and a typed `let` declaration. Resolution
then fails before module augmentation collision diagnostics can be reached.

Problem: this bucket is not a standalone implementation order. The current
first blocker is resolving `./observable` to the sibling virtual
`observable.ts` section, which is issue 5229.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationCollidingNamesInAugmentation1 --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationCollidingNamesInAugmentation1.ts
```

## Desired final state

This generated bucket is closed. Implement virtual `@Filename` import
resolution from `issues/open/5229a-resolve-imports-between-filename-sections.md`
before re-triaging the later duplicate-declaration/module-augmentation
diagnostics.

## Scope

In scope:

- [x] Inspect fresh coverage and smart triage.
- [x] Confirm AST construction succeeds.
- [x] Confirm module graph reports issue-232 missing local module `./observable`.
- [x] Confirm issue 5229 owns virtual `@Filename` sibling imports.

Out of scope:

- Direct implementation from this generated bucket.
- Duplicate identifier or declaration merge diagnostics.
- Runtime/module augmentation semantics.
- Package or non-local module resolution.

## Affected paths

Expected:

- `issues/open/5229a-resolve-imports-between-filename-sections.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner issue 5229 contains the exact `moduleAugmentationCollidingNamesInAugmentation1.ts` evidence.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationCollidingNamesInAugmentation1 --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationCollidingNamesInAugmentation1.ts
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

- `reference/typescript/tests/cases/compiler/moduleAugmentationCollidingNamesInAugmentation1.ts`

## Duplicate detection

- `issues/open/5229a-resolve-imports-between-filename-sections.md` owns the
  current `./observable` virtual-section import resolution blocker.
- Generic name-resolution candidates from smart triage do not own this exact
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

Fresh smart triage headline:

```text
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
message: unresolved name: `Observable` at 49..59
line: 3, column: 13
```

Source shape:

```ts
// @filename: map1.ts
import { Observable } from "./observable"
(<any>Observable.prototype).map = function() { }
declare module "./observable" { interface I {x0} }

// @filename: map2.ts
import { Observable } from "./observable"
(<any>Observable.prototype).map = function() { }
declare module "./observable" { interface I {x1} }

// @filename: observable.ts
export declare class Observable<T> {
    filter(pred: (e:T) => boolean): Observable<T>;
}

// @filename: main.ts
import { Observable } from "./observable"
import "./map1";
import "./map2";
let x: Observable<number>;
```

The AST includes named imports from `./observable`, prototype assignment
expressions, side-effect imports of `./map1` and `./map2`, and a typed `let`.
Resolved/module graph validation then reports:

```text
issue-232: missing local module `./observable`
```

TypeScript oracle reports later diagnostics including:

```text
TS2300: Duplicate identifier 'Observable'.
TS2395: Individual declarations in merged declaration 'Observable' must be all exported or all local.
TS2307: Cannot find module './observable' or its corresponding type declarations.
TS2664: Invalid module name in augmentation, module './observable' cannot be found.
TS2882: Cannot find module or type declarations for side-effect import of './map1'/'./map2'.
```

Those are not reachable until issue 5229 resolves sibling virtual sections.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationCollidingNamesInAugmentation1 --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedModule:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationCollidingNamesInAugmentation1.ts
result: pass; AST succeeds, current blocker is virtual `./observable` resolution owned by issue 5229
date: 2026-05-08
```

Remaining risks:

- After issue 5229 advances, duplicate identifier and merged-declaration
  diagnostics may need a separate focused issue.
