---
id: 1428
title: "Implement Conditionaltypesubclassextendstypeparam"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: [5002]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1428.

## Summary

Triage conditionalTypeSubclassExtendsTypeParam across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory
`conditionalTypeSubclassExtendsTypeParam` with diagnostics: type-system. Fresh
triage on 2026-05-07 shows the compiler now builds the representative
successfully, and TypeScript oracle also reports no diagnostics.

Problem: conditionalTypeSubclassExtendsTypeParam has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeSubclassExtendsTypeParam.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeSubclassExtendsTypeParam.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale build-pass bucket
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

- [x] Duplicate candidates below are confirmed as no-match for required implementation work
- [x] No child issue needed because the current compiler result is BuildPass
- [x] This issue includes the reference path, build-pass result, visible symbols, and parser evidence
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeSubclassExtendsTypeParam.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conditionalTypeSubclassExtendsTypeParam.ts
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

## Affected test files

- `reference/typescript/tests/cases/compiler/conditionalTypeSubclassExtendsTypeParam.ts`

## Duplicate detection

- No existing open/done issue is needed. Fresh triage shows this generated
  bucket is stale.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: build-pass
Diagnostic: BuildPass / pass
Path: reference/typescript/tests/cases/compiler/conditionalTypeSubclassExtendsTypeParam.ts
Failure: ts2wasm build succeeded
Source overview: 208 bytes, 7 lines
Visible symbols:
- class Model
- class Field
```

Compiler evidence:

```text
tokens: ok
ast: ok; AST is empty because the declarations/types are erased for runtime build
resolved: ok; resolved program is empty
TypeScript oracle: ok, diagnostics=[]
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conditionalTypeSubclassExtendsTypeParam.ts --detail --no-dashboard-data
result: executed=1, build_pass=1, semantic_pass=0, unsupported=0, blocked=0, semantic_enabled=0
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale build-pass bucket; no child issue created.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
