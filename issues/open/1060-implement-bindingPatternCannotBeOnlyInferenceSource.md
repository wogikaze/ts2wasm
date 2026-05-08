---
id: 1060
title: "Implement Bindingpatterncannotbeonlyinferencesource"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: [5174]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage bindingPatternCannotBeOnlyInferenceSource across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `bindingPatternCannotBeOnlyInferenceSource` with diagnostics: duplicate-local. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: bindingPatternCannotBeOnlyInferenceSource has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bindingPatternCannotBeOnlyInferenceSource.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bindingPatternCannotBeOnlyInferenceSource.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bindingPatternCannotBeOnlyInferenceSource.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bindingPatternCannotBeOnlyInferenceSource.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5174-ignore-empty-binding-pattern-synthetic-names.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/bindingPatternCannotBeOnlyInferenceSource.ts`

## Duplicate detection

- No open issue was found for the repeated empty binding-pattern synthetic-name duplicate-local failure.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/bindingPatternCannotBeOnlyInferenceSource.ts`
- Diagnostic: `DuplicateLocal` / `compiler-diagnostic`
- Failure: `duplicate local binding: {}` at `937..1148`
- Source context: repeated `const {} = ...` destructuring declarations
- Visible symbols before failure: `funcs1`
- Compiler evidence: tokens and AST succeed; AST stores empty patterns as `Let { name: "{}" }` and `Let { name: "[]" }`; validation treats repeated `{}` as a real duplicate local.
- TypeScript oracle: reports type/inference diagnostics on unknown destructuring, not duplicate local declarations.
- Superseding child: `issues/open/5174-ignore-empty-binding-pattern-synthetic-names.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bindingPatternCannotBeOnlyInferenceSource.ts
result: pass; current blocker identified as false duplicate-local for empty binding patterns, split to issue 5174
date: 2026-05-06
```

Remaining risks:

- The intended TypeScript type-inference diagnostics need follow-up triage after issue 5174 removes the false duplicate-local blocker.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

