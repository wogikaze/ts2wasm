---
id: 3572
title: "Implement Nounusedlocals Import Export"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage noUnusedLocals-import-export across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows the representative path now build-passes and TypeScript
reports no diagnostics.

Problem: the original import-export blocker for
`noUnusedLocals_selfReference_skipsBlockLocations.ts` is stale.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference_skipsBlockLocations.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference_skipsBlockLocations.ts --detail
```

## Desired final state

This generated bucket is closed as stale build-pass evidence. Do not implement
directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm current behavior with focused coverage and triage
- [x] Close as stale build-pass evidence
- [x] Preserve exact reproduction commands and representative evidence

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

- [x] Duplicate candidates below are confirmed as no-match or stale
- [x] Focused coverage shows `build_pass`
- [x] Focused triage shows TypeScript oracle diagnostics are empty
- [x] No child issue is needed for this representative path

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference_skipsBlockLocations.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference_skipsBlockLocations.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only stale bucket close.
- `cargo nextest run`; metadata-only stale bucket close.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference_skipsBlockLocations.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/open/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/open/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/open/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

### Smart triage: Build pass: noUnusedLocals selfReference skipsBlockLocations

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference_skipsBlockLocations.ts`

Current evidence:

```text
coverage: build_pass=1 unsupported=0
triage: BuildPass / pass
TypeScript oracle: diagnostics=[]
```

Compiler evidence:

```text
tokens: ok through namespace n, function f, switch case function g, default function h
ast: ok; namespace is erased for runtime
resolved: ok
```

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference_skipsBlockLocations.ts --detail --no-dashboard-data
result: pass; representative path reports build_pass
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference_skipsBlockLocations.ts
result: pass; fresh triage shows BuildPass and TypeScript oracle diagnostics=[]
date: 2026-05-08
```

Remaining risks:

- none
