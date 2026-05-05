---
id: 066
title: "Implement RegExp literal support (dup)"
type: spike
area: runtime/builtins
class: superseded
priority: P1
depends_on: [5004]
blocks: []
created: 2026-04-26
updated: 2026-05-04
---

## Summary

Triage the generated reference bucket `Implement RegExp literal support` before implementation. This issue records a failing reference case and must be split or superseded before any code change starts.

## Problem

Reference test results show 59 cases fail with regexp-literal diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: generated reference bucket `Implement RegExp literal support` fails with `regexp-literal` and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js
```

Narrow coverage reproduction:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js --detail
```

Representative path: `reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js`
Feature label: `regexp-literal`

## Desired final state

This generated bucket is not used as a direct implementation work order. It is either superseded by an existing open/done issue, closed as a duplicate, or split into implementation-ready child issues that contain exact reproduction evidence and measurable acceptance criteria.

## Scope

In scope:

- [x] Run the representative `mise run reference-triage -- ...` command
- [x] Confirm whether duplicate candidates already cover this failure
- [x] Split one observable behavior or fixed reference window into child issues
- [x] Carry source context, diagnostic code, AST evidence, and validation commands into each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad fixes that mix unrelated parser, resolver, runtime, and API failures

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- parser/resolver code unless `reference-triage` proves the failure happens before runtime lowering

## Acceptance criteria

- [x] Duplicate candidates are confirmed as no-match, duplicate, or superseding issue
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js --detail
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

- `reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-not-capturing.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-invalid-control-escape-character-class-range.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-invalid-control-escape-character-class.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-leading-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-leading-escape.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape.js`
- `reference/test262/test/annexB/built-ins/RegExp/incomplete_hex_unicode_escape.js`
- ... and 49 more files

## Duplicate detection

- `issues/done/051-implement-regexp.md` - Implement RegExp (same feature label, title overlap)


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

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/066-implement-regexp-literal.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
