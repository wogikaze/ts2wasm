---
id: 2410
title: "Implement For"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage for across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `for` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: for has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/for.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/for.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/for.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/for.ts
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

- `reference/typescript/tests/cases/compiler/for.ts`

## Duplicate detection

- `issues/done/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/done/106-implement-accessors.md` - Implement Accessors (same feature label, same group key, title overlap)
- `issues/done/193-implement-arguments.md` - Implement Arguments (same feature label, same group key, title overlap)
- `issues/done/201-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/done/225-implement-eval-annexb-function-declarations.md` - Implement eval and Annex B function declaration semantics (same feature label, same group key, title overlap)
- `issues/done/313-implement-array-builtin.md` - Implement array-builtin support (same feature label, same group key, title overlap)
- `issues/done/314-implement-string-builtin.md` - Implement string-builtin support (same feature label, same group key, title overlap)
- `issues/open/411-implement-annexb-ishtmldda.md` - Implement annexb-ishtmldda support (same feature label, same group key, title overlap)
- `issues/done/412-implement-arguments-object.md` - Implement arguments-object support (same feature label, same group key, title overlap)
- `issues/done/414-implement-array-builtin.md` - Implement array-builtin support (same feature label, same group key, title overlap)

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
