---
id: 3320
title: "Implement Moduleaugmentationimportsandexports"
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
dependency-module `export class` issue 5324.

## Problem

Fresh triage shows the six `moduleAugmentationImportsAndExports` references
parse their virtual sections, imports, prototype assignments, and ambient
module augmentations far enough to hit the current static module export boundary
for dependency virtual files containing `export class`.

Problem: `moduleAugmentationImportsAndExports` duplicates the issue-5005
dependency-module `export class` blocker already owned by issue 5324.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports4.ts
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5324-support-dependency-export-class-declarations.md`.

## Scope

In scope:

- [x] Inspect fresh coverage and representative smart triage.
- [x] Confirm whether existing open/done issues already cover this bucket.
- [x] Supersede this bucket with the existing dependency export-class owner.
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence.

Out of scope:

- Direct implementation from this generated bucket.
- Full module augmentation type merging.
- Package resolution or non-local module specifier support.

## Affected paths

Expected:

- `issues/open/5324-support-dependency-export-class-declarations.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner contains an exact issue-5005 dependency export-class command.
- [x] Triage evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence.
- [x] Completion evidence names the exact reference paths and diagnostic boundary.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports4.ts
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

- [x] existing owner: `issues/open/5324-support-dependency-export-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports1.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports2.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports3.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports4.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports5.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports6.ts`

## Duplicate detection

- `issues/open/5324-support-dependency-export-class-declarations.md` owns the
  current issue-5005 boundary for dependency virtual files that start with
  `export class`.
- `issues/done/5229-w0-user-runtime-string-origin.md` is related
  for `@filename` local import resolution, but the smart-triage first
  diagnostic for this bucket is issue-5005 dependency `export class`.

## Smart triage

### `moduleAugmentationImportsAndExports1.ts`

The reference contains virtual sections:

```ts
// @filename: f1.ts
export class A {}

// @filename: f2.ts
export class B { n: number; }

// @filename: f3.ts
import {A} from "./f1";
import {B} from "./f2";
A.prototype.foo = function () { return undefined; }
declare module "./f1" { interface A { foo(): B; } }
```

Tokens and AST parse the export classes, named imports, prototype assignment,
and the ambient module augmentation. Smart triage reports:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form outside the current static export slice
```

The resolved dump also shows the local virtual import path `./f1`; once the
dependency export-class boundary advances, virtual-section resolution follow-up
may overlap with issue 5229.

### `moduleAugmentationImportsAndExports4.ts`

This representative adds namespace exports and import-equals aliases:

```ts
namespace N {
    export interface Ifc { a: number; }
    export interface Cls { b: number; }
}
import I = N.Ifc;
import C = N.Cls;
```

The first smart-triage diagnostic remains the same issue-5005 dependency
`export class` boundary before the namespace/interface augmentation semantics
matter.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports --detail --no-dashboard-data
result: pass; executed=6, unsupported=6, unsupported_features=import-export=6
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports1.ts
result: pass; current blocker is issue-5005 dependency-module export class, superseded by issue 5324
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationImportsAndExports4.ts
result: pass; current blocker is issue-5005 dependency-module export class, superseded by issue 5324
date: 2026-05-08
```

Remaining risks:

- Dependency virtual file `export class` support remains open in issue 5324.
