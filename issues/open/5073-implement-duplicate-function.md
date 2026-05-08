---
id: 5073
title: "Implement duplicate-function support (dup)"
type: spike
area: reference/triage
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-04
updated: 2026-05-04
---

## Summary

Triage duplicate-function feature across 7 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 7 cases fail with duplicate-function diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: duplicate-function feature has 7 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/destructuring/binding/syntax/recursive-array-and-object-patterns.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/destructuring/binding/syntax/recursive-array-and-object-patterns.js --detail
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
mise run reference-coverage -- test262 --limit 14
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/destructuring/binding/syntax/recursive-array-and-object-patterns.js --detail
mise run reference-triage -- test262 reference/test262/test/language/destructuring/binding/syntax/recursive-array-and-object-patterns.js
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

- `reference/test262/test/language/destructuring/binding/syntax/recursive-array-and-object-patterns.js`
- `reference/test262/test/language/eval-code/direct/var-env-func-init-multi.js`
- `reference/test262/test/language/global-code/decl-func-dup.js`
- `reference/test262/test/language/statements/function/S13_A6_T1.js`
- `reference/test262/test/language/statements/function/S13_A6_T2.js`
- `reference/test262/test/language/statements/function/S14_A5_T1.js`
- `reference/test262/test/language/statements/function/S14_A5_T2.js`

## Duplicate detection

- `issues/open/1347-implement-commentOnClassAccessor.md` - Implement Commentonclassaccessor (same feature label, same group key, title overlap)
- `issues/open/2043-implement-duplicateIdentifierRelatedSpans-duplicate-function.md` - Implement Duplicateidentifierrelatedspans Duplicate Function (same feature label, same group key, title overlap)
- `issues/open/2600-implement-getAndSetNotIdenticalType-duplicate-function.md` - Implement Getandsetnotidenticaltype Duplicate Function (same feature label, same group key, title overlap)
- `issues/open/2803-implement-inferSetterParamType.md` - Implement Infersetterparamtype (same feature label, same group key, title overlap)
- `issues/open/2853-implement-inheritanceMemberAccessorOverridingAccessor.md` - Implement Inheritancememberaccessoroverridingaccessor (same feature label, same group key, title overlap)
- `issues/open/2854-implement-inheritanceMemberAccessorOverridingMethod.md` - Implement Inheritancememberaccessoroverridingmethod (same feature label, same group key, title overlap)
- `issues/open/2855-implement-inheritanceMemberAccessorOverridingProperty.md` - Implement Inheritancememberaccessoroverridingproperty (same feature label, same group key, title overlap)
- `issues/open/2856-implement-inheritanceMemberFuncOverridingAccessor.md` - Implement Inheritancememberfuncoverridingaccessor (same feature label, same group key, title overlap)
- `issues/open/2860-implement-inheritanceStaticAccessorOverridingAccessor.md` - Implement Inheritancestaticaccessoroverridingaccessor (same feature label, same group key, title overlap)
- `issues/open/2861-implement-inheritanceStaticAccessorOverridingMethod.md` - Implement Inheritancestaticaccessoroverridingmethod (same feature label, same group key, title overlap)

## Smart triage

### Smart triage unavailable

```text
Traceback (most recent call last):
  File "/home/wogikaze/wgkz/ts2wasm/scripts/run/reference-triage.py", line 597, in <module>
    raise SystemExit(main(sys.argv[1:]))
  File "/home/wogikaze/wgkz/ts2wasm/scripts/run/reference-triage.py", line 588, in main
    report = build_report(args.suite, path, args.max_dump_chars)
  File "/home/wogikaze/wgkz/ts2wasm/scripts/run/reference-triage.py", line 469, in build_report
    build_input, oracle_input, source, diagnostic_source = prepare_triage_input(
  File "/home/wogikaze/wgkz/ts2wasm/scripts/run/reference-triage.py", line 448, in prepare_triage_input
    metadata = REFERENCE_COVERAGE.test262_runner.parse_test262_metadata(source)
AttributeError: 'NoneType' object has no attribute 'parse_test262_metadata'
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/426-implement-duplicate-function.md` に統合されました。
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

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/5073-implement-duplicate-function.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
