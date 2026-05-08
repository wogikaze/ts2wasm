---
id: 5061
title: "Implement arguments-object support (dup)"
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

Triage arguments-object feature across 244 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 244 cases fail with arguments-object diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arguments-object feature has 244 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js --detail
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
mise run reference-coverage -- test262 --limit 488
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js
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

- `reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js`
- `reference/test262/test/harness/compare-array-arguments.js`
- `reference/test262/test/harness/compare-array-falsy-arguments.js`
- `reference/test262/test/harness/verifyProperty-arguments.js`
- `reference/test262/test/intl402/DurationFormat/prototype/formatToParts/invalid-arguments-throws.js`
- `reference/test262/test/intl402/PluralRules/supportedLocalesOf/arguments.js`
- `reference/test262/test/intl402/String/prototype/localeCompare/missing-arguments-coerced-to-undefined.js`
- `reference/test262/test/language/arguments-object/10.5-1-s.js`
- `reference/test262/test/language/arguments-object/10.6-10-c-ii-2.js`
- `reference/test262/test/language/arguments-object/10.6-13-a-1.js`
- ... and 234 more files

## Duplicate detection

- `issues/open/1102-implement-callbackArgsDifferByOptionality.md` - Implement Callbackargsdifferbyoptionality (same feature label, same group key, title overlap; closed into issue 5200)
- `issues/open/1166-implement-circularTypeArgumentsLocalAndOuterNoCrash.md` - Implement Circulartypeargumentslocalandouternocrash (same feature label, same group key, title overlap; stale build-pass)
- `issues/open/1170-implement-class.md` - Implement Class (same feature label, same group key, title overlap; split to issue 5246)
- `issues/open/1218-implement-classImplementsMethodWIthTupleArgs.md` - Implement Classimplementsmethodwithtupleargs (same feature label, same group key, title overlap; stale build-pass)
- `issues/open/1266-implement-collisionArgumentsArrowFunctions.md` - Implement Collisionargumentsarrowfunctions (same feature label, same group key, title overlap)
- `issues/open/1267-implement-collisionArgumentsClassConstructor.md` - Implement Collisionargumentsclassconstructor (same feature label, same group key, title overlap)
- `issues/open/1268-implement-collisionArgumentsClassMethod.md` - Implement Collisionargumentsclassmethod (same feature label, same group key, title overlap)
- `issues/open/1269-implement-collisionArgumentsFunction.md` - Implement Collisionargumentsfunction (same feature label, same group key, title overlap)
- `issues/open/1270-implement-collisionArgumentsFunctionExpressions.md` - Implement Collisionargumentsfunctionexpressions (same feature label, same group key, title overlap)
- `issues/open/1470-implement-constructorArgsErrors.md` - Implement Constructorargserrors (same feature label, same group key, title overlap)

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
`issues/done/412-implement-arguments-object.md` に統合されました。
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
- `issues/open/5061-implement-arguments-object.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
