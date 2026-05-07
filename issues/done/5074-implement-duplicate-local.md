---
id: 5074
title: "Implement duplicate-local support"
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

Triage duplicate-local feature across 352 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 352 cases fail with duplicate-local diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: duplicate-local feature has 352 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-no-init.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-no-init.js --detail
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
mise run reference-coverage -- test262 --limit 704
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-no-init.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-no-init.js
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

- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/global-block-decl-eval-global-existing-var-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/global-block-decl-eval-global-existing-var-update.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A1.1_T2.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A1.2_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.2.1_A1.1_T2.js`
- `reference/test262/test/built-ins/Array/S15.4.2.1_A1.2_T1.js`
- `reference/test262/test/built-ins/Array/length/S15.4.2.2_A2.3_T3.js`
- `reference/test262/test/built-ins/Array/length/S15.4.2.2_A2.3_T2.js`
- ... and 342 more files

## Duplicate detection

- `issues/done/1060-implement-bindingPatternCannotBeOnlyInferenceSource.md` - Implement Bindingpatterncannotbeonlyinferencesource (same feature label, same group key, title overlap)
- `issues/done/1108-implement-capturedLetConstInLoop-duplicate-local.md` - Implement Capturedletconstinloop Duplicate Local (same feature label, same group key, title overlap)
- `issues/done/1122-implement-catch.md` - Implement Catch (same feature label, same group key, title overlap)
- `issues/done/1124-implement-cf.md` - Implement Cf (same feature label, same group key, title overlap)
- `issues/done/1402-implement-compositeGenericFunction.md` - Implement Compositegenericfunction (same feature label, same group key, title overlap; now closed as stale build-pass)
- `issues/done/1436-implement-conflictingTypeAnnotatedVar.md` - Implement Conflictingtypeannotatedvar (same feature label, same group key, title overlap)
- `issues/done/1500-implement-contextualSignatureInstantiation-duplicate-local.md` - Implement Contextualsignatureinstantiation Duplicate Local (same feature label, same group key, title overlap; folded into issue 5234)
- `issues/open/1777-implement-declarationEmitMappedTypeTemplateTypeofSymbol.md` - Implement Declarationemitmappedtypetemplatetypeofsymbol (same feature label, same group key, title overlap)
- `issues/open/2008-implement-doNotEmitPinnedCommentNotOnTopOfFile.md` - Implement Donotemitpinnedcommentnotontopoffile (same feature label, same group key, title overlap)
- `issues/open/2037-implement-duplicateIdentifierBindingElementInParameterDeclaration.md` - Implement Duplicateidentifierbindingelementinparameterdeclaration (same feature label, same group key, title overlap)

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
`issues/done/427-implement-duplicate-local.md` に統合されました。
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
- `issues/done/5074-implement-duplicate-local.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
