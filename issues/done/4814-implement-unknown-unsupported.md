---
id: 4814
title: "Investigate and classify unknown-unsupported cases (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage unknown-unsupported feature across 5 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 5 cases fail with unknown-unsupported diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: unknown-unsupported feature has 5 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/classExpressionWithComputedPropertyInLoop.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/classExpressionWithComputedPropertyInLoop.ts --detail
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
mise run reference-coverage -- tsgo --limit 10
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/classExpressionWithComputedPropertyInLoop.ts --detail
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/classExpressionWithComputedPropertyInLoop.ts
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

- `reference/typescript-go/testdata/tests/cases/compiler/classExpressionWithComputedPropertyInLoop.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/jsSpeculativeParsingError.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/taggedTemplateNestedInvalidEscape.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/varianceComputationNoCrash.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/unicodeSurrogatesInStringLiterals.ts`

## Duplicate detection

- `issues/open/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key, title overlap)
- `issues/open/1026-implement-badArrayIndex.md` - Implement Badarrayindex (same feature label, same group key)
- `issues/open/1030-implement-badOverloadError.md` - Implement Badoverloaderror (same feature label, same group key)
- `issues/open/1043-implement-bestChoiceType.md` - Implement Bestchoicetype (same feature label, same group key)
- `issues/open/1045-implement-betterErrorForAccidentalCall.md` - Implement Bettererrorforaccidentalcall (same feature label, same group key)
- `issues/open/1058-implement-binderBinaryExpressionStress.md` - Implement Binderbinaryexpressionstress (same feature label, same group key)
- `issues/open/1059-implement-binderBinaryExpressionStressJs.md` - Implement Binderbinaryexpressionstressjs (same feature label, same group key)
- `issues/open/106-implement-accessors.md` - Implement Accessors (same feature label, same group key)
- `issues/open/1065-implement-bitwiseCompoundAssignmentOperators.md` - Implement Bitwisecompoundassignmentoperators (same feature label, same group key)
- `issues/open/1086-implement-builtinIterator.md` - Implement Builtiniterator (same feature label, same group key)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


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
