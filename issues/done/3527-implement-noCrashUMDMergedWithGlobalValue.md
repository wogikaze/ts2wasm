---
id: 3527
title: "Implement Nocrashumdmergedwithglobalvalue"
type: spike
area: frontend/parser
class: superseded
priority: P1
depends_on: [5231]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5231-parse-export-as-namespace-declarations.md`. Fresh triage shows the current blocker is `export as namespace SomeInterface;`, the exact declaration form owned by issue 5231.

## Problem

Reference test results originally showed 1 case failing in directory `noCrashUMDMergedWithGlobalValue` with diagnostics: import-export. Fresh triage reports:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 46..52
```

Problem: this generated bucket is not a standalone implementation order. The current concrete parser boundary is already tracked by issue 5231.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashUMDMergedWithGlobalValue.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashUMDMergedWithGlobalValue.ts --detail --no-dashboard-data
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

Compiler evidence:

```text
tokens: ok; Export Ident("as") Ident("namespace") Ident("SomeInterface") Semicolon
ast: fails before AST with UnsupportedModule issue-055 at export as namespace
resolved: same issue-055 boundary
```

TypeScript oracle:

```text
TS1315: Global module exports may only appear in declaration files.
TypeScript AST: NamespaceExportDeclaration `export as namespace SomeInterface;`
```

## Desired final state

This generated bucket is closed as superseded by issue 5231. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage evidence
- [x] Confirm issue 5231 covers the current `export as namespace` boundary
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue
- [x] Update issue 5231 with this representative path

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

- [x] Duplicate candidates below are confirmed as superseded by issue 5231
- [x] Superseding issue contains an exact `reference-triage` command
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashUMDMergedWithGlobalValue.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashUMDMergedWithGlobalValue.ts
```

Not run:

- `cargo fmt --all --check` / `cargo nextest run`: issue metadata-only supersession; no Rust implementation changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing: `issues/open/5231-parse-export-as-namespace-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noCrashUMDMergedWithGlobalValue.ts`

## Duplicate detection

- `issues/open/5231-parse-export-as-namespace-declarations.md`: exact owner for the current `export as namespace` issue-055 boundary.

## Smart triage

Generated on 2026-05-08.

```text
Feature label: import-export
Diagnostic: UnsupportedModule / unsupported-feature-boundary
Message: issue-055: unsupported static export; module resolution and loading are not implemented at 46..52
TypeScript AST: NamespaceExportDeclaration
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
