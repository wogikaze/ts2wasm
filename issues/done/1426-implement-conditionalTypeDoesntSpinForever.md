---
id: 1426
title: "Implement Conditionaltypedoesntspinforever"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Triage conditionalTypeDoesntSpinForever across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory
`conditionalTypeDoesntSpinForever` with diagnostics: import-export. Fresh triage
on 2026-05-07 shows the current first blocker is `export enum`, which is already
tracked by issue 5277.

Problem: conditionalTypeDoesntSpinForever has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeDoesntSpinForever.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeDoesntSpinForever.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5277
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Superseding issue 5277 contains an exact triage command and export-enum diagnostic change
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference path and current stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeDoesntSpinForever.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeDoesntSpinForever.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by `issues/open/5277-parse-export-enum-declarations-to-enum-boundary.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionalTypeDoesntSpinForever.ts`

## Duplicate detection

- Superseded by `issues/open/5277-parse-export-enum-declarations-to-enum-boundary.md`.
  The current 1426 first blocker is the same issue-055 `export enum` boundary
  that 5277 owns.
- Generic import/export duplicate candidates are no-match because they cover
  default import, class/function/variable export, namespace, or module graph
  forms rather than exported enum declarations.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: import-export
Diagnostic: UnsupportedModule / unsupported-feature-boundary
Path: reference/typescript/tests/cases/compiler/conditionalTypeDoesntSpinForever.ts
Failure: issue-055: unsupported static export; module resolution and loading are not implemented at 214..220
line: 5, column: 5
Visible symbols before failure: []
```

Source context:

```text
5 | export enum PubSubRecordIsStoredInRedisAsA {
6 |     redisHash = "redisHash",
7 |     jsonEncodedRedisString = "jsonEncodedRedisString"
8 |   }
```

Compiler evidence:

```text
tokens: ok; Export, Ident("enum"), Ident("PubSubRecordIsStoredInRedisAsA"), LeftBrace, ...
ast: false; UnsupportedModule issue-055 at Export
resolved: false; same UnsupportedModule issue-055 diagnostic
```

TypeScript oracle evidence:

```text
TypeScript parses the exported enum and later reports expected semantic diagnostics:
TS2322 for SO_FAR/object assignment and TYPE/undefined assignment, plus TS2769
for Object.keys overload resolution. The ts2wasm first blocker is earlier and
module-syntax owned.
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeDoesntSpinForever.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=type-system:1, build_pass=0, semantic_pass=0, blocked=0
date: 2026-05-07
note: smart triage identifies the first concrete compiler diagnostic as UnsupportedModule issue-055 at export enum.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as superseded by issue 5277; no child issue created.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
