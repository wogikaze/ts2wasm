---
id: 173
title: "Implement Amdlikeinputdeclarationemit"
type: spike
area: reference
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-29
---

## Summary

Triage the generated reference bucket `Implement Amdlikeinputdeclarationemit` before implementation. This issue records a failing reference case and must be split or superseded before any code change starts.

## Problem

Reference test results show 1 cases fail in directory `amdLikeInputDeclarationEmit` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: generated reference bucket `Implement Amdlikeinputdeclarationemit` fails with `parser-syntax` and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts
```

Narrow coverage reproduction:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts --detail
```

Representative path: `reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts`
Feature label: `parser-syntax`

## Desired final state

This generated bucket is not used as a direct implementation work order. It is either superseded by an existing open/done issue, closed as a duplicate, or split into implementation-ready child issues that contain exact reproduction evidence and measurable acceptance criteria.

## Scope

In scope:

- [ ] Run the representative `mise run reference-triage -- ...` command
- [ ] Confirm whether duplicate candidates already cover this failure
- [ ] Split one observable behavior or fixed reference window into child issues
- [ ] Carry source context, diagnostic code, AST evidence, and validation commands into each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad fixes that mix unrelated parser, resolver, runtime, and API failures

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- unrelated runtime/backend files unless `reference-triage` proves the failure is not parser/frontend

## Acceptance criteria

- [ ] Duplicate candidates are confirmed as no-match, duplicate, or superseding issue
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts --detail
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

- `reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts`

## Duplicate detection

- none found by path/title/feature scan

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
