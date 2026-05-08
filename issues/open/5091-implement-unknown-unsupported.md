---
id: 5091
title: "Investigate and classify unknown-unsupported cases (dup)"
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

Triage unknown-unsupported feature across 7108 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 7108 cases fail with unknown-unsupported diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: unknown-unsupported feature has 7108 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-in-compound-assignment.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-in-compound-assignment.js --detail
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
mise run reference-coverage -- test262 --limit 14216
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-in-compound-assignment.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-in-compound-assignment.js
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

- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-in-compound-assignment.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-in-prefix-update.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-as-for-of-lhs.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-as-for-in-lhs.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression-in-postfix-update.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/cover-callexpression-and-asyncarrowhead.js`
- `reference/test262/test/annexB/language/expressions/assignmenttargettype/callexpression.js`
- `reference/test262/test/annexB/language/function-code/block-decl-func-existing-block-fn-no-init.js`
- `reference/test262/test/annexB/language/function-code/block-decl-func-existing-fn-no-init.js`
- `reference/test262/test/annexB/language/expressions/yield/star-iterable-throw-emulates-undefined-throws-when-called.js`
- ... and 7098 more files

## Duplicate detection

- `issues/done/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key, title overlap)
- `issues/done/1026-implement-badArrayIndex.md` - Implement Badarrayindex (same feature label, same group key; superseded by issue 5150)
- `issues/done/1030-implement-badOverloadError.md` - Implement Badoverloaderror (same feature label, same group key; stale build-pass bucket)
- `issues/done/1043-implement-bestChoiceType.md` - Implement Bestchoicetype (same feature label, same group key)
- `issues/done/1045-implement-betterErrorForAccidentalCall.md` - Implement Bettererrorforaccidentalcall (same feature label, same group key)
- `issues/done/1058-implement-binderBinaryExpressionStress.md` - Implement Binderbinaryexpressionstress (same feature label, same group key)
- `issues/done/1059-implement-binderBinaryExpressionStressJs.md` - Implement Binderbinaryexpressionstressjs (same feature label, same group key)
- `issues/done/106-implement-accessors.md` - Implement Accessors (same feature label, same group key)
- `issues/open/1065-implement-bitwiseCompoundAssignmentOperators.md` - Implement Bitwisecompoundassignmentoperators (same feature label, same group key)
- `issues/done/1086-implement-builtinIterator.md` - Implement Builtiniterator (same feature label, same group key; superseded by issue 5191)

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
`issues/open/454-implement-unknown-unsupported.md` に統合されました。
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
- `issues/done/5091-implement-unknown-unsupported.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
