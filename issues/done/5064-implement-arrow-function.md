---
id: 5064
title: "Implement arrow functions (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-04
updated: 2026-05-04
---

## Summary

Triage arrow-function feature across 175 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 175 cases fail with arrow-function diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrow-function feature has 175 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/expressions/arrow-function/dflt-params-arg-val-not-undefined.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/arrow-function/dflt-params-arg-val-not-undefined.js --detail
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
mise run reference-coverage -- test262 --limit 350
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/arrow-function/dflt-params-arg-val-not-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/language/expressions/arrow-function/dflt-params-arg-val-not-undefined.js
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

- `reference/test262/test/language/expressions/arrow-function/dflt-params-arg-val-not-undefined.js`
- `reference/test262/test/language/expressions/arrow-function/arrow/capturing-closure-variables-2.js`
- `reference/test262/test/language/expressions/arrow-function/arrow/capturing-closure-variables-1.js`
- `reference/test262/test/language/expressions/arrow-function/arrow/concisebody-lookahead-assignmentexpression-1.js`
- `reference/test262/test/language/expressions/arrow-function/dstr/ary-ptrn-elem-ary-elem-init.js`
- `reference/test262/test/language/expressions/arrow-function/dflt-params-trailing-comma.js`
- `reference/test262/test/language/expressions/arrow-function/dstr/ary-ptrn-elem-ary-empty-init.js`
- `reference/test262/test/language/expressions/arrow-function/dstr/ary-ptrn-elem-ary-elem-iter.js`
- `reference/test262/test/language/expressions/arrow-function/dstr/ary-ptrn-elem-ary-rest-iter.js`
- `reference/test262/test/language/expressions/arrow-function/dstr/ary-ptrn-elem-ary-empty-iter.js`
- ... and 165 more files

## Duplicate detection

- `issues/done/1031-implement-badThisBinding.md` - Implement Badthisbinding (same feature label, same group key, title overlap; split to issue 5152)
- `issues/done/1107-implement-capturedLetConstInLoop-arrow-function.md` - Implement Capturedletconstinloop Arrow Function (same feature label, same group key, title overlap; stale build-pass bucket)
- `issues/done/1144-implement-checkSwitchStatementIfCaseTypeIsString.md` - Implement Checkswitchstatementifcasetypeisstring (same feature label, same group key, title overlap)
- `issues/done/1196-implement-classExtendsAcrossFiles.md` - Implement Classextendsacrossfiles (same feature label, same group key, title overlap)
- `issues/open/1300-implement-collisionRestParameterArrowFunctions.md` - Implement Collisionrestparameterarrowfunctions (same feature label, same group key, title overlap)
- `issues/open/1308-implement-collisionSuperAndLocalFunctionInConstructor.md` - Implement Collisionsuperandlocalfunctioninconstructor (same feature label, same group key, title overlap)
- `issues/open/1312-implement-collisionSuperAndLocalVarInConstructor.md` - Implement Collisionsuperandlocalvarinconstructor (same feature label, same group key, title overlap)
- `issues/done/132-implement-allowJsClassThisTypeCrash.md` - Implement Allowjsclassthistypecrash (same feature label, same group key, title overlap)
- `issues/done/1401-implement-compositeContextualSignature.md` - Implement Compositecontextualsignature (same feature label, same group key, title overlap; now closed as stale build-pass)
- `issues/open/1490-implement-contextualOverloadListFromArrayUnion.md` - Implement Contextualoverloadlistfromarrayunion (same feature label, same group key, title overlap)

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
`issues/done/415-implement-arrow-function.md` に統合されました。
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
- `issues/done/5064-implement-arrow-function.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
