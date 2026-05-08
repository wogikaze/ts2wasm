---
id: 4812
title: "Implement RegExp literal support (dup)"
type: spike
area: runtime/builtins
class: blocked
priority: P1
depends_on: [5004]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

> **Reopened by audit** (2026-05-06)
> Classification: false-done (blocked)
> Reason: relapsed false-done: reopened in df7621e3, re-closed without implementation. No implementation commits.
>
> True-done checklist:
> 1. Implementation commits in the repo that satisfy the acceptance criteria
> 2. Filled completion evidence section with commits and validation results
> 3. No relapsed false-done pattern (previously reopened but re-closed without evidence)

## Summary

Triage regexp-literal feature across 5 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 5 cases fail with regexp-literal diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: regexp-literal feature has 5 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/contextuallyTypedJsxChildren2.tsx
```

Coverage window:

```sh
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/contextuallyTypedJsxChildren2.tsx --detail
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

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- parser/resolver code unless `reference-triage` proves the failure happens before runtime lowering

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
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/contextuallyTypedJsxChildren2.tsx --detail
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/contextuallyTypedJsxChildren2.tsx
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

- `reference/typescript-go/testdata/tests/cases/compiler/contextuallyTypedJsxChildren2.tsx`
- `reference/typescript-go/testdata/tests/cases/compiler/expandoContextualTypes.tsx`
- `reference/typescript-go/testdata/tests/cases/compiler/jsxFunctionTypeChildren.tsx`
- `reference/typescript-go/testdata/tests/cases/compiler/jsxNestedIndentation.tsx`
- `reference/typescript-go/testdata/tests/cases/compiler/jsxTernaryWithObjectInAttribute.tsx`

## Duplicate detection

- `issues/open/066-implement-regexp-literal.md` - Implement RegExp literal support (same feature label, same group key, title overlap)
- `issues/open/1139-implement-checkJsxNotSetError.md` - Implement Checkjsxnotseterror (same feature label, same group key, title overlap)
- `issues/open/201-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same reference path)
- `issues/open/2230-implement-excessiveStackDepthFlatArray.md` - Implement Excessivestackdepthflatarray (same feature label, same group key, title overlap)
- `issues/open/2872-implement-initializedDestructuringAssignmentTypes.md` - Implement Initializeddestructuringassignmenttypes (same feature label, same group key, title overlap)
- `issues/open/3097-implement-jsFileCompilationTypeArgumentSyntaxOfCall.md` - Implement Jsfilecompilationtypeargumentsyntaxofcall (same feature label, same group key, title overlap)
- `issues/open/3125-implement-jsxEmitWithAttributes.md` - Implement Jsxemitwithattributes (same feature label, same group key, title overlap)
- `issues/open/3126-implement-jsxFactoryAndReactNamespace.md` - Implement Jsxfactoryandreactnamespace (same feature label, same group key, title overlap)
- `issues/open/3127-implement-jsxFactoryIdentifier.md` - Implement Jsxfactoryidentifier (same feature label, same group key, title overlap)
- `issues/open/3130-implement-jsxFactoryMissingErrorInsideAClass.md` - Implement Jsxfactorymissingerrorinsideaclass (same feature label, same group key, title overlap)

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/5020-implement-regexp-literal.md` に統合されました。
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

## Close note

Closed as duplicate of `issues/open/066-implement-regexp-literal.md`. All work tracked under issue 066.

superseded-by: 066


---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This is a generated triage bucket issue. It was
created as a `class: blocked` spike with `depends_on` pointing to a parent
meta-issue (5004 or 5007). When the parent meta-issue was moved to
`issues/done/`, this child issue was dragged along without any implementation
or triage work. The `
## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/5020-implement-regexp-literal.md` に統合されました。
そちらを参照してください。
## Completion evidence` section is unfilled (commits
placeholder `...`, validation result empty). Zero implementation commits
reference this issue.

**True-done checklist** (all must pass):

1. **Triage the representative failure path**: Confirm it is superseded by an
   existing open/done issue OR split into implementation-ready child issues
   with exact reproduction commands.

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - Issue URL or child issue path documenting the triage outcome
   - Or: the exact failing reference path has a matching open/done issue
   - Or: the failing test case no longer reproduces the original diagnostic
