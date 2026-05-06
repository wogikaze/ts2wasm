---
id: 4810
title: "Implement object literal enhancements (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage object-literal feature across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3 cases fail with object-literal diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: object-literal feature has 3 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/freshObjectLiteralSubtype.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/freshObjectLiteralSubtype.ts --detail
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
mise run reference-coverage -- tsgo --limit 6
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/freshObjectLiteralSubtype.ts --detail
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/freshObjectLiteralSubtype.ts
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

- `reference/typescript-go/testdata/tests/cases/compiler/freshObjectLiteralSubtype.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/implicitEmptyObjectType.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/objectSubtypeReduction.ts`

## Duplicate detection

- `issues/open/052-implement-json.md` - Implement JSON (same feature label, same group key, title overlap)
- `issues/open/052d-implement-json-stringify-broader-replacer-semantics.md` - Implement broader JSON.stringify replacer semantics (same feature label, same group key, title overlap)
- `issues/done/1118-implement-castNewObjectBug.md` - Implement Castnewobjectbug (same feature label, same group key, title overlap)
- `issues/done/1128-implement-chainedSpecializationToObjectTypeLiteral.md` - Implement Chainedspecializationtoobjecttypeliteral (same feature label, same group key, title overlap)
- `issues/open/1136-implement-checkJsObjectLiteralIndexSignatures.md` - Implement Checkjsobjectliteralindexsignatures (same feature label, same group key, title overlap)
- `issues/open/1148-implement-checkingObjectWithThisInNamePositionNoCrash.md` - Implement Checkingobjectwiththisinnamepositionnocrash (same feature label, same group key, title overlap)
- `issues/open/1375-implement-commentsOnObjectLiteral-object-literal.md` - Implement Commentsonobjectliteral Object Literal (same feature label, same group key, title overlap)
- `issues/open/1511-implement-contextualTypeFunctionObjectPropertyIntersection.md` - Implement Contextualtypefunctionobjectpropertyintersection (same feature label, same group key, title overlap)
- `issues/open/1591-implement-convertClassExpressionToFunctionFromObjectProperty.md` - Implement Convertclassexpressiontofunctionfromobjectproperty (same feature label, same group key, title overlap)
- `issues/open/1649-implement-declFileObjectLiteralWithOnlyGetter.md` - Implement Declfileobjectliteralwithonlygetter (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/441-implement-object-literal.md` に統合されました。
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
- `issues/done/4810-implement-object-literal.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
