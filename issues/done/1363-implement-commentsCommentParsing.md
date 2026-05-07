---
id: 1363
title: "Implement Commentscommentparsing"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: [5280]
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed as superseded by `issues/open/5280-validate-commented-top-level-function-overloads.md`.

## Problem

Reference test results show 1 case failing in directory
`commentsCommentParsing` with diagnostics: parser-syntax. Fresh triage shows
tokens and AST now succeed; validation stops at a top-level function overload
group with comments.

Problem: `commentsCommentParsing.ts` currently reports `DuplicateFunction` for
the commented `f1` overload signatures plus implementation group.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsCommentParsing.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsCommentParsing.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
DuplicateFunction: duplicate function definition: `f1` at 2455..2463
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5280-validate-commented-top-level-function-overloads.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing commented overload issue
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
- [x] Superseding issue contains matching commented overload evidence
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, AST evidence, and TypeScript oracle evidence
- [x] Superseding issue acceptance names the diagnostic/stdout change for commented overload groups

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsCommentParsing.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsCommentParsing.ts
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

- [x] superseded by `issues/open/5280-validate-commented-top-level-function-overloads.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsCommentParsing.ts`

## Duplicate detection

- `issues/open/5280-validate-commented-top-level-function-overloads.md` owns
  commented bodyless top-level overload signatures followed by one
  implementation. `commentsCommentParsing.ts` fails in the same feature family
  at the commented `f1` overload group.
- `issues/done/5200-validate-top-level-function-overload-implementations.md`
  owns general top-level overload grouping without the comment/trivia-specific
  focus.
- `issues/done/5199-report-function-overload-list-class-merge-diagnostics.md`
  covers function overload lists followed by class merge diagnostics and is not
  this blocker.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage duplicate function: commentsCommentParsing

- Issue class: triage-needed
- Feature label: duplicate-function
- Diagnostic: DuplicateFunction / compiler-diagnostic
- Path: reference/typescript/tests/cases/compiler/commentsCommentParsing.ts
```

Failure:

```text
duplicate function definition: `f1` at 2455..2463
```

Source context:

```ts
/** fn f1 with number
* @param { string} b about b
*/
function f1(a: number);
function f1(b: string);
/**@param opt optional parameter*/
function f1(aOrb, opt?) {
    return aOrb;
}
```

Compiler evidence:

```text
tokens: ok
ast: ok, includes Function f1(a: number), Function f1(b: string), Function f1(aOrb, opt?) with body
validate_ast: DuplicateFunction for the commented overload group
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
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsCommentParsing.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=DuplicateFunction:1, unsupported_features=duplicate-function:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsCommentParsing.ts
result: parser/AST ok; validate_ast fails with DuplicateFunction for commented f1 overload group; superseded by issue 5280
date: 2026-05-06
```

Remaining risks:

- Later comment parsing behavior is not independently validated until the
  commented overload boundary advances through issue 5280.
