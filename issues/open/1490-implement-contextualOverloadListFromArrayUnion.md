---
id: 1490
title: "Implement Contextualoverloadlistfromarrayunion"
type: spike
area: frontend/syntax
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
> Evidence: Empty completion evidence. No feat/fix commit for #1490.

## Summary

Triage contextualOverloadListFromArrayUnion across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualOverloadListFromArrayUnion` with diagnostics: arrow-function. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualOverloadListFromArrayUnion has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to `issues/done/5368-isolate-exported-bindings-across-filename-sections.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows the generated arrow-function bucket is stale.
The current first build blocker is a multi-section external-module binding
collision before contextual overload inference can be reached.

Current focused coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
```

Resolved dump evidence:

```text
error: [DuplicateLocal] duplicate local binding: `yThen` at 215..256
```

Source context:

```ts
// @filename: one.ts
declare const y: never[] | string[];
export const yThen = y.map(item => item.length);
// @filename: two.ts
declare const y: number[][] | string[];
export const yThen = y.map(item => item.length);
```

The repeated `export const yThen` declarations belong to different
`@filename` external-module sections, but the compiler currently resolves them
in one shared binding scope.

This bucket was split to
`issues/done/5368-isolate-exported-bindings-across-filename-sections.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, diagnostic DuplicateLocal
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts
result: pass; current blocker is repeated exported binding across @filename sections, split to issue 5368
date: 2026-05-07
```

Remaining risks:

- The contextual overload list behavior and later arrow callback inference remain
  hidden until external-module `@filename` section bindings are isolated.
