---
id: 1362
title: "Implement Commentsclassmembers"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: [5192]
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1362.

## Summary

Closed as superseded by `issues/done/5192-support-first-class-class-constructor-values.md`.

## Problem

Reference test results show 1 case failing in directory
`commentsClassMembers` with diagnostics: parser-syntax. Fresh triage shows
parser and AST construction now succeed; the current blocker is the shared class
runtime value boundary.

Problem: `commentsClassMembers.ts` currently reports `issue-5011` when `c1` is
used as an expression value in `var i1_c = c1;`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsClassMembers.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsClassMembers.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
UnsupportedSyntax: issue-5011: class `c1` cannot be used as a value — class runtime is not yet supported at 5071..5073
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/done/5192-support-first-class-class-constructor-values.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing class runtime value issue
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
- [x] Superseding issue contains matching `issue-5011` class-value evidence
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, AST evidence, and TypeScript oracle evidence
- [x] Superseding issue acceptance names the diagnostic change for class constructor values

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsClassMembers.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsClassMembers.ts
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

- [x] superseded by `issues/done/5192-support-first-class-class-constructor-values.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsClassMembers.ts`

## Duplicate detection

- `issues/done/5192-support-first-class-class-constructor-values.md` owns the
  shared class constructor value boundary. `commentsClassMembers.ts` fails at
  `var i1_c = c1;`, which is the same `issue-5011` family.
- `issues/done/5011-class-runtime-value-semantics.md` documents the current
  structural diagnostic that prevents silent class value erasure.
- `issues/open/421-implement-class.md` is the broad class syntax issue and is
  too wide for this current blocker.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage class: commentsClassMembers

- Issue class: triage-needed
- Feature label: class
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commentsClassMembers.ts
```

Failure:

```text
issue-5011: class `c1` cannot be used as a value — class runtime is not yet supported at 5071..5073
```

Source context:

```ts
var i1_s_ncprop = c1.nc_s3;

var i1_c = c1;

class cProperties {
```

Compiler evidence:

```text
tokens: ok
ast: ok, includes ClassDecl c1 and later Let i1_c = Ident c1
resolved: issue-5011 at identifier c1 in `var i1_c = c1;`
```

TypeScript oracle:

```text
ok: false only because strict property initialization diagnostics are reported
diagnostics: TS2564 property initialization diagnostics
binding / visible symbols include i1_c initializer `c1`
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsClassMembers.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsClassMembers.ts
result: parser/AST ok; resolved fails with issue-5011 class value use; superseded by issue 5192
date: 2026-05-06
```

Remaining risks:

- Comment handling is not independently validated until the class value boundary
  advances through issue 5192.
