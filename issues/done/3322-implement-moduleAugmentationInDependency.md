---
id: 3322
title: "Implement Moduleaugmentationindependency"
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

Closed this generated `import-export` bucket as superseded by the completed
issue 232 non-local module specifier boundary.

Fresh coverage still reports two `UnsupportedModule: import-export` cases, but
smart triage shows the parser reaches AST construction and then module graph
validation intentionally rejects the bare package specifier `A`.

## Problem

The stale generated issue describes `moduleAugmentationInDependency` as an
untriaged import/export syntax blocker. Current evidence is narrower: both
reference files parse the ambient external module declaration and `export {}`,
then fail on `import "A"` in `/src/app.ts`.

Problem: this bucket is already owned by issue 232's explicit out-of-scope
boundary for package resolution, import maps, absolute specifiers, and other
non-local module specifiers.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationInDependency --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInDependency.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInDependency2.ts
```

## Desired final state

This generated bucket is closed. The current observable blocker remains covered
by `issues/open/232-resolve-local-relative-es-module-graph.md`, which
intentionally rejects bare/non-local module specifiers with an issue-linked
diagnostic.

## Scope

In scope:

- [x] Inspect fresh coverage for both affected reference files.
- [x] Run smart triage for both affected reference files.
- [x] Confirm the parser reaches AST construction before module graph rejection.
- [x] Confirm completed issue 232 owns the current non-local specifier boundary.

Out of scope:

- Implementing Node/package resolution for `node_modules/A`.
- Implementing import maps, path mapping, or absolute specifier support.
- Module augmentation semantic diagnostics after package resolution.
- Direct Rust implementation from this generated bucket.

## Affected paths

Expected:

- none

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Current blocker is linked to completed issue 232.
- [x] Triage evidence names the exact failing specifier and reference paths.
- [x] Closure preserves TypeScript oracle diagnostics for future semantic work.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationInDependency --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInDependency.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInDependency2.ts
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

- `reference/typescript/tests/cases/compiler/moduleAugmentationInDependency.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationInDependency2.ts`

## Duplicate detection

- `issues/open/232-resolve-local-relative-es-module-graph.md` is the current
  owner. It explicitly keeps package resolution, `node_modules`, import maps,
  TypeScript path mapping, and other non-local module specifiers out of scope
  and rejects bare specifiers with issue-linked diagnostics.
- Broad duplicate candidates from smart triage were generic import/export
  buckets and did not own this exact `import "A"` package specifier boundary.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=2
build_pass=0
unsupported=2
unsupported_diagcodes=UnsupportedModule:2
unsupported_features=import-export:2
```

Both reference files have the same shape:

```ts
// @filename: /node_modules/A/index.d.ts
declare module "ext" {
}
export {};

// @filename: /src/app.ts
import "A"
```

The second file uses `/node_modules/A/index.ts` instead of
`/node_modules/A/index.d.ts`.

Tokens and AST succeed for both files. The AST contains:

```text
ExportNamed { specifiers: [] }
ImportSideEffect { specifier: "A" }
```

Resolved/module graph validation then fails:

```text
error: [UnsupportedModule] issue-232: unsupported non-local module specifier `A`; package resolution, import maps, and absolute specifiers are not implemented
```

TypeScript oracle reports:

```text
TS2664: Invalid module name in augmentation, module 'ext' cannot be found.
TS2882: Cannot find module or type declarations for side-effect import of 'A'.
```

Those diagnostics may become relevant only after a future package-resolution
scope extends beyond issue 232's accepted non-local specifier boundary.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationInDependency --detail --no-dashboard-data
result: pass; executed=2, unsupported=2, UnsupportedModule:2
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInDependency.ts
result: pass; AST succeeds, module_graph reports issue-232 unsupported non-local module specifier `A`
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationInDependency2.ts
result: pass; AST succeeds, module_graph reports issue-232 unsupported non-local module specifier `A`
date: 2026-05-08
```

Remaining risks:

- Future package-resolution work may expose TypeScript oracle diagnostics TS2664
  and TS2882, but that is outside this generated bucket.
