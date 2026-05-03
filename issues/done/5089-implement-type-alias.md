---
id: 5089
title: "Implement type-alias support (dup)"
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

Triage type-alias feature across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3 cases fail with type-alias diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: type-alias feature has 3 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/rest-parameters/no-alias-arguments.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/rest-parameters/no-alias-arguments.js --detail
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
mise run reference-coverage -- test262 --limit 6
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/rest-parameters/no-alias-arguments.js --detail
mise run reference-triage -- test262 reference/test262/test/language/rest-parameters/no-alias-arguments.js
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

- `reference/test262/test/language/rest-parameters/no-alias-arguments.js`
- `reference/test262/test/language/statements/for-of/arguments-unmapped-aliasing.js`
- `reference/test262/test/language/statements/for-of/arguments-mapped-aliasing.js`

## Duplicate detection

- `issues/open/090-implement-acceptableAlias.md` - Implement Acceptablealias (same feature label, same group key, title overlap)
- `issues/open/111-implement-aliasAssignments.md` - Implement Aliasassignments (same feature label, same group key, title overlap)
- `issues/open/112-implement-aliasBug.md` - Implement Aliasbug (same feature label, same group key, title overlap)
- `issues/open/113-implement-aliasDoesNotDuplicateSignatures.md` - Implement Aliasdoesnotduplicatesignatures (same feature label, same group key, title overlap)
- `issues/open/114-implement-aliasErrors.md` - Implement Aliaserrors (same feature label, same group key, title overlap)
- `issues/open/115-implement-aliasInaccessibleModule.md` - Implement Aliasinaccessiblemodule (same feature label, same group key, title overlap)
- `issues/open/116-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md` - Implement Aliasinstantiationexpressiongenericintersectionnocrash (same feature label, same group key, title overlap)
- `issues/open/117-implement-aliasOfGenericFunctionWithRestBehavedSameAsUnaliased.md` - Implement Aliasofgenericfunctionwithrestbehavedsameasunaliased (same feature label, same group key, title overlap)
- `issues/open/118-implement-aliasOnMergedModuleInterface.md` - Implement Aliasonmergedmoduleinterface (same feature label, same group key, title overlap)
- `issues/open/120-implement-aliasUsageInArray.md` - Implement Aliasusageinarray (same feature label, same group key, title overlap)

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
`issues/open/452-implement-type-alias.md` に統合されました。
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
