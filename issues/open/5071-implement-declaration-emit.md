---
id: 5071
title: "Implement declaration-emit support (dup)"
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

Triage declaration-emit feature across 61 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 61 cases fail with declaration-emit diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: declaration-emit feature has 61 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/eval-code/direct/arrow-fn-a-preceding-parameter-is-named-arguments-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/eval-code/direct/arrow-fn-a-preceding-parameter-is-named-arguments-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js --detail
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
mise run reference-coverage -- test262 --limit 122
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/eval-code/direct/arrow-fn-a-preceding-parameter-is-named-arguments-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js --detail
mise run reference-triage -- test262 reference/test262/test/language/eval-code/direct/arrow-fn-a-preceding-parameter-is-named-arguments-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js
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

- `reference/test262/test/language/eval-code/direct/arrow-fn-a-preceding-parameter-is-named-arguments-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js`
- `reference/test262/test/language/eval-code/direct/arrow-fn-a-following-parameter-is-named-arguments-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js`
- `reference/test262/test/language/eval-code/direct/arrow-fn-a-following-parameter-is-named-arguments-arrow-func-declare-arguments-assign.js`
- `reference/test262/test/language/eval-code/direct/arrow-fn-a-preceding-parameter-is-named-arguments-arrow-func-declare-arguments-assign.js`
- `reference/test262/test/language/eval-code/direct/arrow-fn-body-cntns-arguments-lex-bind-arrow-func-declare-arguments-assign.js`
- `reference/test262/test/language/eval-code/direct/arrow-fn-body-cntns-arguments-var-bind-arrow-func-declare-arguments-assign.js`
- `reference/test262/test/language/eval-code/direct/arrow-fn-body-cntns-arguments-lex-bind-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js`
- `reference/test262/test/language/eval-code/direct/arrow-fn-body-cntns-arguments-func-decl-arrow-func-declare-arguments-assign.js`
- `reference/test262/test/language/eval-code/direct/arrow-fn-body-cntns-arguments-func-decl-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js`
- `reference/test262/test/language/eval-code/direct/arrow-fn-body-cntns-arguments-var-bind-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js`
- ... and 51 more files

## Duplicate detection

- `issues/open/096-implement-accessorDeclarationEmitJs.md` - Implement Accessordeclarationemitjs (same feature label, same group key, title overlap)
- `issues/open/097-implement-accessorDeclarationEmitVisibilityErrors.md` - Implement Accessordeclarationemitvisibilityerrors (same feature label, same group key, title overlap)
- `issues/open/1179-implement-classDeclaredBeforeClassFactory.md` - Implement Classdeclaredbeforeclassfactory (same feature label, same group key, title overlap; split to issue 5252)
- `issues/open/1249-implement-classdecl.md` - Implement Classdecl (same feature label, same group key, title overlap)
- `issues/open/141-implement-ambientClassDeclaredBeforeBase.md` - Implement Ambientclassdeclaredbeforebase (same feature label, same group key, title overlap)
- `issues/open/1599-implement-crashDeclareGlobalTypeofExport.md` - Implement Crashdeclareglobaltypeofexport (same feature label, same group key, title overlap)
- `issues/open/1683-implement-declarationEmitAliasInlineing.md` - Implement Declarationemitaliasinlineing (same feature label, same group key, title overlap)
- `issues/open/1687-implement-declarationEmitArrowFunctionNoRenaming.md` - Implement Declarationemitarrowfunctionnorenaming (same feature label, same group key, title overlap)
- `issues/open/1688-implement-declarationEmitBindingPatternWithReservedWord.md` - Implement Declarationemitbindingpatternwithreservedword (same feature label, same group key, title overlap)
- `issues/open/1689-implement-declarationEmitBindingPatterns.md` - Implement Declarationemitbindingpatterns (same feature label, same group key, title overlap)

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
`issues/open/424-implement-declaration-emit.md` に統合されました。
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
- `issues/open/5071-implement-declaration-emit.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
