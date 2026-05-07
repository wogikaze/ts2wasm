---
id: 1376
title: "Implement Commentsonrequirestatement"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: [5285]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1376.

## Summary

Closed as superseded by
`issues/open/5285-support-export-var-initializer-declarations.md`.

Fresh triage shows the first blocker is the same `issue-055` initialized
`export var` declaration boundary, before the later re-export statements or
module-resolution diagnostics are reached.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsOnRequireStatement` with diagnostics: import-export. Fresh focused
triage on 2026-05-07 shows tokenization succeeds, but AST construction stops at
the first `export var subject = 10;`.

Problem: `commentsOnRequireStatement.ts` currently cannot represent initialized
entry-module `export var` declarations.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnRequireStatement.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnRequireStatement.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Smart triage reports:

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 68..74
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5285-support-export-var-initializer-declarations.md`; later
re-export or missing-module diagnostics should be recorded after that blocker
advances.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing initialized export-var issue
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
- [x] Superseding issue contains the exact initialized export-var diagnostic family
- [x] This issue includes failing path, diagnostic code, source context, token/TypeScript AST evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnRequireStatement.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnRequireStatement.ts
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

- [x] superseded by: `issues/open/5285-support-export-var-initializer-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsOnRequireStatement.ts`

## Duplicate detection

- `issues/open/5285-support-export-var-initializer-declarations.md` owns this
  initialized `export var name = expr;` boundary.
- `issues/done/5283-support-entry-export-var-declarations.md` covers typed
  export-var declarations without initializers and is too narrow for this
  first blocker.
- `issues/done/232-resolve-local-relative-es-module-graph.md` may cover later
  missing `./0` / `./1` module diagnostics after export-var parsing advances,
  but it is not the first current blocker.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage import export: commentsOnRequireStatement

- Issue class: triage-needed
- Feature label: import-export
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/commentsOnRequireStatement.ts
```

Source context:

```ts
// @Filename: 0.ts
export var subject = 10;

// @Filename: 1.ts
export var subject1 = 10;

// @Filename: 2.ts
export {subject} from './0';
export {subject1} from './1';
```

Compiler evidence:

```text
tokens: ok through export var declarations and export-from declarations
ast/resolved: fails at first export var with issue-055 unsupported variable export
```

TypeScript oracle:

```text
TS2323 Cannot redeclare exported variable 'subject'
TS2323 Cannot redeclare exported variable 'subject1'
TS2307 Cannot find module './0'
TS2307 Cannot find module './1'
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnRequireStatement.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnRequireStatement.ts
result: issue-055 unsupported variable export; superseded by issue 5285
date: 2026-05-07
```

Remaining risks:

- After issue 5285 advances this path, the re-export declarations may expose
  missing virtual module or duplicate export diagnostics.
