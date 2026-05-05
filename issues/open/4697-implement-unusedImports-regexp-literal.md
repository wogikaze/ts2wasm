---
id: 4697
title: "Implement Unusedimports Regexp Literal"
type: spike
area: reference/triage
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage unusedImports-regexp-literal across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail in directory `unusedImports-regexp-literal` with diagnostics: regexp-literal. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: unusedImports-regexp-literal has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/unusedImports14.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/unusedImports14.ts --detail
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

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- parser/resolver code unless `reference-triage` proves the failure happens before runtime lowering

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
mise run reference-coverage -- tsc --limit 8
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/unusedImports14.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/unusedImports14.ts
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

- `reference/typescript/tests/cases/compiler/unusedImports14.ts`
- `reference/typescript/tests/cases/compiler/unusedImports13.ts`
- `reference/typescript/tests/cases/compiler/unusedImports15.ts`
- `reference/typescript/tests/cases/compiler/unusedImports16.ts`

## Duplicate detection

- `issues/done/066-implement-regexp-literal.md` - Implement RegExp literal support (same feature label, title overlap)
- `issues/open/444-implement-regexp-literal.md` - Implement RegExp literal support (same feature label, title overlap)
- `issues/done/051-implement-regexp.md` - Implement RegExp (same feature label, title overlap)

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
