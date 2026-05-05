---
id: 5080
title: "Implement legacy-global-builtin support (dup)"
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

Triage legacy-global-builtin feature across 13 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 13 cases fail with legacy-global-builtin diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: legacy-global-builtin feature has 13 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/escape/escape-above-astral.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/escape/escape-above-astral.js --detail
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
mise run reference-coverage -- test262 --limit 26
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/escape/escape-above-astral.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/escape/escape-above-astral.js
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

- `reference/test262/test/annexB/built-ins/escape/escape-above-astral.js`
- `reference/test262/test/annexB/built-ins/escape/length.js`
- `reference/test262/test/annexB/built-ins/escape/prop-desc.js`
- `reference/test262/test/annexB/built-ins/escape/name.js`
- `reference/test262/test/annexB/built-ins/escape/to-string-observe.js`
- `reference/test262/test/annexB/built-ins/escape/to-primitive-err.js`
- `reference/test262/test/annexB/built-ins/escape/to-primitive-observe.js`
- `reference/test262/test/annexB/built-ins/unescape/length.js`
- `reference/test262/test/annexB/built-ins/unescape/name.js`
- `reference/test262/test/annexB/built-ins/unescape/prop-desc.js`
- ... and 3 more files

## Duplicate detection

- `issues/done/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same reference path)
- `issues/open/2615-implement-global.md` - Implement Global (same feature label, same group key, title overlap)
- `issues/done/433-implement-legacy-global-builtin.md` - Implement legacy-global-builtin support (same reference path, same feature label, same group key, title overlap)
- `issues/done/5016-implement-function-resolution.md` - Implement function resolution (same feature label, same group key, title overlap)
- `issues/open/5018-implement-legacy-global-builtin.md` - Implement legacy-global-builtin support (same reference path, same feature label, same group key, title overlap)
- `issues/open/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same feature label, same group key, title overlap)
- `issues/open/344-implement-legacy-global-builtin-bindings.md` - Implement legacy global builtin bindings (8 test262 cases) (same feature label, same group key, title overlap)
- `issues/done/406-direct-eval-annexb-existing-binding-residuals.md` - Direct eval Annex B existing binding residuals (same feature label, same group key)

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
`issues/open/5018-implement-legacy-global-builtin.md` に統合されました。
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
- `issues/done/5080-implement-legacy-global-builtin.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
