---
id: 1358
title: "Implement Commentsatendoffile"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: [5282]
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1358.

## Summary

Closed after splitting the current labeled empty statement parser blocker into
`issues/open/5282-parse-labeled-empty-statements.md`.

## Problem

Reference test results show 1 case failing in directory
`commentsAtEndOfFile` with an unknown-unsupported diagnostic. Fresh triage
shows the current blocker is not EOF comment handling; it is the labeled empty
statement `Input: ;`.

Problem: `commentsAtEndOfFile1.ts` currently reports
`UnsupportedSyntax: unsupported expression: ... Semicolon` while parsing the
body of the `Input:` label.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsAtEndOfFile1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsAtEndOfFile1.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 59, end: 60 } })
unsupported_features=unknown-unsupported:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5282-parse-labeled-empty-statements.md`.

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
- [x] Child issue includes failing path, diagnostic code, source context, tokens/AST evidence, and TypeScript oracle evidence
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsAtEndOfFile1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsAtEndOfFile1.ts
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

- [x] `issues/open/5282-parse-labeled-empty-statements.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsAtEndOfFile1.ts`

## Duplicate detection

- `issues/done/290-fix-asi-eof-semicolon-parser-bucket.md` is related ASI
  history but covered `expected Semicolon, got None`, not labeled `;`.
- `issues/done/5211-sparse-array-spread-support.md` is about
  the boundary before a following label, not a label whose body is `;`.
- Broad unknown-unsupported buckets are not exact matches.
- `issues/open/5282-parse-labeled-empty-statements.md` owns this current
  blocker.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage unknown unsupported: commentsAtEndOfFile1

- Issue class: triage-needed
- Feature label: unknown-unsupported
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commentsAtEndOfFile1.ts
```

Failure:

```text
unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 59, end: 60 } })
```

Source context:

```ts
Input:
;
//Testing two
```

Compiler evidence:

```text
tokens: ok; Ident("Input"), Colon, Semicolon
ast: fails while parsing the labeled statement body
resolved: same parser failure
```

TypeScript oracle:

```text
ok: true
diagnostics: []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsAtEndOfFile1.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsAtEndOfFile1.ts
result: UnsupportedSyntax for labeled empty statement; split to issue 5282
date: 2026-05-06
```

Remaining risks:

- EOF comment emit fidelity is not proven until the labeled empty statement
  parser blocker advances.
