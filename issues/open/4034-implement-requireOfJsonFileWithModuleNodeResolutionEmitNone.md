---
id: 4034
title: "Implement Requireofjsonfilewithmodulenoderesolutionemitnone"
type: spike
area: frontend/syntax
class: blocked
priority: P2
depends_on: [5007]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage requireOfJsonFileWithModuleNodeResolutionEmitNone across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `requireOfJsonFileWithModuleNodeResolutionEmitNone` with diagnostics: module-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: requireOfJsonFileWithModuleNodeResolutionEmitNone has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/requireOfJsonFileWithModuleNodeResolutionEmitNone.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/requireOfJsonFileWithModuleNodeResolutionEmitNone.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/requireOfJsonFileWithModuleNodeResolutionEmitNone.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/requireOfJsonFileWithModuleNodeResolutionEmitNone.ts
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

- `reference/typescript/tests/cases/compiler/requireOfJsonFileWithModuleNodeResolutionEmitNone.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.

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


---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This is a generated triage bucket issue. It was
created as a `class: blocked` spike with `depends_on` pointing to a parent
meta-issue (5004 or 5007). When the parent meta-issue was moved to
`issues/done/`, this child issue was dragged along without any implementation
or triage work. The `## Completion evidence` section is unfilled (commits
placeholder `...`, validation result empty). Zero implementation commits
reference this issue.

**True-done checklist** (all must pass):

1. **Triage the representative failure path**: Confirm it is superseded by an
   existing open/done issue OR split into implementation-ready child issues
   with exact reproduction commands.

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - Issue URL or child issue path documenting the triage outcome
   - Or: the exact failing reference path has a matching open/done issue
   - Or: the failing test case no longer reproduces the original diagnostic

