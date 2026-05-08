---
id: 1061
title: "Implement Bindingpatterncontextualtypedoesnotcausewidening"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage bindingPatternContextualTypeDoesNotCauseWidening across 1 failing reference test cases and close this generated bucket because fresh evidence shows no current compiler blocker.

## Problem

Older reference results classified this path as a name-resolution failure. Fresh smart triage on 2026-05-06 reports `BuildPass` and the TypeScript oracle has no diagnostics.

Problem: bindingPatternContextualTypeDoesNotCauseWidening no longer has a current compiler blocker; no child implementation issue is needed for this generated bucket.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bindingPatternContextualTypeDoesNotCauseWidening.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bindingPatternContextualTypeDoesNotCauseWidening.ts --detail
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
- [x] This closed issue includes the reference path, diagnostic code, and source context
- [x] Completion evidence records the exact fixture/reference path and diagnostic result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bindingPatternContextualTypeDoesNotCauseWidening.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bindingPatternContextualTypeDoesNotCauseWidening.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/bindingPatternContextualTypeDoesNotCauseWidening.ts`

## Duplicate detection

- No child issue needed; fresh triage found no current compiler blocker.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/bindingPatternContextualTypeDoesNotCauseWidening.ts`
- Diagnostic: `BuildPass` / `pass`
- Failure: none; `ts2wasm build succeeded`
- Source context: `const {  } = pick(['b'], { a: 'a', b: 'b' });`
- Compiler evidence: tokens, AST, and resolved dumps succeed; resolved output contains `DestructureLet { pattern: Object([]), ... }`.
- TypeScript oracle: no diagnostics.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bindingPatternContextualTypeDoesNotCauseWidening.ts
result: pass; BuildPass with TypeScript oracle ok, no current compiler blocker
date: 2026-05-06
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

