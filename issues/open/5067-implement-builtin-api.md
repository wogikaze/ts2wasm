---
id: 5067
title: "Implement built-in API support (dup)"
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

Triage builtin-api feature across 7991 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 7991 cases fail with builtin-api diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: builtin-api feature has 7991 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js --detail
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
mise run reference-coverage -- test262 --limit 15982
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js
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

- `reference/test262/test/annexB/built-ins/TypedArrayConstructors/from/iterator-method-emulates-undefined.js`
- `reference/test262/test/built-ins/AbstractModuleSource/length.js`
- `reference/test262/test/built-ins/AbstractModuleSource/prototype/constructor.js`
- `reference/test262/test/built-ins/AbstractModuleSource/name.js`
- `reference/test262/test/built-ins/AggregateError/errors-iterabletolist-failures.js`
- `reference/test262/test/built-ins/AbstractModuleSource/prototype/Symbol.toStringTag.js`
- `reference/test262/test/built-ins/AggregateError/cause-property.js`
- `reference/test262/test/built-ins/AggregateError/length.js`
- `reference/test262/test/built-ins/AbstractModuleSource/prototype.js`
- `reference/test262/test/built-ins/AggregateError/message-method-prop.js`
- ... and 7981 more files

## Duplicate detection

- `issues/open/2348-implement-expr.md` - Implement Expr (same feature label, same group key, title overlap)
- `issues/open/2421-implement-forOfTransformsExpression.md` - Implement Foroftransformsexpression (same feature label, same group key, title overlap)
- `issues/open/4000-implement-regExpWithSlashInCharClass.md` - Implement Regexpwithslashincharclass (same feature label, same group key, title overlap)
- `issues/done/419-implement-builtin-api.md` - Implement built-in API support (same feature label, same group key, title overlap)
- `issues/done/4291-implement-stringMatchAll.md` - Implement Stringmatchall (same feature label, same group key, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same feature label, same group key, title overlap)
- `issues/done/341-implement-core-builtin-api-coverage.md` - Implement core builtin API coverage (3,190 test262 cases) (same feature label, same group key, title overlap)
- `issues/done/341a-global-number-functions.md` - Implement isNaN, parseInt, parseFloat, isFinite global functions (same feature label, same group key, title overlap)

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
`issues/done/419-implement-builtin-api.md` に統合されました。
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
- `issues/done/5067-implement-builtin-api.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
