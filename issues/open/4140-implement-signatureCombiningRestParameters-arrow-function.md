---
id: 4140
title: "Implement Signaturecombiningrestparameters Arrow Function"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage signatureCombiningRestParameters-arrow-function across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `signatureCombiningRestParameters-arrow-function` with diagnostics: arrow-function. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: signatureCombiningRestParameters-arrow-function has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/signatureCombiningRestParameters2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/signatureCombiningRestParameters2.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/signatureCombiningRestParameters2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/signatureCombiningRestParameters2.ts
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

- `reference/typescript/tests/cases/compiler/signatureCombiningRestParameters2.ts`
- `reference/typescript/tests/cases/compiler/signatureCombiningRestParameters5.ts`

## Duplicate detection

- `issues/done/415-implement-arrow-function.md` - Implement arrow functions (same feature label, title overlap)
- `issues/done/430-implement-function.md` - Implement function support (same feature label, title overlap)
- `issues/done/456-implement-APISample-arrow-function.md` - Implement Apisample Arrow Function (same feature label, title overlap)
- `issues/open/542-implement-APISample-arrow-function.md` - Implement Apisample Arrow Function (same feature label, title overlap)
- `issues/done/036-implement-arrow-function.md` - Implement arrow function (same feature label, title overlap)
- `issues/done/062d-function-this-and-arguments.md` - Implement function this and arguments semantics (same feature label, title overlap)
- `issues/done/210-implement-arrow-function-closure-lexical-this.md` - Implement arrow function closure and lexical this semantics (same feature label, title overlap)

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
