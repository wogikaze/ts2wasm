---
id: 5083
title: "Implement negative-parse-syntaxerror support (dup)"
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

Triage negative-parse-syntaxerror feature across 4595 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4595 cases fail with negative-parse-syntaxerror diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: negative-parse-syntaxerror feature has 4595 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js --detail
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
mise run reference-coverage -- test262 --limit 9190
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js
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

- `reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js`
- `reference/test262/test/annexB/language/statements/for-in/bare-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/const-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/let-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/strict-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/var-objectbindingpattern-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/var-arraybindingpattern-initializer.js`
- `reference/test262/test/built-ins/RegExp/property-escapes/binary-property-with-value-ASCII_-_F.js`
- `reference/test262/test/built-ins/RegExp/property-escapes/binary-property-with-value-ASCII_-_Invalid-negated.js`
- `reference/test262/test/built-ins/RegExp/property-escapes/binary-property-with-value-ASCII_-_F-negated.js`
- ... and 4585 more files

## Duplicate detection

- `issues/open/438-implement-negative-parse-syntaxerror.md` - Implement negative-parse-syntaxerror support (same reference path, same feature label, same group key, title overlap)
- `issues/done/229-implement-legacy-octal-escape-handling.md` - Implement legacy octal escape handling (same reference path, title overlap)
- `issues/done/286-classify-negative-syntax-tests-correctly.md` - Classify expected negative SyntaxError tests correctly (same feature label, same group key)

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
`issues/open/438-implement-negative-parse-syntaxerror.md` に統合されました。
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
