---
id: 1360
title: "Implement Commentsbeforevariablestatement"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: [5283]
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1360.

## Summary

Closed after splitting the current `export var` module-syntax blocker into
`issues/done/5283-support-entry-export-var-declarations.md`.

## Problem

Reference test results show 1 case failing in directory
`commentsBeforeVariableStatement` with an import/export diagnostic. Fresh triage
shows the current blocker is not comment syntax; it is the issue-055 variable
export boundary for `export var b: number;`.

Problem: `commentsBeforeVariableStatement1.ts` currently reports
`UnsupportedModule: issue-055: unsupported variable export`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
UnsupportedModule: issue-055: unsupported variable export; module resolution and loading are not implemented at 83..89
unsupported_features=unknown-unsupported:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/done/5283-support-entry-export-var-declarations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
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
- [x] Child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, token evidence, and TypeScript oracle evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5283-support-entry-export-var-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts`

## Duplicate detection

- `issues/done/5175-support-export-let-destructuring-declarations.md` is
  related but covers `export let` destructuring and excludes general
  `export var`.
- `issues/done/5144-support-entry-export-function-declarations.md`,
  `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`, and
  `issues/open/5277-parse-export-enum-declarations-to-enum-boundary.md` cover
  sibling export forms.
- `issues/open/432-implement-import-export.md` is the broad import/export
  generated bucket and is too wide.
- `issues/done/5283-support-entry-export-var-declarations.md` owns this current
  blocker.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage import export: commentsBeforeVariableStatement1

- Issue class: triage-needed
- Feature label: import-export
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts
```

Failure:

```text
issue-055: unsupported variable export; module resolution and loading are not implemented at 83..89
```

Source context:

```ts
/** b's comment*/
export var b: number;
```

Compiler evidence:

```text
tokens: ok; Export, Var, Ident("b"), Colon, Ident("number"), Semicolon
ast: fails at issue-055 variable export boundary
resolved: same parser/module failure
```

TypeScript oracle:

```text
ok: true
diagnostics: []
binding b: number
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsBeforeVariableStatement1.ts
result: UnsupportedModule issue-055 for `export var`; split to issue 5283
date: 2026-05-06
```

Remaining risks:

- Comment emit fidelity is not proven until the `export var` boundary advances.
