---
id: 1507
title: "Implement Contextualtypebasedonintersectionwithanyinthemix Name Resolution"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1507.

## Summary

Triage contextualTypeBasedOnIntersectionWithAnyInTheMix-name-resolution across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail in directory `contextualTypeBasedOnIntersectionWithAnyInTheMix-name-resolution` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypeBasedOnIntersectionWithAnyInTheMix-name-resolution has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix2.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale because all affected paths now build-pass
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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

- [x] Duplicate candidates below are confirmed; no child issue is needed
- [x] No child issue created because the compiler now reports build_pass for all affected paths
- [x] This issue includes affected paths, diagnostic status, source context, visible symbols, and parser/TypeScript evidence
- [x] Completion evidence names the exact reference paths and build-pass results

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 8
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current compiler build has no name-resolution blocker on these paths

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix2.ts`
- `reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix3.ts`
- `reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix4.ts`
- `reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix5.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/done/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)
- `issues/done/648-implement-argumentsAsPropertyName-name-resolution.md` - Implement Argumentsaspropertyname Name Resolution (same feature label, title overlap)
- `issues/done/654-implement-argumentsReferenceInConstructor-name-resolution.md` - Implement Argumentsreferenceinconstructor Name Resolution (same feature label, title overlap)
- `issues/done/657-implement-argumentsReferenceInMethod-name-resolution.md` - Implement Argumentsreferenceinmethod Name Resolution (same feature label, title overlap)
- `issues/done/693-implement-arrayToLocaleStringES-name-resolution.md` - Implement Arraytolocalestringes Name Resolution (same feature label, title overlap)
- `issues/done/733-implement-assignmentCompatability-name-resolution.md` - Implement Assignmentcompatability Name Resolution (same feature label, title overlap)

## Smart triage

Fresh coverage on 2026-05-07 shows the generated name-resolution bucket is stale:

- `contextualTypeBasedOnIntersectionWithAnyInTheMix2.ts`: `build_pass=1`, `blocked=0`, `unsupported_diagcodes=[]`, `unsupported_features=[]`; `reference-triage` reports `BuildPass` and TypeScript oracle diagnostics `[]`.
- `contextualTypeBasedOnIntersectionWithAnyInTheMix3.ts`: `build_pass=1`, `blocked=0`, `unsupported_diagcodes=[]`, `unsupported_features=[]`; `reference-triage` reports `BuildPass`. The TypeScript oracle still reports TS2322 for `const d`, which is semantic parity evidence rather than a ts2wasm build blocker.
- `contextualTypeBasedOnIntersectionWithAnyInTheMix4.ts`: `build_pass=1`, `blocked=0`, `unsupported_diagcodes=[]`, `unsupported_features=[]`; `reference-triage` reports `BuildPass`. The TypeScript oracle still reports TS2322 string-literal assignability diagnostics, which are semantic parity evidence rather than a ts2wasm build blocker.
- `contextualTypeBasedOnIntersectionWithAnyInTheMix5.ts`: `build_pass=1`, `blocked=0`, `unsupported_diagcodes=[]`, `unsupported_features=[]`; `reference-triage` reports `BuildPass` and TypeScript oracle diagnostics `[]`.

No child issue is required for the original generated name-resolution blocker.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter <each affected contextualTypeBasedOnIntersectionWithAnyInTheMix*.ts> --detail --no-dashboard-data
result: pass; all four affected files are build_pass with blocked=0 and empty unsupported diagnostics/features
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix2.ts
result: pass; BuildPass; TypeScript oracle diagnostics=[]
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix3.ts
result: pass; BuildPass; TypeScript oracle reports semantic TS2322 on const d
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix4.ts
result: pass; BuildPass; TypeScript oracle reports semantic TS2322 string-literal assignability diagnostics
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix5.ts
result: pass; BuildPass; TypeScript oracle diagnostics=[]
date: 2026-05-07
```

Remaining risks:

- TypeScript semantic diagnostics in cases 3 and 4 remain outside this build-blocker cleanup because semantic parity is not the current gate.
