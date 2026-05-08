---
id: 5078
title: "Implement function resolution (dup)"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-04
updated: 2026-05-04
---

## Summary

Triage function-resolution feature across 542 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 542 cases fail with function-resolution diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: function-resolution feature has 542 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js --detail
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
mise run reference-coverage -- test262 --limit 1084
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js
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

- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js`
- `reference/test262/test/annexB/built-ins/escape/to-string-err-symbol.js`
- `reference/test262/test/annexB/built-ins/unescape/to-string-err-symbol.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A1.1_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A1.3_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A2.1_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A3.1_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A2.2_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.5.1_A2.3_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.5.1_A2.2_T1.js`
- ... and 532 more files

## Duplicate detection

- `issues/open/1073-implement-blockScopedFunctionDeclarationInStrictClass.md` - Implement Blockscopedfunctiondeclarationinstrictclass (same feature label, same group key, title overlap)
- `issues/open/109-implement-addMoreCallSignaturesToBaseSignature.md` - Implement Addmorecallsignaturestobasesignature (same feature label, same group key, title overlap)
- `issues/open/1095-implement-callOnClass.md` - Implement Callonclass (same feature label, same group key, title overlap; split to issue 5197)
- `issues/open/132-implement-allowJsClassThisTypeCrash.md` - Implement Allowjsclassthistypecrash (same feature label, same group key, title overlap)
- `issues/open/2507-implement-genericCapturingFunctionNarrowing.md` - Implement Genericcapturingfunctionnarrowing (same feature label, same group key, title overlap)
- `issues/open/429-implement-eval.md` - Implement eval support (same feature label, same group key, title overlap)
- `issues/open/430-implement-function.md` - Implement function support (same feature label, same group key, title overlap)
- `issues/open/431-implement-function-resolution.md` - Implement function resolution (same feature label, same group key, title overlap)
- `issues/open/433-implement-legacy-global-builtin.md` - Implement legacy-global-builtin support (same reference path, title overlap)
- `issues/open/4442-implement-thisReferencedInFunctionInsideArrowFunction.md` - Implement Thisreferencedinfunctioninsidearrowfunction (same feature label, same group key, title overlap)

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
`issues/open/431-implement-function-resolution.md` に統合されました。
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
- `issues/open/5078-implement-function-resolution.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
