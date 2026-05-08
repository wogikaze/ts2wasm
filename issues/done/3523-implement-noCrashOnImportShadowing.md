---
id: 3523
title: "Implement Nocrashonimportshadowing"
type: spike
area: compiler/module-graph
class: superseded
priority: P2
depends_on: [5469, 5438, 5412]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5469-support-named-exports-of-namespace-imports-in-dependency-modules.md`. Fresh triage shows the current actionable issue-5005 boundary is dependency-module `export { B }` over a local namespace-import binding. Existing issues 5438 and 5412 cover known later type-only named export and duplicate-const diagnostic boundaries.

## Problem

Reference test results originally showed 1 case failing in directory `noCrashOnImportShadowing` with diagnostics: duplicate-local. Fresh focused evidence shows two surfaces:

- focused coverage still classifies the representative as `DuplicateLocal`;
- smart triage reaches the dependency-module named export validator first and reports issue-5005 unknown local binding for `export { B }`.

Problem: this generated bucket is not a standalone implementation order. The current actionable module-graph blocker is split to issue 5469, with later known boundaries tracked by existing issues.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnImportShadowing.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnImportShadowing.ts --detail --no-dashboard-data
```

Current coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
semantic_enabled=0
```

Current triage:

```text
UnsupportedSyntax: issue-5005: dependency module `export { B }` references unknown local binding `B` at 100..101
```

Compiler evidence:

```text
tokens: ok through export const zzz, import namespace B, interface B, export { B }, import { B }, duplicate const x, and namespace import OriginalB
ast: contains ImportNamespace local B from "./b", Interface B, ExportNamed local/exported B, ImportNamed B from "./a", and two const x declarations
resolved/module build: stops at issue-5005 for dependency module export { B }
```

TypeScript oracle evidence:

```text
TS2300 duplicate identifier B at the namespace import and interface
TS2451 duplicate block-scoped variable x at both const x declarations
TS2307 cannot find local virtual modules "./a" / "./b" in the oracle run
```

## Desired final state

This generated bucket is closed as superseded after splitting the current module-graph blocker to issue 5469. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage evidence
- [x] Split dependency-module namespace-import named export support to issue 5469
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue
- [x] Link known later owners for named interface export and duplicate const diagnostics

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as split/superseded
- [x] Child issue 5469 contains an exact `reference-triage` command
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnImportShadowing.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnImportShadowing.ts
```

Not run:

- `cargo fmt --all --check` / `cargo nextest run`: issue metadata-only split; no Rust implementation changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5469-support-named-exports-of-namespace-imports-in-dependency-modules.md`
- [x] existing: `issues/open/5438-support-named-exports-of-local-interfaces.md`
- [x] existing: `issues/open/5412-report-ts2451-duplicate-const-filename-sections.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noCrashOnImportShadowing.ts`

## Duplicate detection

- `issues/open/5438-support-named-exports-of-local-interfaces.md`: related type-only named export owner, but not exact for namespace-import value export from a dependency module.
- `issues/open/5412-report-ts2451-duplicate-const-filename-sections.md`: related duplicate-const diagnostic owner for the broad coverage classification.
- No exact existing implementation-ready owner was found for dependency-module `export { B }` over a namespace-import value binding, so issue 5469 was created.

## Smart triage

Generated on 2026-05-08.

```text
### Smart triage: Triage import export: noCrashOnImportShadowing

- Issue class: triage-needed
- Feature label: import-export
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/noCrashOnImportShadowing.ts

UnsupportedSyntax: issue-5005: dependency module `export { B }` references unknown local binding `B` at 100..101
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
