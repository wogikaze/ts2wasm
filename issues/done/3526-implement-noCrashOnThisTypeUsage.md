---
id: 3526
title: "Implement Nocrashonthistypeusage"
type: spike
area: ir/compiler
class: superseded
priority: P1
depends_on: [5232]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5232-support-entry-export-class-declarations.md`. Fresh triage reports the current module boundary as entry-module `export class ObservableValue<T>` issue-5005.

## Problem

Reference test results originally showed 1 case failing in directory `noCrashOnThisTypeUsage` with diagnostics: import-export. Fresh triage still reports import/export, with the concrete issue-5005 export-class boundary:

```text
UnsupportedModule: issue-5005: entry module `export ObservableValue` uses a declaration form outside the current static export slice; only export const and export default are supported
```

Problem: this generated bucket is not a standalone implementation order. The current reported blocker is already owned by issue 5232.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnThisTypeUsage.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnThisTypeUsage.ts --detail --no-dashboard-data
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Triage evidence:

```text
Feature label: import-export
Diagnostic: UnsupportedModule / unsupported-feature-boundary
Message: issue-5005: entry module `export ObservableValue` uses a declaration form outside the current static export slice
```

Compiler dump note:

```text
tokens: ok through interface, notifyListeners, export class ObservableValue, constructor parameter property, and observe method
ast: ok; contains ExportDecl(ClassDecl ObservableValue)
resolved dump: also shows later issue-289 constructor capture for outer local notifyListeners
stack trace: reported blocker remains UnsupportedModule issue-5005 for entry-module export class
```

TypeScript oracle:

```text
ok; diagnostics=[]
```

## Desired final state

This generated bucket is closed as superseded by issue 5232. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage evidence
- [x] Confirm issue 5232 covers the current entry-module export-class boundary
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue
- [x] Record the later issue-289 constructor capture risk without creating a premature duplicate

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

- [x] Duplicate candidates below are confirmed as superseded by issue 5232
- [x] Superseding issue contains exact `reference-triage` commands
- [x] Closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnThisTypeUsage.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnThisTypeUsage.ts
```

Not run:

- `cargo fmt --all --check` / `cargo nextest run`: issue metadata-only supersession; no Rust implementation changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing: `issues/open/5232-support-entry-export-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noCrashOnThisTypeUsage.ts`

## Duplicate detection

- `issues/open/5232-support-entry-export-class-declarations.md`: exact owner for the current entry-module `export class` issue-5005 boundary.
- Later issue-289 constructor capture evidence may require a narrower follow-up after issue 5232 advances this reference.

## Smart triage

Generated on 2026-05-08.

```text
Feature label: import-export
Diagnostic: UnsupportedModule / unsupported-feature-boundary
Message: issue-5005: entry module `export ObservableValue` uses a declaration form outside the current static export slice
```

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
