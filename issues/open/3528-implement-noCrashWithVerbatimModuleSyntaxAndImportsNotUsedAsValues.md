---
id: 3528
title: "Implement Nocrashwithverbatimmodulesyntaxandimportsnotusedasvalues"
type: spike
area: ir/compiler
class: superseded
priority: P1
depends_on: [5324]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5324-support-dependency-export-class-declarations.md`. Fresh triage reports the current blocker as dependency-module `export class A {}` issue-5005.

## Problem

Reference test results originally showed 1 case failing in directory `noCrashWithVerbatimModuleSyntaxAndImportsNotUsedAsValues` with diagnostics: import-export. Fresh triage reports:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form outside the current static export slice at 7..17
```

Problem: this generated bucket is not a standalone implementation order. The current reported blocker is already owned by issue 5324.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashWithVerbatimModuleSyntaxAndImportsNotUsedAsValues.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashWithVerbatimModuleSyntaxAndImportsNotUsedAsValues.ts --detail --no-dashboard-data
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
semantic_enabled=0
```

Compiler evidence:

```text
tokens: ok through export class A, import {A} from "./file", and const a
ast: ok; contains ExportDecl(ClassDecl A), ImportNamed A from "./file", and let a
stack trace: UnsupportedModule issue-5005 for dependency-module export class
resolved dump: later issue-232 missing local module "./file" after AST/module graph
```

TypeScript oracle:

```text
TS2395: Individual declarations in merged declaration 'A' must be all exported or all local.
TS2307: Cannot find module './file' or its corresponding type declarations.
```

## Desired final state

This generated bucket is closed as superseded by issue 5324. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage evidence
- [x] Confirm issue 5324 covers the current dependency export-class boundary
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue
- [x] Record later `./file` virtual import-resolution risk without duplicating issue 5229

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

- [x] Duplicate candidates below are confirmed as superseded by issue 5324
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashWithVerbatimModuleSyntaxAndImportsNotUsedAsValues.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashWithVerbatimModuleSyntaxAndImportsNotUsedAsValues.ts
```

Not run:

- `cargo fmt --all --check` / `cargo nextest run`: issue metadata-only supersession; no Rust implementation changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing: `issues/open/5324-support-dependency-export-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noCrashWithVerbatimModuleSyntaxAndImportsNotUsedAsValues.ts`

## Duplicate detection

- `issues/open/5324-support-dependency-export-class-declarations.md`: exact owner for the current dependency-module `export class` issue-5005 boundary.
- `issues/open/5229a-resolve-imports-between-filename-sections.md`: likely later owner for the `./file` virtual import-resolution boundary after 5324 advances this reference.

## Smart triage

Generated on 2026-05-08.

```text
Feature label: import-export
Diagnostic: UnsupportedModule / unsupported-feature-boundary
Message: issue-5005: dependency module declaration export uses a form outside the current static export slice at 7..17
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
