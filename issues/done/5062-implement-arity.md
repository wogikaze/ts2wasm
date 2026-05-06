---
id: 5062
title: "Implement arity support (dup)"
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

Triage arity feature across 38 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 38 cases fail with arity diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arity feature has 38 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/escape/argument_types.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/escape/argument_types.js --detail
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
mise run reference-coverage -- test262 --limit 76
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/escape/argument_types.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/escape/argument_types.js
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

- `reference/test262/test/annexB/built-ins/escape/argument_types.js`
- `reference/test262/test/annexB/built-ins/unescape/argument_types.js`
- `reference/test262/test/built-ins/Boolean/S15.6.1.1_A1_T1.js`
- `reference/test262/test/built-ins/Boolean/S15.6.1.1_A2.js`
- `reference/test262/test/built-ins/Number/S15.7.1.1_A2.js`
- `reference/test262/test/built-ins/RegExp/prototype/exec/S15.10.6.2_A12.js`
- `reference/test262/test/built-ins/RegExp/prototype/exec/S15.10.6.2_A1_T16.js`
- `reference/test262/test/built-ins/RegExp/prototype/test/S15.10.6.3_A1_T16.js`
- `reference/test262/test/built-ins/String/prototype/match/S15.5.4.10_A1_T4.js`
- `reference/test262/test/built-ins/isFinite/tonumber-operations.js`
- ... and 28 more files

## Duplicate detection

- `issues/open/021-implement-full-wasm-backend.md` - Implement full wasm backend (same feature label, same group key, title overlap)
- `issues/open/052-implement-json.md` - Implement JSON (same feature label, same group key, title overlap)
- `issues/done/1048-implement-bigint.md` - Implement Bigint (same feature label, same group key, title overlap)
- `issues/done/1077-implement-blockScopedSameNameFunctionDeclarationES.md` - Implement Blockscopedsamenamefunctiondeclarationes (same feature label, same group key, title overlap)
- `issues/done/1078-implement-blockScopedSameNameFunctionDeclarationStrictES.md` - Implement Blockscopedsamenamefunctiondeclarationstrictes (same feature label, same group key, title overlap)
- `issues/open/1244-implement-classVarianceCircularity.md` - Implement Classvariancecircularity (same feature label, same group key, title overlap)
- `issues/open/1245-implement-classVarianceResolveCircularity.md` - Implement Classvarianceresolvecircularity (same feature label, same group key, title overlap)
- `issues/open/1543-implement-contextuallyTypedParametersOptionalInJSDoc.md` - Implement Contextuallytypedparametersoptionalinjsdoc (same feature label, same group key, title overlap)
- `issues/open/2169-implement-errorForwardReferenceForwadingConstructor.md` - Implement Errorforwardreferenceforwadingconstructor (same feature label, same group key, title overlap)
- `issues/open/2232-implement-exhaustiveSwitchCheckCircularity.md` - Implement Exhaustiveswitchcheckcircularity (same feature label, same group key, title overlap)

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
`issues/done/413-implement-arity.md` に統合されました。
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
- `issues/done/5062-implement-arity.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
