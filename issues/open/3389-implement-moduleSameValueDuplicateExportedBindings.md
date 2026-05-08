---
id: 3389
title: "Close moduleSameValueDuplicateExportedBindings bucket to virtual re-export owner"
type: maintenance
area: compiler/module-graph
class: superseded
priority: P1
depends_on: [432, 5229]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import-export bucket as superseded by issue 5229.
Fresh triage shows both reference paths stop before duplicate exported-binding
semantics at the virtual `@filename` re-export resolution boundary.

## Problem

The original bucket grouped two `moduleSameValueDuplicateExportedBindings`
reference files without smart-triage evidence.

Affected files:

- `reference/typescript/tests/cases/compiler/moduleSameValueDuplicateExportedBindings1.ts`
- `reference/typescript/tests/cases/compiler/moduleSameValueDuplicateExportedBindings2.ts`

Both files start in virtual `a.ts` with:

```ts
export * from "./b";
export * from "./c";
```

The compiler reports missing `./b` instead of resolving the sibling virtual
`b.ts` section.

## Current failure

Fresh triage for both files reports:

```text
UnsupportedModule: issue-232: missing local module `./b` re-exported from a.ts; tried ./b.ts, ./b.js
```

The dumps show later unsupported export forms (`export var foo = 42` in file 1
and `export enum Animals` in file 2), but those are not yet actionable until
the `./b` / `./c` virtual re-export graph resolves.

## Desired final state

This generated bucket remains closed. The first actionable blocker is owned by
`issues/open/5229-resolve-imports-between-filename-sections.md`.

## Scope

Completed:

- [x] Re-ran focused coverage for both listed files.
- [x] Re-ran smart triage for both listed files.
- [x] Confirmed both files match issue 5229's virtual-section local re-export
      resolution scope.
- [x] Added an ownership note to issue 5229.

Out of scope:

- Direct implementation from this generated bucket.
- Duplicate exported-binding semantics after virtual re-export resolution.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduleSameValueDuplicateExportedBindings1.ts`
- `reference/typescript/tests/cases/compiler/moduleSameValueDuplicateExportedBindings2.ts`

## Acceptance criteria

- [x] Current first diagnostic is recorded for both paths.
- [x] Matching owner issue 5229 is identified.
- [x] This bucket is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleSameValueDuplicateExportedBindings1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleSameValueDuplicateExportedBindings2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleSameValueDuplicateExportedBindings1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleSameValueDuplicateExportedBindings2.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Issue 5229 already covers resolving local imports and re-exports between
reference `@filename` sections. The current `export * from "./b"` failure is
the same first blocker.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: focused coverage and triage commands listed above
result: pass; both files currently stop at issue-232 missing local module ./b
date: 2026-05-08
```

Remaining risks:

- Duplicate exported-binding diagnostics may need a later issue after issue
  5229 resolves the virtual re-export graph.
