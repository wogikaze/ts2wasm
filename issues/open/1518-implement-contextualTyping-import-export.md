---
id: 1518
title: "Implement Contextualtyping Import Export"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1518.

## Summary

Triage contextualTyping-import-export across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTyping-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTyping-import-export has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTyping.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTyping.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTyping.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTyping.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5378-report-mixed-ambient-function-overload-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTyping.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/open/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/open/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

Date: 2026-05-07

Command:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTyping.ts
```

Result: split to
`issues/open/5378-report-mixed-ambient-function-overload-diagnostics.md`.

Current diagnostic:

```text
DuplicateLocal: duplicate local variable: `EF1` at 5033..5064
feature_label: duplicate-local
```

Source context:

```ts
declare function EF1(a:number, b:number):number;

function EF1(a,b) { return a+b; }

var efv = EF1(1,2);
```

Compiler evidence:

- tokens: ok
- ast: ok through contextual typing cases, including `EF1` declarations
- visible symbols include bodyless ambient function `EF1`, concrete function
  `EF1`, and binding `efv`
- resolved/name resolution: fails with generic `DuplicateLocal` before the
  TypeScript mixed ambient/non-ambient overload diagnostic is reached
- TypeScript oracle reports TS2384, `Overload signatures must all be ambient or
  non-ambient.`, at the ambient `EF1` signature

Duplicate review:

- `issues/open/5200-validate-top-level-function-overload-implementations.md`
  owns valid non-ambient overload signatures plus implementations, not mixed
  ambient/non-ambient diagnostics.
- `issues/done/5226-w0-ast-node-span-requirement.md` owns
  multiple ambient overload declarations, not a mixed ambient signature plus
  non-ambient implementation.
- `issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`
  owns var/function duplicate identifier diagnostics, not overload ambientness.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5378-report-mixed-ambient-function-overload-diagnostics.md`

Validation result:

```text
command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTyping.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, current failure is DuplicateLocal duplicate-local
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTyping.ts
result: pass; reproduced generic DuplicateLocal for mixed ambient/non-ambient EF1 overload and split to issue 5378
date: 2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5378 reports the
  source-spanned mixed ambient/non-ambient overload diagnostic.
