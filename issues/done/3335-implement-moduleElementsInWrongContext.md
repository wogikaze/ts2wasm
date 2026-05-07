---
id: 3335
title: "Implement Moduleelementsinwrongcontext"
type: spike
area: frontend/syntax
class: split
priority: P1
depends_on: []
blocks: [5413]
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import/export bucket by splitting its current parser and
semantic blockers:

- `issues/open/5186-parse-export-assignment-for-diagnostics.md` owns the
  immediate `export = M;` parser boundary seen in two cases.
- `issues/open/5413-report-module-elements-in-wrong-context.md` owns the first
  currently reachable wrong-context diagnostic for a nested namespace
  declaration.

## Problem

The bucket groups three `moduleElementsInWrongContext*` reference cases. Fresh
coverage shows two still stop at a generic unsupported static export boundary,
while the namespace-wrapped variant already builds but does not surface
TypeScript's wrong-context diagnostics.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleElementsInWrongContext --detail --no-dashboard-data
```

Observed result:

```text
executed=3
build_pass=1
unsupported=2
unsupported_diagcodes=UnsupportedSyntax:2
unsupported_features=import-export:2
```

Per-file result:

```text
reference/typescript/tests/cases/compiler/moduleElementsInWrongContext.ts: UnsupportedSyntax: import-export
reference/typescript/tests/cases/compiler/moduleElementsInWrongContext2.ts: UnsupportedSyntax: import-export
reference/typescript/tests/cases/compiler/moduleElementsInWrongContext3.ts: build_pass
```

Focused triage for the first two cases:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleElementsInWrongContext.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleElementsInWrongContext2.ts
```

Current compiler evidence:

```text
moduleElementsInWrongContext.ts:
  UnsupportedModule issue-055 unsupported static export at 183..189
  source context: export = M; inside a bare block

moduleElementsInWrongContext2.ts:
  UnsupportedModule issue-055 unsupported static export at 200..206
  source context: export = M; inside a function body
```

`issues/open/5186-parse-export-assignment-for-diagnostics.md` is the existing
implementation-ready owner for representing `export = expr;` so later
diagnostics can use the exported expression span.

Focused triage for the third case:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleElementsInWrongContext3.ts
```

Current compiler evidence:

```text
BuildPass: ts2wasm build succeeded
```

First TypeScript oracle diagnostic:

```text
TS1235: A namespace declaration is only allowed at the top level of a namespace or module.
```

## Desired final state

Implement the split issues instead of working from this generated bucket.

## Scope

In scope:

- [x] Confirm the current first blocker for each affected reference file.
- [x] Link the `export = M;` parser blocker to existing issue 5186.
- [x] Split the first currently reachable wrong-context diagnostic into issue 5413.
- [x] Preserve exact reproduction commands and representative diagnostics.

Out of scope:

- Direct implementation from this generated bucket.
- CommonJS export assignment runtime/module loading.
- General import/export module graph resolution.

## Affected paths

Expected implementation owners:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `crates/frontend/src/`
- focused parser/diagnostic fixtures

Do not touch from this bucket:

- backend emit
- package/module resolution

## Acceptance criteria

- [x] Existing parser owner recorded: `issues/open/5186-parse-export-assignment-for-diagnostics.md`.
- [x] New semantic diagnostics owner created: `issues/open/5413-report-module-elements-in-wrong-context.md`.
- [x] The generated bucket no longer remains as a stale blocked import/export issue.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
git diff --cached --check
```

Reference commands already run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleElementsInWrongContext --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleElementsInWrongContext.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleElementsInWrongContext2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleElementsInWrongContext3.ts
```

Not run:

- `cargo fmt --all --check` (issue lifecycle only; no Rust changes)
- `cargo nextest run` (issue lifecycle only; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5413-report-module-elements-in-wrong-context.md`

## Notes

Split into issue 5413 and existing issue 5186.

## Completion evidence

Commits:

- filled by commit

Validation result:

```text
command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-08

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-08

command: git diff --check
result: pass
date: 2026-05-08

command: git diff --cached --check
result: pass
date: 2026-05-08
```

Remaining risks:

- Later wrong-context diagnostics for ambient module declarations, export
  forms, and import forms should be split after issue 5413 lands and fresh
  triage exposes the next blocker.
