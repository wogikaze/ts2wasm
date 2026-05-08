---
id: 1025
title: "Implement Awaitedtypestrictnull"
type: spike
area: runtime/builtins
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

> **Reopened by audit** (2026-05-06)
> Classification: false-done (triage-needed)
> Reason: triage-needed bucket with all acceptance criteria unchecked, empty completion evidence, zero implementation commits. No triage work was performed before moving to done/.
>
> True-done checklist:
> 1. Inspect the smart triage report and confirm/update duplicates
> 2. Split into implementation-ready child issue(s) or supersede by existing issue
> 3. Fill completion evidence with commits and validation results
>
> Rechecked on 2026-05-06 with `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/awaitedTypeStrictNull.ts`; the build still passes, so this stale generated bucket remains closed.

## Summary

Triage awaitedTypeStrictNull across 1 generated reference bucket entry and close it if current evidence shows no implementation blocker.

## Problem

Older reference test results showed 1 case failing in directory `awaitedTypeStrictNull` with diagnostics: parser-syntax. Fresh smart triage on 2026-05-06 shows the case now builds successfully, so this generated bucket is stale.

Problem: awaitedTypeStrictNull no longer has a current compiler blocker; no child implementation issue is needed for this generated bucket.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeStrictNull.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitedTypeStrictNull.ts --detail
```

## Desired final state

This generated bucket is closed as stale because the only affected reference case currently reports `BuildPass` / `pass`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] No child issue created because fresh triage found no current compiler blocker
- [x] Preserve exact reproduction commands and representative diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `mise run reference-triage -- ...` command
- [x] This closed issue includes the reference path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence records the exact fixture/reference path and diagnostic result

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitedTypeStrictNull.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeStrictNull.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/awaitedTypeStrictNull.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Build pass: awaitedTypeStrictNull

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/awaitedTypeStrictNull.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeStrictNull.ts
```

Failure location:

```json
{
  "code": "BuildPass",
  "message": "ts2wasm build succeeded",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "build-pass",
  "error_type": "pass"
}
```

Source context:

```text
// @target: esnext
// @strictNullChecks: true

type T1 = Awaited<number>;
type T2 = Awaited<Promise<number>>;
type T3 = Awaited<number | Promise<number>>;
```

Compiler evidence:

```text
AST and resolved dumps are empty because the current supported build erases these type-only declarations.
TypeScript oracle reports type-system diagnostics, but there is no current compiler build blocker on this path.
```

## Completion evidence

Commits:

- pending local commit

Validation result:

```text
command:
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/awaitedTypeStrictNull.ts
result:
pass; emitted BuildPass / pass smart-triage report for the only affected reference path
date:
2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

