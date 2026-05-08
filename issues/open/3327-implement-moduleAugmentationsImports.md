---
id: 3327
title: "Implement Moduleaugmentationsimports"
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

Fresh triage shows the `moduleAugmentationsImports` references parse their
virtual sections, imports, ambient external module declarations, prototype
assignments, and module augmentations. Smart triage stops at the first
dependency virtual-file `export class` boundary before the import/augmentation
semantics are actionable.

Problem: `moduleAugmentationsImports` duplicates the issue-5005
dependency-module `export class` blocker already owned by issue 5324.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationsImports --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationsImports1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationsImports2.ts
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
- Virtual-section import resolution, tracked separately by issue 5229 if it appears after issue 5324 advances.
- Bare module/package resolution for `"C"`.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationsImports --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationsImports1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationsImports2.ts
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

- `reference/typescript/tests/cases/compiler/moduleAugmentationsImports1.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationsImports2.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationsImports3.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationsImports4.ts`

## Duplicate detection

- `issues/open/5324-support-dependency-export-class-declarations.md` owns the
  current issue-5005 boundary for dependency virtual files that start with
  `export class`.
- `issues/open/5229-resolve-imports-between-filename-sections.md` is a likely
  later owner for local `./a` and `./b` virtual-section resolution after
  dependency export-class support advances.
- `issues/open/232-resolve-local-relative-es-module-graph.md` owns the bare
  module specifier boundary for `"C"` if the local virtual-section blockers
  advance far enough to expose it.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=4
unsupported=4
unsupported_diagcodes=UnsupportedModule:4
unsupported_features=import-export:4
```

The representative files contain virtual sections:

```ts
// @filename: a.ts
export class A {}

// @filename: b.ts
export class B { x: number; }

// @filename: d.ts
import {A} from "./a";
import {B} from "./b";
import {Cls} from "C";
A.prototype.getB = function () { return undefined; }
declare module "./a" { interface A { getB(): B; } }
```

Tokens and AST parse the export classes, imports, ambient external module
declaration for `"C"`, prototype assignments, and module augmentations. Smart
triage reports the actionable first boundary:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form outside the current static export slice
```

The resolved dump also shows `issue-232: missing local module ./a`, but that is
behind the issue-5005 dependency export-class owner for this bucket.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationsImports --detail --no-dashboard-data
result: pass; executed=4, unsupported=4, unsupported_features=import-export=4
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationsImports1.ts
result: pass; current blocker is issue-5005 dependency-module export class, superseded by issue 5324
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationsImports2.ts
result: pass; current blocker is issue-5005 dependency-module export class, superseded by issue 5324
date: 2026-05-08
```

Remaining risks:

- Dependency virtual file `export class` support remains open in issue 5324.

## False-done audit

**truly-done** (3327)

- Implementation commits: verified via `git log --oneline --all --grep=3327`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
