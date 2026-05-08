---
id: 3309
title: "Implement Moduleaugmentationdeclarationemit"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated `declaration-emit` bucket as superseded by the open
implementation-ready issue 5229.

Fresh coverage now classifies both affected references as `import-export`, and
smart triage shows both parse into AST before module graph validation fails on
the virtual sibling import `./observable`.

## Problem

`moduleAugmentationDeclarationEmit1.ts` and
`moduleAugmentationDeclarationEmit2.ts` are declaration-oriented multi-section
reference inputs with virtual `map.ts`, `observable.ts`, and `main.ts` files.
The current compiler cannot resolve `import { Observable } from
"./observable"` from `map.ts` or `main.ts` to the sibling virtual
`observable.ts` section.

Problem: declaration emit behavior is not the first reachable blocker in the
current runner view. The actionable blocker is virtual `@Filename` section
import resolution, already tracked by issue 5229.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit2.ts
```

## Desired final state

This generated bucket is closed. Implement virtual `@Filename` import
resolution from `issues/open/5229a-resolve-imports-between-filename-sections.md`
before re-triaging declaration emit parity for these references.

## Scope

In scope:

- [x] Inspect fresh coverage for both affected reference files.
- [x] Run smart triage for both affected reference files.
- [x] Confirm AST construction succeeds for import/prototype/module-augmentation shapes.
- [x] Confirm module graph reports issue-232 missing local module `./observable`.
- [x] Confirm issue 5229 owns virtual `@Filename` sibling imports.

Out of scope:

- Direct implementation from this generated bucket.
- Declaration emit parity.
- Duplicate identifier or declaration merge diagnostics.
- Runtime/module augmentation semantics.

## Affected paths

Expected:

- `issues/open/5229a-resolve-imports-between-filename-sections.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner issue 5229 contains the exact `moduleAugmentationDeclarationEmit` evidence.
- [x] Closure preserves exact reproduction commands and current diagnostics for both files.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit2.ts
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

- `reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit1.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit2.ts`

## Duplicate detection

- `issues/open/5229a-resolve-imports-between-filename-sections.md` owns the
  current `./observable` virtual-section import resolution blocker.
- Generic name-resolution candidates from smart triage do not own this exact
  virtual multi-file module graph boundary.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=2
build_pass=0
unsupported=2
unsupported_diagcodes=UnsupportedModule:2
unsupported_features=import-export:2
```

Fresh smart triage headlines for both files:

```text
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
message: unresolved name: `Observable` at 49..59
```

Shared source shape:

```ts
// @filename: map.ts
import { Observable } from "./observable"
(<any>Observable.prototype).map = function() { }
declare module "./observable" {
    interface Observable<T> {
        map<U>(proj: (e:T) => U): Observable<U>
    }
    namespace Observable {
        let someAnotherValue: number | string;
    }
}

// @filename: observable.ts
export declare class Observable<T> {
    filter(pred: (e:T) => boolean): Observable<T>;
}
export namespace Observable {
    let someValue: number;
}

// @filename: main.ts
import { Observable } from "./observable"
import "./map";
```

The AST includes named imports from `./observable`, prototype assignment
expressions, side-effect import of `./map`, and use-site declarations. Resolved
module graph validation reports:

```text
issue-232: missing local module `./observable`
```

TypeScript oracle reports later diagnostics including duplicate identifiers,
all-exported-or-all-local merged declarations, missing `./observable`, invalid
augmentation, and missing side-effect import `./map`. The second file also
reports property `someAnotherValue` missing on `typeof Observable`.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit --detail --no-dashboard-data
result: pass; executed=2, unsupported=2, UnsupportedModule:2
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit1.ts
result: pass; AST succeeds, current blocker is virtual `./observable` resolution owned by issue 5229
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDeclarationEmit2.ts
result: pass; AST succeeds, current blocker is virtual `./observable` resolution owned by issue 5229
date: 2026-05-08
```

Remaining risks:

- After issue 5229 advances, declaration emit, duplicate identifier, merged
  declaration, and property diagnostics may need separate focused issues.
