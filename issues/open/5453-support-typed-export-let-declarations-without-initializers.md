---
id: 5453
title: "Support typed export let declarations without initializers"
type: feature
area: frontend/module-syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Support the narrow `export let name: Type;` declaration form far enough to
advance past the current issue-055 variable export boundary.

Split from generated bucket
`issues/open/3462-implement-narrowingPastLastAssignmentInModule.md`.

## Problem

Problem: `narrowingPastLastAssignmentInModule.ts` tokenizes the exported `let`
declaration, but the parser/module frontend stops immediately at the leading
`export` with issue-055:

```ts
export let x1: string | number;
```

Current diagnostic:

```text
UnsupportedModule: issue-055: unsupported variable export; module resolution and loading are not implemented at 159..165
```

This blocks the reference before the later closure narrowing behavior for
exported mutable variables can be triaged.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingPastLastAssignmentInModule.ts
```

Representative source:

```ts
function action(f: Function) {}

export let x1: string | number;
x1 = "abc";
action(() => { x1 /* string | number */ });
```

Compiler evidence:

```text
tokens: ok; Export, Let, Ident("x1"), Colon, Ident("string"), Pipe, Ident("number"), Semicolon
ast: fails at issue-055 variable export boundary before creating a declaration node
resolved: same issue-055 module boundary
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

The frontend accepts `export let name: Type;` and records or erases the export
metadata consistently with the current module slice. The representative path
should advance past the current `159..165` issue-055 diagnostic to the next
parser/module/narrowing blocker.

## Scope

In scope:

- [ ] Parse a simple typed `export let <identifier>: <type>;` declaration with
  no initializer.
- [ ] Preserve existing behavior for initialized `export let` and export-let
  destructuring work tracked by issue 5175.
- [ ] Add focused parser/module coverage for
  `export let value: string | number;`.
- [ ] Re-run the representative triage and record any later blocker.

Out of scope:

- Initialized `export let` and export-let destructuring forms, tracked by
  `issues/done/5175-support-export-let-destructuring-declarations.md`.
- `export var`, tracked by
  `issues/done/5283-support-entry-export-var-declarations.md`.
- Named export declarations such as `export { x2 };`.
- Default exports such as `export default x4;`.
- Full module graph loading or emit fidelity.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- focused parser/module tests

Do not touch:

- backend/runtime ABI unless existing export metadata cannot represent this
  declaration form
- unrelated import/export forms

## Acceptance criteria

- [ ] `export let value: string | number;` parses without
  `issue-055: unsupported variable export`.
- [ ] Existing `export const value = 1;` and focused export-let initializer
  fixtures still pass.
- [ ] `narrowingPastLastAssignmentInModule.ts` no longer reports issue-055 at
  the leading `export` for `x1`.
- [ ] Any later `export { x2 }`, `export default x4`, ASI, or narrowing blocker
  is recorded here or split into a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingPastLastAssignmentInModule.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingPastLastAssignmentInModule.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Related but distinct:

- Issue 5175 owns initialized `export let` and destructuring export-let forms.
- Issue 5283 owns `export var name: type;`.
- Issue 432 is the broad import/export bucket and should not be selected
  directly when this narrow issue exists.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
