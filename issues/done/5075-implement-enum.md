---
id: 5075
title: "Implement enum support (dup)"
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

Triage enum feature across 59 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 59 cases fail with enum diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: enum feature has 59 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js --detail
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
mise run reference-coverage -- test262 --limit 118
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js
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

- `reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-else-decl-a-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-else-decl-b-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-else-stmt-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-no-else-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-stmt-else-decl-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/switch-case-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/switch-dflt-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/harness/propertyhelper-verifyenumerable-enumerable-symbol.js`
- `reference/test262/test/harness/propertyhelper-verifyenumerable-not-enumerable-symbol.js`
- ... and 49 more files

## Duplicate detection

- `issues/done/1001-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, same group key, title overlap)
- `issues/done/1002-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, same group key, title overlap)
- `issues/done/1012-implement-autonumberingInEnums.md` - Implement Autonumberinginenums (same feature label, same group key, title overlap)
- `issues/done/1070-implement-blockScopedEnumVariablesUseBeforeDef-enum.md` - Implement Blockscopedenumvariablesusebeforedef Enum (same feature label, same group key, title overlap)
- `issues/done/1071-implement-blockScopedEnumVariablesUseBeforeDef-import-export.md` - Implement Blockscopedenumvariablesusebeforedef Import Export (same feature label, same group key, title overlap)
- `issues/open/1099-implement-callOverloads-parser-syntax.md` - Implement Calloverloads Parser Syntax (same feature label, same group key, title overlap)
- `issues/open/1109-implement-capturedLetConstInLoop-import-export.md` - Implement Capturedletconstinloop Import Export (same feature label, same group key, title overlap)
- `issues/open/1111-implement-capturedLetConstInLoop-parser-syntax.md` - Implement Capturedletconstinloop Parser Syntax (same feature label, same group key, title overlap)
- `issues/open/1143-implement-checkSuperCallBeforeThisAccessing-parser-syntax.md` - Implement Checksupercallbeforethisaccessing Parser Syntax (same feature label, same group key, title overlap)
- `issues/open/1164-implement-circularReferenceInReturnType-parser-syntax.md` - Implement Circularreferenceinreturntype Parser Syntax (same feature label, same group key, title overlap)

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
`issues/open/428-implement-enum.md` に統合されました。
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
- `issues/done/5075-implement-enum.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
