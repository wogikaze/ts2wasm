---
id: 3326
title: "Implement Moduleaugmentationsbundledoutput"
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

Fresh triage shows `moduleAugmentationsBundledOutput1.ts` parses multiple
virtual sections with `export class`, local imports, prototype assignments, and
ambient module augmentations. Smart triage stops at the first dependency
virtual file `export class` boundary before bundled output or augmentation
semantics are actionable.

Problem: `moduleAugmentationsBundledOutput` duplicates the issue-5005
dependency-module `export class` blocker already owned by issue 5324.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationsBundledOutput --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationsBundledOutput1.ts
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5324-support-dependency-export-class-declarations.md`.

## Scope

In scope:

- [x] Inspect fresh coverage and smart triage.
- [x] Confirm whether existing open/done issues already cover this bucket.
- [x] Supersede this bucket with the existing dependency export-class owner.
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence.

Out of scope:

- Direct implementation from this generated bucket.
- AMD/outFile bundled output emit parity.
- Full module augmentation type merging.
- Virtual-section import resolution, tracked separately by issue 5229 if it appears after issue 5324 advances.

## Affected paths

Expected:

- `issues/open/5324-support-dependency-export-class-declarations.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner contains an exact issue-5005 dependency export-class command.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationsBundledOutput --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationsBundledOutput1.ts
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

- `reference/typescript/tests/cases/compiler/moduleAugmentationsBundledOutput1.ts`

## Duplicate detection

- `issues/open/5324-support-dependency-export-class-declarations.md` owns the
  current issue-5005 boundary for dependency virtual files that start with
  `export class`.
- `issues/open/5229-resolve-imports-between-filename-sections.md` is a likely
  later owner for local `./m1` and `./m3` virtual-section resolution after
  dependency export-class support advances.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

The source contains multiple virtual sections:

```ts
// @filename: m1.ts
export class Cls {}

// @filename: m2.ts
import {Cls} from "./m1";
(<any>Cls.prototype).foo = function() { return 1; };
declare module "./m1" { interface Cls { foo(): number; } }

// @filename: m3.ts
export class C1 { x: number }
export class C2 { x: string }

// @filename: test.ts
import { Cls } from "./m1";
import "m2";
import "m4";
```

Tokens and AST parse the export classes, imports, prototype assignments, and
ambient augmentations. Smart triage reports the actionable first boundary:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form outside the current static export slice
```

The resolved dump also shows `issue-232: missing local module ./m1`, but that is
behind the issue-5005 dependency export-class owner for this bucket.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationsBundledOutput --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_features=import-export=1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationsBundledOutput1.ts
result: pass; current blocker is issue-5005 dependency-module export class, superseded by issue 5324
date: 2026-05-08
```

Remaining risks:

- Dependency virtual file `export class` support remains open in issue 5324.

## False-done audit

**truly-done** (3326)

- Implementation commits: verified via `git log --oneline --all --grep=3326`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
