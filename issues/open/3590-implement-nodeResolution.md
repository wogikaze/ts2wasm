---
id: 3590
title: "Implement Noderesolution"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5283, 5285, 5484, 5485]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nodeResolution across 8 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 8 cases fail in directory `nodeResolution`.

Fresh coverage and triage show this generated bucket is not one executable
work item. It splits into existing export-var parser blockers and a new
virtual `node_modules` declaration resolution blocker.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nodeResolution2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nodeResolution2.ts --detail
```

## Desired final state

This generated bucket is split/superseded by narrower issues:

- `issues/open/5283-support-entry-export-var-declarations.md`
- `issues/open/5285-support-export-var-initializer-declarations.md`
- `issues/open/5484-resolve-import-equals-require-to-virtual-node-modules-declarations.md`
- `issues/open/5485-resolve-import-equals-require-to-virtual-node-modules-index-declarations.md`

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeResolution --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution4.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution5.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution6.ts
```

Not run:

- cargo fmt --all --check: metadata-only issue split/close
- cargo nextest run: metadata-only issue split/close

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5484-resolve-import-equals-require-to-virtual-node-modules-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nodeResolution2.ts`
- `reference/typescript/tests/cases/compiler/nodeResolution1.ts`
- `reference/typescript/tests/cases/compiler/nodeResolution6.ts`
- `reference/typescript/tests/cases/compiler/nodeResolution4.ts`
- `reference/typescript/tests/cases/compiler/nodeResolution3.ts`
- `reference/typescript/tests/cases/compiler/nodeResolution5.ts`
- `reference/typescript/tests/cases/compiler/nodeResolution7.ts`
- `reference/typescript/tests/cases/compiler/nodeResolution8.ts`

## Duplicate detection

- `issues/open/5283-support-entry-export-var-declarations.md` covers simple
  typed and untyped `export var` declarations without initializers.
- `issues/open/5285-support-export-var-initializer-declarations.md` covers
  initialized `export var` declarations.
- `issues/open/5295-resolve-import-equals-require-to-virtual-node-modules-class-export.md`
  is related but not a duplicate: it covers class exports from virtual
  `node_modules` source sections, not declaration-file package shapes.
- `issues/open/5229a-resolve-imports-between-filename-sections.md` is related
  but not a duplicate: it covers local static import/export source specifiers,
  not bare `import = require("pkg")` package lookup.

## Smart triage

Generated on 2026-05-08.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeResolution --detail --no-dashboard-data
result: executed=8; unsupported=8; UnsupportedModule=4; UnsupportedSyntax=4; import-export=4; module-resolution=4
per-file:
  nodeResolution1.ts: UnsupportedSyntax/module-resolution
  nodeResolution2.ts: UnsupportedSyntax/module-resolution
  nodeResolution3.ts: UnsupportedSyntax/module-resolution
  nodeResolution4.ts: UnsupportedSyntax/module-resolution
  nodeResolution5.ts: UnsupportedModule/import-export
  nodeResolution6.ts: UnsupportedModule/import-export
  nodeResolution7.ts: UnsupportedModule/import-export
  nodeResolution8.ts: UnsupportedModule/import-export
```

Representative smart triage:

```text
nodeResolution1.ts:
  headline: issue-055 unsupported variable export at `export var x = 1;`
  owner: issues/open/5285-support-export-var-initializer-declarations.md

nodeResolution2.ts:
  headline: issue-055 unsupported variable export at `export var x: number;`
  owner: issues/open/5283-support-entry-export-var-declarations.md

nodeResolution4.ts:
  headline: issue-055 unsupported variable export at `export var y;`
  owner: issues/open/5283-support-entry-export-var-declarations.md

nodeResolution5.ts:
  ast: ok; ImportDefault source "a"
  resolved: issue-232 unsupported non-local module specifier `a`
  owner: issues/open/5484-resolve-import-equals-require-to-virtual-node-modules-declarations.md

nodeResolution6.ts:
  ast: ok; var x, export declare var y, ImportDefault source "a"
  resolved/wat: issue-232 unsupported non-local module specifier `a`
  owner: issues/open/5484-resolve-import-equals-require-to-virtual-node-modules-declarations.md

nodeResolution7.ts and nodeResolution8.ts:
  expected owner: issues/open/5485-resolve-import-equals-require-to-virtual-node-modules-index-declarations.md
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `4786744d`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeResolution --detail --no-dashboard-data
result: split into existing issues 5283/5285 and new issues 5484/5485
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution1.ts
result: current blocker is initialized export-var issue-055, owned by issue 5285
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeResolution5.ts
result: current blocker is virtual node_modules declaration package resolution, split to issue 5484
date: 2026-05-08
```

Remaining risks:

- `nodeResolution3.ts` is expected to follow the same typed export-var shape as
  `nodeResolution2.ts`.
- `nodeResolution7.ts` and `nodeResolution8.ts` are expected to follow the
  same virtual `node_modules/a/index.d.ts` package-resolution shape as issue
  5485.
- After 5283/5285/5484 advance, this bucket may expose narrower bundler
  module-resolution parity gaps.
