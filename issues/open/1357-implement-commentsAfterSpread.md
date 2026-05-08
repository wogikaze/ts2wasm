---
id: 1357
title: "Implement Commentsafterspread"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: [5281]
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1357.

## Summary

Closed after splitting the current resolver blocker into
`issues/open/5281-resolve-commented-arrow-rest-parameters.md`.

## Problem

Reference test results previously showed 1 case failing in directory
`commentsAfterSpread` with parser-syntax diagnostics. Fresh triage shows
tokens and AST now succeed; the current blocker is name resolution for an arrow
rest parameter with comments between `...` and the parameter name.

Problem: `commentsAfterSpread.ts` currently reports
`UnresolvedName: unresolved name: \`args\`` for
`(.../* comment h */args) => args.length`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsAfterSpread.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsAfterSpread.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
UnresolvedName: unresolved name: `args` at 725..729
unsupported_features=name-resolution:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5281-resolve-commented-arrow-rest-parameters.md`.

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
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsAfterSpread.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsAfterSpread.ts
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

- [x] `issues/open/5281-resolve-commented-arrow-rest-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsAfterSpread.ts`

## Duplicate detection

- `issues/open/1300-implement-collisionRestParameterArrowFunctions.md` is
  related but is a generated collision bucket without matching current
  `UnresolvedName: args` evidence.
- `issues/done/038-implement-rest-parameters.md` and
  `issues/open/212-implement-rest-parameter-argument-collection.md` are related
  rest-parameter history, but this reference still fails in arrow rest
  parameter name resolution.
- `issues/open/5064-implement-arrow-function.md` is the broad arrow-function
  generated bucket and was superseded by narrower children.
- `issues/open/5281-resolve-commented-arrow-rest-parameters.md` owns this
  current blocker.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage name resolution: commentsAfterSpread

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/commentsAfterSpread.ts
```

Failure location:

```text
unresolved name: `args` at 725..729
line 59, column 30
```

Source context:

```ts
const h = (.../* comment h */args) => args.length;

const i = (
  first, .../* comment i */rest
) => rest.length;
```

Compiler evidence:

```text
tokens: ok; includes spread/rest tokens around comments
ast: ok; object spreads and arrow functions parse
resolved: fails in resolve_names with UnresolvedName for `args`
```

TypeScript oracle:

```text
ok: true
diagnostics: []
parameter args: any[]
binding h: (...args: any[]) => number
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsAfterSpread.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsAfterSpread.ts
result: UnresolvedName for arrow rest parameter `args`; split to issue 5281
date: 2026-05-06
```

Remaining risks:

- Later object rest/spread runtime behavior in the same reference file is not
  proven until the arrow rest parameter resolver blocker is fixed.
