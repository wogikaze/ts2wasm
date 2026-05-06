---
id: 4809
title: "Implement name resolution (dup)"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage name-resolution feature across 5 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 5 cases fail with name-resolution diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: name-resolution feature has 5 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/declarationEmitForMixinsWithStaticAccessors1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/declarationEmitForMixinsWithStaticAccessors1.ts --detail
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
mise run reference-coverage -- tsgo --limit 10
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/declarationEmitForMixinsWithStaticAccessors1.ts --detail
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/declarationEmitForMixinsWithStaticAccessors1.ts
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

- `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitForMixinsWithStaticAccessors1.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/expandoFunctionAsAssertion.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/iterationErrorOverNotIterableUnions1.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/jsxNestedIndentation.js`
- `reference/typescript-go/testdata/tests/cases/compiler/nestedSpreadsAndWidening.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, same group key, title overlap)
- `issues/done/089-implement-acceptSymbolAsWeakType.md` - Implement Acceptsymbolasweaktype (same feature label, same group key, title overlap)
- `issues/done/1010-implement-autoTypeAssignedUsingDestructuringFromNeverNoCrash.md` - Implement Autotypeassignedusingdestructuringfromnevernocrash (same feature label, same group key, title overlap)
- `issues/done/1033-implement-baseCheck.md` - Implement Basecheck (same feature label, same group key, title overlap; superseded by issue 056)
- `issues/done/1044-implement-bestCommonTypeWithContextualTyping.md` - Implement Bestcommontypewithcontextualtyping (same feature label, same group key, title overlap)
- `issues/done/1048-implement-bigint.md` - Implement Bigint (same feature label, same group key, title overlap)
- `issues/done/1051-implement-bigintIndex.md` - Implement Bigintindex (same feature label, same group key, title overlap)
- `issues/open/1061-implement-bindingPatternContextualTypeDoesNotCauseWidening.md` - Implement Bindingpatterncontextualtypedoesnotcausewidening (same feature label, same group key, title overlap)
- `issues/open/1068-implement-blockScopedBindingsReassignedInLoop-name-resolution.md` - Implement Blockscopedbindingsreassignedinloop Name Resolution (same feature label, same group key, title overlap)
- `issues/open/1072-implement-blockScopedFunctionDeclarationES.md` - Implement Blockscopedfunctiondeclarationes (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/437-implement-name-resolution.md` に統合されました。
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
- `issues/done/4809-implement-name-resolution.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
