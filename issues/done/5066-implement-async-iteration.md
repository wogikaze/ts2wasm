---
id: 5066
title: "Implement async-iteration support (dup)"
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

Triage async-iteration feature across 480 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 480 cases fail with async-iteration diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: async-iteration feature has 480 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-fn.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-fn.js --detail
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
mise run reference-coverage -- test262 --limit 960
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-fn.js --detail
mise run reference-triage -- test262 reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-fn.js
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

- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-fn.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-gen.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-class.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-cover.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-arrow.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-rest-iteration.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-obj-id-init-fn-name-class.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-obj-id-init-fn-name-cover.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-obj-id-init-fn-name-fn.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-obj-id-init-fn-name-arrow.js`
- ... and 470 more files

## Duplicate detection

- `issues/done/416-implement-async.md` - Implement async/await support (same feature label, same group key, title overlap)
- `issues/open/417-implement-async-iteration.md` - Implement async-iteration support (same reference path, same feature label, same group key, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/230-implement-async-iteration-for-await-of.md` - Implement async iteration and for-await-of (same feature label, same group key, title overlap)
- `issues/done/444-implement-regexp-literal.md` - Implement RegExp literal support (same feature label, same group key, title overlap)

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
`issues/open/417-implement-async-iteration.md` に統合されました。
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
- `issues/done/5066-implement-async-iteration.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
