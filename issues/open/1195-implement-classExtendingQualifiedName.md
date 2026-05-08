---
id: 1195
title: "Implement Classextendingqualifiedname"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1195.

## Summary

Triage classExtendingQualifiedName across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `classExtendingQualifiedName` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExtendingQualifiedName has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendingQualifiedName2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingQualifiedName2.ts --detail
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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingQualifiedName2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendingQualifiedName2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5313-report-non-exported-namespace-member-qualified-heritage.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendingQualifiedName2.ts`
- `reference/typescript/tests/cases/compiler/classExtendingQualifiedName.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-07.

Fresh focused coverage shows this generated blocker bucket is stale:

```text
executed=2
build_pass=2
unsupported=0
reference/typescript/tests/cases/compiler/classExtendingQualifiedName.ts: build_pass
reference/typescript/tests/cases/compiler/classExtendingQualifiedName2.ts: build_pass
```

Representative triage:

```text
classExtendingQualifiedName2.ts:
ts2wasm: BuildPass
TypeScript oracle: ok, no diagnostics
source: namespace M { export class C {} class D extends M.C {} }

classExtendingQualifiedName.ts:
ts2wasm: BuildPass
TypeScript oracle: TS2339 Property 'C' does not exist on type 'typeof M'.
source: namespace M { class C {} class D extends M.C {} }
```

Compiler evidence:

```text
tokens: ok for namespace, class C, class D, Extends, Ident M, Dot, Ident C
ast: ok
resolved: ok
```

Split child: `issues/open/5313-report-non-exported-namespace-member-qualified-heritage.md`.

Related issues are no-match for this exact residual semantic diagnostic:

- Issue 5225 handles a current unsupported qualified heritage implementation blocker.
- Issue 5287 handles same-file namespace value access such as `m1.fooExport()`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingQualifiedName.ts
result: pass; current compiler build-passes, TypeScript oracle reports TS2339 for non-exported namespace member, split to issue 5313
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingQualifiedName2.ts
result: pass; compiler and TypeScript both accept exported namespace member qualified heritage
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingQualifiedName --detail --no-dashboard-data
result: pass; executed=2, build_pass=2, unsupported=0
date: 2026-05-07
```

Remaining risks:

- Semantic parity for the non-exported namespace member is tracked by issue 5313.
