---
id: 788
title: "Implement Transportstream (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Zero implementation commits. Batch-closed without evidence. Batch audit `3f0bfdf18` stamped as truly-done without individual verification.
> Evidence: `git log --oneline --all --grep=788` shows only creation/chore commits — no feat/fix commit.

## Summary

Triage TransportStream across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `TransportStream` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: TransportStream has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/TransportStream.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/TransportStream.ts --detail
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


## Triage result

Failing test: `TransportStream.ts` — duplicate

This issue was reopened by false-done audit. It is a TypeScript compiler reference test case classified as superseded by meta-issue dependencies.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/TransportStream.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/TransportStream.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/TransportStream.ts`

## Duplicate detection

- `issues/done/081-implement-TransportStream.md` - Implement Transportstream (same reference path, same feature label, same group key, title overlap)
- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, same group key, title overlap)
- `issues/done/467-implement-TransportStream.md` - Implement Transportstream (same reference path, same feature label, same group key, title overlap)
- `issues/open/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, same group key, title overlap)
- `issues/open/553-implement-TransportStream.md` - Implement Transportstream (same reference path, same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/553-implement-TransportStream.md` に統合されました。
そちらを参照してください。
## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none

## ⚠️ False-done audit (re-opened from `issues/done/`)

**Why this was false-done**: This generated triage spike issue was moved to `issues/done/` without actual triage or implementation. It has `type: spike` and `class: blocked` depending on meta-issues (`depends_on: [5000]` or `[5001]`), with zero implementation commits referencing this issue, empty completion evidence (commits `...`), and no Status note or Close note documenting triage results.

**True-done checklist** (all must pass):
1. Perform actual triage review of the reference failure case(s)
2. Either split into implementation-ready child issue(s) or confirm superseded by an existing issue (document with Status note)
3. Fill in completion evidence section with triage results and commit SHAs
4. Verify all acceptance criteria checkboxes reflect completed work

**Commands that must pass**:
```sh
cargo fmt --all --check
cargo nextest run
```

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

