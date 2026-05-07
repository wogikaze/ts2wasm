---
id: 5285
title: "Support export var initializer declarations"
type: feature
area: frontend/module-syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Accept one initialized entry-module `export var name = expr;` declaration shape
far enough that it no longer stops at the generic issue-055 variable export
boundary.

## Problem

`commentsExternalModules2.ts` parses through its import-equals statement and
member-use expressions, then stops on an exported variable declaration with an
initializer:

```text
issue-055: unsupported variable export; module resolution and loading are not implemented
```

Problem: `export var newVar = new extMod.m1.m2.c();` currently cannot be
represented as an exported variable declaration.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules2.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsExternalModules --detail --no-dashboard-data
```

Observed result:

```text
commentsExternalModules2.ts: issue-055 unsupported variable export at 1313..1319
```

Source context:

```ts
import extMod = require("commentsExternalModules2_0");
extMod.m1.fooExport();
export var newVar = new extMod.m1.m2.c();
extMod.m4.fooExport();
```

Compiler evidence:

```text
tokens: ok through import-equals, member calls, and `export var newVar = ...`
ast/resolved: issue-055 unsupported variable export before `newVar` is represented
TypeScript oracle: exported variable is a top-level FirstStatement; TS2307 is only for the missing module specifier
```

## Desired final state

The frontend accepts initialized `export var name = expr;` declarations and
records the named export, or advances to the next narrower module/import
boundary.

## Scope

In scope:

- [ ] Parse initialized `export var name = expr;` declarations with focused coverage, then re-run `commentsExternalModules2.ts` and confirm the issue-055 variable export boundary is gone.

Out of scope:

- Simple typed export-var declarations without initializers, tracked by `issues/open/5283-support-entry-export-var-declarations.md`.
- Exported `let` destructuring, tracked by `issues/open/5175-support-export-let-destructuring-declarations.md`.
- Package/bare specifier resolution, which is out of scope for completed issue 232.
- Full external module execution and comment/declaration emit fidelity.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- focused module/export tests

Do not touch:

- package resolution or import-map behavior

## Acceptance criteria

- [ ] `commentsExternalModules2.ts` no longer reports issue-055 `unsupported variable export` at `export var newVar`.
- [ ] A focused fixture covers `export var name = expr;` and preserves unrelated unsupported module specifier diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsExternalModules --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from `issues/done/1366-implement-commentsExternalModules.md`.

Related but not duplicates:

- `issues/open/5283-support-entry-export-var-declarations.md` covers simple typed `export var name: type;`.
- `issues/open/5175-support-export-let-destructuring-declarations.md` covers exported `let` destructuring.
- `issues/done/232-resolve-local-relative-es-module-graph.md` covers intentional bare/non-local specifier rejection.
- `commentsExternalModules3.ts` has the same initialized export-var shape and
  should be rechecked after this representative slice advances.
- `issues/done/1376-implement-commentsOnRequireStatement.md` reaches the same
  initialized `export var subject = 10;` boundary before its later re-export
  and missing-module diagnostics.

## Completion evidence

Fill when implemented.
