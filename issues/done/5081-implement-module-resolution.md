---
id: 5081
title: "Implement module-resolution support (dup)"
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

Triage module-resolution feature across 42 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 42 cases fail with module-resolution diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: module-resolution feature has 42 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js --detail
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
mise run reference-coverage -- test262 --limit 84
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js --detail
mise run reference-triage -- test262 reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js
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

- `reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-id-identifier-resolution-trlng.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-id-identifier-resolution-lone.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-id-identifier-resolution-middle.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-id-identifier-resolution-last.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-id-identifier-resolution-first.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-prop-identifier-resolution-lone.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-prop-identifier-resolution-first.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-prop-identifier-resolution-last.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-prop-identifier-resolution-middle.js`
- ... and 32 more files

## Duplicate detection

- `issues/open/1037-implement-baseIndexSignatureResolution.md` - Implement Baseindexsignatureresolution (same feature label, same group key, title overlap)
- `issues/done/133-implement-allowJsCrossMonorepoPackage.md` - Implement Allowjscrossmonorepopackage (same feature label, same group key, title overlap)
- `issues/open/1430-implement-conditionallyDuplicateOverloadsCausedByOverloadResolution.md` - Implement Conditionallyduplicateoverloadscausedbyoverloadresolution (same feature label, same group key, title overlap)
- `issues/done/169-implement-ambiguousOverloadResolution.md` - Implement Ambiguousoverloadresolution (same feature label, same group key, title overlap)
- `issues/done/2058-implement-duplicatePackage-module-resolution.md` - Implement Duplicatepackage Module Resolution (same feature label, same group key, title overlap)
- `issues/open/2443-implement-functionDeclarationWithResolutionOfTypeNamedArguments.md` - Implement Functiondeclarationwithresolutionoftypenamedarguments (same feature label, same group key, title overlap)
- `issues/open/2444-implement-functionDeclarationWithResolutionOfTypeOfSameName.md` - Implement Functiondeclarationwithresolutionoftypeofsamename (same feature label, same group key, title overlap)
- `issues/open/2448-implement-functionExpressionWithResolutionOfTypeNamedArguments.md` - Implement Functionexpressionwithresolutionoftypenamedarguments (same feature label, same group key, title overlap)
- `issues/open/2449-implement-functionExpressionWithResolutionOfTypeOfSameName.md` - Implement Functionexpressionwithresolutionoftypeofsamename (same feature label, same group key, title overlap)
- `issues/done/3371-implement-moduleResolution-module-resolution.md` - Implement Moduleresolution Module Resolution (same feature label, same group key, title overlap)

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
`issues/done/436-implement-module-resolution.md` に統合されました。
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
