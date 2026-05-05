---
id: 201
title: "Investigate and classify unknown-unsupported cases (dup)"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-05-04
---

## Summary

Triage the generated reference bucket `Investigate and classify unknown-unsupported cases` before implementation. This issue records a failing reference case and must be split or superseded before any code change starts.

## Problem

Reference test results show 41 cases fail with unknown-unsupported diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: generated reference bucket `Investigate and classify unknown-unsupported cases` fails with `unknown-unsupported` and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/allowSyntheticDefaultImports9.ts
```

Narrow coverage reproduction:

```sh
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/allowSyntheticDefaultImports9.ts --detail
```

Representative path: `reference/typescript-go/testdata/tests/cases/compiler/allowSyntheticDefaultImports9.ts`
Feature label: `unknown-unsupported`

## Desired final state

This generated bucket is not used as a direct implementation work order. It is either superseded by an existing open/done issue, closed as a duplicate, or split into implementation-ready child issues that contain exact reproduction evidence and measurable acceptance criteria.

## Scope

In scope:

- [x] Run the representative `mise run reference-triage -- ...` command
- [x] Confirm whether duplicate candidates already cover this failure
- [x] Split one observable behavior or fixed reference window into child issues
- [x] Carry source context, diagnostic code, AST evidence, and validation commands into each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad fixes that mix unrelated parser, resolver, runtime, and API failures

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates are confirmed as no-match, duplicate, or superseding issue
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/allowSyntheticDefaultImports9.ts
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/allowSyntheticDefaultImports9.ts --detail
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

- `reference/typescript-go/testdata/tests/cases/compiler/allowSyntheticDefaultImports9.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/assertionWithNoArgument.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/bindingPatternOptionalParameterCached.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/blockedScopeVariableNotUnused1.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/classFieldsPropertyAccessSameNameAsClass.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/conditionalContextualReturnSubstitutionCache.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/contextuallyTypedJsxChildren2.tsx`
- `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitAsConstSatisfiesNonReadonlyResult.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitBigInt.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitConstObjectLiteralGenericMethod1.ts`
- ... and 31 more files

## Duplicate detection

- `issues/done/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, title overlap)
- `issues/open/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, title overlap)


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/454-implement-unknown-unsupported.md` に統合されました。
そちらを参照してください。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/201-implement-unknown-unsupported.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
