---
id: 1019
title: "Implement Awaitinnonasyncfunction"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage awaitInNonAsyncFunction across 1 failing reference test case and split this generated bucket into a smaller implementation-ready child issue.

## Problem

Reference test results show 1 case fails in directory `awaitInNonAsyncFunction` with diagnostics: runtime-subset. Fresh triage shows the first current blocker is the context-specific `for await...of` diagnostic in a non-async function.

Problem: awaitInNonAsyncFunction has 1 reference failure that is now tracked by child issue 5146.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/done/5146-report-for-await-context-errors-before-async-runtime.md`.

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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue 5146 contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5146-report-for-await-context-errors-before-async-runtime.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage runtime subset: awaitInNonAsyncFunction

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts
```

Failure location:

```json
{
  "code": "UnsupportedRuntimeSubset",
  "message": "issue-230: `for await...of` async iteration requires Promise and async iterator runtime semantics, which are not supported in this milestone at 138..147",
  "span_start": 138,
  "span_end": 147,
  "line": 6,
  "column": 3,
  "feature_label": "runtime-subset",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
3 | // https://github.com/Microsoft/TypeScript/issues/26586
4 |
5 | function normalFunc(p: Promise<number>) {
6 |   for await (const _ of []);
7 |   return await p;
8 | }
```

TypeScript oracle:

```text
TS1103: 'for await' loops are only allowed within async functions and at the top levels of modules.
TS1308: 'await' expressions are only allowed within async functions and at the top levels of modules.
```

## Completion evidence

Commits:

- pending local commit

Validation result:

```text
command:
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/awaitInNonAsyncFunction.ts
result:
pass; emitted UnsupportedRuntimeSubset report for `for await` in a non-async function; split to issue 5146
date:
2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

