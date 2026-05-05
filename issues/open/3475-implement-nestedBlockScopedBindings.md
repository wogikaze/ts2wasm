---
id: 3475
title: "Implement Nestedblockscopedbindings"
type: spike
area: frontend/resolver
class: blocked
priority: P2
depends_on: [5006]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage nestedBlockScopedBindings across 9 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 9 cases fail in directory `nestedBlockScopedBindings` with diagnostics: scope-analysis. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: nestedBlockScopedBindings has 9 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 18
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings1.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings10.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings12.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings16.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings3.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings15.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings2.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings9.ts`

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
