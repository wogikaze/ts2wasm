---
id: 5076
title: "Implement eval support (dup)"
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

Triage eval feature across 703 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 703 cases fail with eval diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: eval feature has 703 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape-BMP.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape-BMP.js --detail
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
mise run reference-coverage -- test262 --limit 1406
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape-BMP.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape-BMP.js
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

- `reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-leading-escape-BMP.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-fn-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-no-skip-try.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-fn-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-no-skip-param.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-skip-early-err-block.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-skip-early-err-for-in.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-skip-early-err-try.js`
- ... and 693 more files

## Duplicate detection

- `issues/done/066-implement-regexp-literal.md` - Implement RegExp literal support (same reference path, title overlap)
- `issues/open/1170-implement-class.md` - Implement Class (same feature label, same group key, title overlap)
- `issues/done/128-implement-aliasUsedAsNameValue.md` - Implement Aliasusedasnamevalue (same feature label, same group key, title overlap)
- `issues/done/132-implement-allowJsClassThisTypeCrash.md` - Implement Allowjsclassthistypecrash (same feature label, same group key, title overlap)
- `issues/open/1988-implement-discriminantUsingEvaluatableTemplateExpression.md` - Implement Discriminantusingevaluatabletemplateexpression (same feature label, same group key, title overlap)
- `issues/open/2068-implement-dynamicImportEvaluateSpecifier.md` - Implement Dynamicimportevaluatespecifier (same feature label, same group key, title overlap)
- `issues/open/2214-implement-evalAfter.md` - Implement Evalafter (same feature label, same group key, title overlap)
- `issues/open/2215-implement-evalOrArgumentsInDeclarationFunctions.md` - Implement Evalorargumentsindeclarationfunctions (same feature label, same group key, title overlap)
- `issues/open/2410-implement-for.md` - Implement For (same feature label, same group key, title overlap)
- `issues/open/2465-implement-functionType.md` - Implement Functiontype (same feature label, same group key, title overlap)

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
`issues/open/429-implement-eval.md` に統合されました。
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
