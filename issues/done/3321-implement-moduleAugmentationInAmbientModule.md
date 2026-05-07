---
id: 3321
title: "Implement Moduleaugmentationinambientmodule"
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

Closed this generated bucket as superseded by the completed issue 232
bare/non-local module specifier boundary.

## Problem

Fresh triage shows the `moduleAugmentationInAmbientModule` references contain
ambient external module declarations for bare specifiers such as `"Observable"`,
`"M"`, and `"Map"`. The compiler tokenizes these declarations and the runtime
entry section, then the resolved module graph rejects the bare specifier import
as issue-232.

Problem: `moduleAugmentationInAmbientModule` duplicates the existing
unsupported bare/non-local module specifier boundary rather than describing a
new executable implementation slice.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule4.ts
```

## Desired final state

This generated bucket is closed. No implementation should start from this
issue; package/bare module resolution remains intentionally out of scope for
issue 232.

## Scope

In scope:

- [x] Inspect fresh coverage and representative smart triage.
- [x] Confirm whether existing open/done issues already cover this bucket.
- [x] Supersede this bucket with the existing issue-232 non-local specifier boundary.
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence.

Out of scope:

- Direct implementation from this generated bucket.
- Package resolution, import maps, or node_modules semantics.
- Full ambient module augmentation type merging.
- Runtime support for declaration-only ambient external modules.

## Affected paths

Expected:

- `issues/done/232-resolve-local-relative-es-module-graph.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner contains the bare/non-local module specifier boundary.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule4.ts
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

- `reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule1.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule2.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule3.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule4.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule5.ts`

## Duplicate detection

- `issues/done/232-resolve-local-relative-es-module-graph.md` owns the current
  source-spanned rejection of bare/non-local module specifiers.
- `issues/done/3316-implement-moduleAugmentationExtendAmbientModule.md` is the
  closest closed module-augmentation precedent: it also closed against issue
  232 for a bare module specifier.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=5
unsupported=5
unsupported_diagcodes=UnsupportedModule:5
unsupported_features=import-export:5
```

### `moduleAugmentationInAmbientModule1.ts`

The source declares ambient external modules and then imports a bare module:

```ts
declare module "Observable" {
    class Observable {}
}

declare module "M" {
    class Cls { x: number }
}

declare module "Map" {
    import { Cls } from "M";
    module "Observable" { interface Observable { foo(): Cls; } }
}

import {Observable} from "Observable";
```

The smart-triage headline reports `issue-211: unknown receiver class for method
foo` while the AST dump contains only the runtime entry section. The resolved
module graph reports the actionable current boundary:

```text
UnsupportedModule: issue-232: unsupported non-local module specifier `Observable`
```

### `moduleAugmentationInAmbientModule2.ts`

This representative adds side-effect import of `Map`:

```ts
import {Observable} from "Observable";
import "Map";
```

The resolved module graph first reports the same issue-232 unsupported
non-local module specifier `Observable`; `Map` is the same bare-specifier family.

### `moduleAugmentationInAmbientModule4.ts`

This representative adds a second declaration file and another nested
augmentation. The TypeScript AST classifies the declaration sections as
`ModuleDeclaration`, and the resolved module graph again reaches:

```text
UnsupportedModule: issue-232: unsupported non-local module specifier `Observable`
```

Issue 232 intentionally rejects these bare specifiers. No child issue is
created from this generated bucket.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule --detail --no-dashboard-data
result: pass; executed=5, unsupported=5, unsupported_features=import-export=5
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule1.ts
result: pass; resolved dump reaches issue-232 unsupported non-local module specifier `Observable`
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule2.ts
result: pass; resolved dump reaches issue-232 unsupported non-local module specifier `Observable`
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInAmbientModule4.ts
result: pass; resolved dump reaches issue-232 unsupported non-local module specifier `Observable`
date: 2026-05-08
```

Remaining risks:

- none for this generated bucket; package/bare module resolution remains out of scope by issue 232 policy.
