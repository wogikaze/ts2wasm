---
id: 5086
title: "Implement RegExp literal support (dup)"
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

Triage regexp-literal feature across 875 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 875 cases fail with regexp-literal diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: regexp-literal feature has 875 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/lastMatch/this-not-regexp-constructor.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/lastMatch/this-not-regexp-constructor.js --detail
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
mise run reference-coverage -- test262 --limit 1750
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/lastMatch/this-not-regexp-constructor.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/lastMatch/this-not-regexp-constructor.js
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

- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/lastMatch/this-not-regexp-constructor.js`
- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/pattern-string.js`
- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/pattern-string-u.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/lastMatch/prop-desc.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/input/this-not-regexp-constructor.js`
- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/pattern-regexp-same.js`
- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/pattern-regexp-props.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/input/prop-desc.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/index/this-subclass-constructor.js`
- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/pattern-regexp-distinct.js`
- ... and 865 more files

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same reference path, title overlap)
- `issues/open/066-implement-regexp-literal.md` - Implement RegExp literal support (same reference path, same feature label, same group key, title overlap)
- `issues/open/1139-implement-checkJsxNotSetError.md` - Implement Checkjsxnotseterror (same feature label, same group key, title overlap)
- `issues/open/2230-implement-excessiveStackDepthFlatArray.md` - Implement Excessivestackdepthflatarray (same feature label, same group key, title overlap)
- `issues/open/2872-implement-initializedDestructuringAssignmentTypes.md` - Implement Initializeddestructuringassignmenttypes (same feature label, same group key, title overlap)
- `issues/open/3097-implement-jsFileCompilationTypeArgumentSyntaxOfCall.md` - Implement Jsfilecompilationtypeargumentsyntaxofcall (same feature label, same group key, title overlap)
- `issues/open/3125-implement-jsxEmitWithAttributes.md` - Implement Jsxemitwithattributes (same feature label, same group key, title overlap)
- `issues/open/3126-implement-jsxFactoryAndReactNamespace.md` - Implement Jsxfactoryandreactnamespace (same feature label, same group key, title overlap)
- `issues/open/3127-implement-jsxFactoryIdentifier.md` - Implement Jsxfactoryidentifier (same feature label, same group key, title overlap)
- `issues/open/3130-implement-jsxFactoryMissingErrorInsideAClass.md` - Implement Jsxfactorymissingerrorinsideaclass (same feature label, same group key, title overlap)

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
`issues/open/5020-implement-regexp-literal.md` に統合されました。
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
- `issues/open/5086-implement-regexp-literal.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
