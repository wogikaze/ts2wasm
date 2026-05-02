---
id: 4812
title: "Implement RegExp literal support"
type: spike
area: runtime/builtins
class: blocked
priority: P1
depends_on: [5004]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

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
mise run reference-coverage -- tsgo --limit 10
mise run reference-coverage -- tsgo --path-filter reference/typescript-go/testdata/tests/cases/compiler/contextuallyTypedJsxChildren2.tsx --detail
mise run reference-triage -- tsgo reference/typescript-go/testdata/tests/cases/compiler/contextuallyTypedJsxChildren2.tsx
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
