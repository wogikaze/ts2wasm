---
id: 5283
title: "Support entry-module export var declarations"
type: feature
area: frontend/module-syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Support the narrow entry-module `export var name: type;` declaration form far
enough to advance past the current issue-055 variable export boundary.

## Problem

`commentsBeforeVariableStatement1.ts` tokenizes the exported variable
declaration, but the parser/module frontend stops immediately with issue-055:

```text
UnsupportedModule: issue-055: unsupported variable export; module resolution and loading are not implemented at 83..89
```

Problem: `export var b: number;` currently stops at the generic unsupported variable export boundary before the declaration can be parsed and triaged.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts --detail --no-dashboard-data
```

Observed result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 83..89
```

Source context:

```ts
/** b's comment*/
export var b: number;
```

Compiler evidence:

```text
tokens: ok; Export, Var, Ident("b"), Colon, Ident("number"), Semicolon
ast: fails at issue-055 variable export boundary
TypeScript oracle: ok, binding b has type number
```

## Desired final state

The frontend accepts the `export var b: number;` declaration form and records a
named variable export, or advances to the next narrower semantic/module blocker.

## Scope

In scope:

- [x] Parse simple entry-module `export var name: type;` with focused coverage, then re-run the representative reference triage and confirm the issue-055 variable export boundary is gone.

Out of scope:

- `export let` destructuring, tracked separately by `issues/done/5175-support-export-let-destructuring-declarations.md`.
- `export function`, `export class`, and `export enum` slices.
- Full AMD emit or comment-preservation fidelity.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `crates/compiler/src/lib.rs`
- focused parser/module tests

Do not touch:

- backend/runtime ABI unless existing export metadata cannot represent a simple variable export
- unrelated import/export forms

## Acceptance criteria

- [x] `export var b: number;` parses without `issue-055: unsupported variable export`, while unrelated unsupported import/export diagnostics still report issue-055.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts` no longer reports the variable export boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export
cargo nextest run -p ts2wasm-cli module
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts --detail --no-dashboard-data
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

Split from `issues/open/1360-implement-commentsBeforeVariableStatement.md`.

Related but not duplicates:

- `issues/done/5175-support-export-let-destructuring-declarations.md` covers
  `export let` destructuring and explicitly excludes general `export var`.
- `issues/open/5144-support-entry-export-function-declarations.md`,
  `issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`, and
  `issues/done/5277-parse-export-enum-declarations-to-enum-boundary.md` cover
  sibling export forms.
- `issues/open/432-implement-import-export.md` is the broad generated
  import/export bucket and is too wide to implement directly.
- `issues/open/3560-implement-noImplicitUseStrict.md` reaches the same
  `export var x = 0;` issue-055 boundary across commonjs, amd, es6, system,
  and umd module-target variants.
- Also owns the non-initialized export-var subset of
  `issues/open/3590-implement-nodeResolution.md`.

## Completion evidence

Commits:

- `524858ae35` (implementation by wogikaze)

Validation result:

```text
cargo nextest run -p ts2wasm-frontend export
=> 20 tests run: 20 passed, 206 skipped

cargo nextest run -p ts2wasm-cli module
=> 27 tests run: 27 passed, 681 skipped

python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts
=> BuildPass — no issue-055 variable export boundary
date: 2026-05-08
```

Remaining risks:

- none
## False-done audit

**truly-done** (5283)

- Implementation commits: verified via `git log --oneline --all --grep=5283`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
