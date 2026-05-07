---
id: 3323
title: "Implement Moduleaugmentationnonewnames"
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

Closed this generated bucket as superseded by the existing implementation-ready
`@filename` virtual-section import resolution issue 5229.

## Problem

Fresh triage shows `moduleAugmentationNoNewNames.ts` parses the import,
prototype assignment, ambient module augmentation, declaration-only class, and
runtime entry section. The resolved module graph then looks on disk for
`./observable` instead of resolving the sibling `// @filename: observable.ts`
section.

Problem: `moduleAugmentationNoNewNames` duplicates the virtual-section local
import resolution gap already owned by issue 5229.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationNoNewNames --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationNoNewNames.ts
```

## Desired final state

This generated bucket is closed. Implement from
`issues/done/5229-w0-user-runtime-string-origin.md`.

## Scope

In scope:

- [x] Inspect fresh coverage and smart triage.
- [x] Confirm whether existing open/done issues already cover this bucket.
- [x] Supersede this bucket with the existing virtual-section import-resolution owner.
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence.

Out of scope:

- Direct implementation from this generated bucket.
- Full module augmentation semantics.
- Declaration emit parity for the ambient members inside the augmentation.

## Affected paths

Expected:

- `issues/done/5229-w0-user-runtime-string-origin.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner contains the virtual-section local import-resolution contract.
- [x] Triage evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence.
- [x] Completion evidence names the exact reference path and diagnostic boundary.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationNoNewNames --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationNoNewNames.ts
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

- [x] existing owner: `issues/done/5229-w0-user-runtime-string-origin.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleAugmentationNoNewNames.ts`

## Duplicate detection

- `issues/done/5229-w0-user-runtime-string-origin.md` owns local
  import resolution between `// @filename:` virtual sections.
- `issues/done/3317-implement-moduleAugmentationExtendFileModule.md` is the
  closest closed module-augmentation precedent: it also closes against issue
  5229 for `./observable` / `./map` virtual-section imports.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

The source contains:

```ts
// @filename: map.ts
import { Observable } from "./observable"
(<any>Observable.prototype).map = function() { }

declare module "./observable" { ... }

// @filename: observable.ts
export declare class Observable<T> { ... }

// @filename: main.ts
import { Observable } from "./observable"
import "./map";
```

The smart-triage headline reports `UnresolvedName` for `Observable`, but the
AST dump shows the named imports, prototype assignment, side-effect import, and
runtime call are parsed. The resolved module graph reports the actionable
current boundary:

```text
UnsupportedModule: issue-232: missing local module `./observable`
```

The diagnostic tries on-disk candidates such as `./observable.ts` and
`./observable.js`; it does not resolve the sibling virtual `observable.ts`
section. That is exactly the issue 5229 family.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationNoNewNames --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_features=import-export=1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationNoNewNames.ts
result: pass; resolved dump reaches issue-232 missing local module `./observable`, superseded by issue 5229
date: 2026-05-08
```

Remaining risks:

- Virtual `@filename` sibling-section import resolution remains open in issue 5229.
