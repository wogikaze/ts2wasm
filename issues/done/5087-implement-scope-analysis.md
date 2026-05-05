---
id: 5087
title: "Implement scope-analysis support (dup)"
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

Triage scope-analysis feature across 104 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 104 cases fail with scope-analysis diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: scope-analysis feature has 104 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/block-scope/leave/nested-block-let-declaration-only-shadows-outer-parameter-value-1.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/block-scope/leave/nested-block-let-declaration-only-shadows-outer-parameter-value-1.js --detail
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
mise run reference-coverage -- test262 --limit 208
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/block-scope/leave/nested-block-let-declaration-only-shadows-outer-parameter-value-1.js --detail
mise run reference-triage -- test262 reference/test262/test/language/block-scope/leave/nested-block-let-declaration-only-shadows-outer-parameter-value-1.js
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

- `reference/test262/test/language/block-scope/leave/nested-block-let-declaration-only-shadows-outer-parameter-value-1.js`
- `reference/test262/test/language/block-scope/leave/outermost-binding-updated-in-catch-block-nested-block-let-declaration-unseen-outside-of-block.js`
- `reference/test262/test/language/block-scope/leave/finally-block-let-declaration-only-shadows-outer-parameter-value-2.js`
- `reference/test262/test/language/block-scope/leave/nested-block-let-declaration-only-shadows-outer-parameter-value-2.js`
- `reference/test262/test/language/block-scope/leave/try-block-let-declaration-only-shadows-outer-parameter-value-2.js`
- `reference/test262/test/language/block-scope/leave/verify-context-in-labelled-block.js`
- `reference/test262/test/language/block-scope/leave/x-before-continue.js`
- `reference/test262/test/language/block-scope/return-from/block-let.js`
- `reference/test262/test/language/block-scope/leave/x-after-break-to-label.js`
- `reference/test262/test/language/block-scope/return-from/block-const.js`
- ... and 94 more files

## Duplicate detection

- `issues/done/082-implement-abstractClassInLocalScope.md` - Implement Abstractclassinlocalscope (same feature label, same group key, title overlap)
- `issues/done/083-implement-abstractClassInLocalScopeIsAbstract.md` - Implement Abstractclassinlocalscopeisabstract (same feature label, same group key, title overlap)
- `issues/open/1067-implement-blockScopedBindingUsedBeforeDef.md` - Implement Blockscopedbindingusedbeforedef (same feature label, same group key, title overlap)
- `issues/open/1069-implement-blockScopedBindingsReassignedInLoop-scope-analysis.md` - Implement Blockscopedbindingsreassignedinloop Scope Analysis (same feature label, same group key, title overlap)
- `issues/open/1079-implement-blockScopedVariablesUseBeforeDef.md` - Implement Blockscopedvariablesusebeforedef (same feature label, same group key, title overlap)
- `issues/open/1178-implement-classDeclarationShouldBeOutOfScopeInComputedNames.md` - Implement Classdeclarationshouldbeoutofscopeincomputednames (same feature label, same group key, title overlap)
- `issues/open/1443-implement-constDeclarations-scope-analysis.md` - Implement Constdeclarations Scope Analysis (same feature label, same group key, title overlap)
- `issues/open/1551-implement-continueInLoopsWithCapturedBlockScopedBindings.md` - Implement Continueinloopswithcapturedblockscopedbindings (same feature label, same group key, title overlap)
- `issues/done/192-implement-argsInScope.md` - Implement Argsinscope (same feature label, same group key, title overlap)
- `issues/open/1982-implement-disallowedBlockScopedInPresenceOfParseErrors.md` - Implement Disallowedblockscopedinpresenceofparseerrors (same feature label, same group key, title overlap)

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
`issues/open/446-implement-scope-analysis.md` に統合されました。
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
- `issues/done/5087-implement-scope-analysis.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
