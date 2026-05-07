---
id: 3388
title: "Split moduleResolutionWithSymlinks parser-syntax bucket"
type: maintenance
area: compiler/module-resolution
class: superseded
priority: P1
depends_on: [5000, 227, 232, 5427, 5428]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated bucket after fresh triage. The current failures are not a
parser-syntax implementation slice; they split into type-reference package
resolution and symlinked `node_modules` bare re-export resolution.

## Problem

The original bucket grouped two `moduleResolutionWithSymlinks` reference paths
under `parser-syntax` without smart-triage evidence.

Affected files:

- `reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_referenceTypes.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_preserveSymlinks.ts`

Fresh coverage still reports both as unsupported module-resolution cases, but
the targeted triage identifies separate implementation owners.

## Current failure

`moduleResolutionWithSymlinks_referenceTypes.ts`:

```text
UnsupportedTypeScriptSyntax: issue-227: triple-slash reference types directive for `library-a` requires type package resolution, which is not supported in this milestone
```

`moduleResolutionWithSymlinks_preserveSymlinks.ts`:

```text
UnsupportedModule: issue-232: unsupported non-local module specifier `real` in static re-export
```

The dumps for `preserveSymlinks` also show the later
`/// <reference types="linked" />` directive boundary, but the first actionable
diagnostic for that path is currently the bare static re-export from the
symlinked dependency file.

## Desired final state

This generated bucket remains closed. The actionable work is tracked by focused
successor issues:

- `issues/open/5427-resolve-reference-types-to-virtual-at-types-packages.md`
- `issues/open/5428-resolve-symlinked-node-modules-static-reexports.md`

## Scope

Completed:

- [x] Re-ran coverage for both affected reference files.
- [x] Re-ran smart triage for both affected reference files.
- [x] Confirmed issue 227 only owns the precise unsupported diagnostic, not
      full virtual `@types` package resolution.
- [x] Confirmed issue 232 only owns the local-relative module graph and
      intentional bare-specifier diagnostic, not package resolution.
- [x] Split the remaining implementation work into focused child issues.

Out of scope:

- Direct implementation from this generated bucket.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_referenceTypes.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_preserveSymlinks.ts`

## Acceptance criteria

- [x] Duplicate candidates are confirmed as no-match or partial owners.
- [x] Successor issues include exact reproduction commands.
- [x] Successor issues include failing paths, diagnostics, and source context.
- [x] This bucket is moved to `done/` with current triage evidence.

## Validation

Required commands for this lifecycle slice:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_referenceTypes.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_preserveSymlinks.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_referenceTypes.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_preserveSymlinks.ts
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `5427` for virtual `@types` package resolution from triple-slash types directives
- [x] `5428` for symlinked `node_modules` package static re-exports

## Notes

`issues/done/227-implement-type-reference-directive-resolution.md` remains a
valid completed diagnostic slice. It explicitly left full TypeScript type
package resolution unimplemented by design, so it is not a duplicate of issue
5427.

`issues/done/232-resolve-local-relative-es-module-graph.md` remains a valid
completed local-relative module graph slice. It explicitly left package
resolution and `node_modules` traversal out of scope, so it is not a duplicate
of issue 5428.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_referenceTypes.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_features=module-resolution:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_preserveSymlinks.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_features=module-resolution:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_referenceTypes.ts
result: pass; first diagnostic is issue-227 type-reference package resolution for library-a
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSymlinks_preserveSymlinks.ts
result: pass; first diagnostic is issue-232 unsupported non-local static re-export specifier real
date: 2026-05-08
```

Remaining risks:

- none for this generated bucket; implementation work is tracked by issues
  5427 and 5428.
