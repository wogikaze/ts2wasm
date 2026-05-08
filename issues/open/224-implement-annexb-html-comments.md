---
id: 224
title: "Implement Annex B HTML-like comments"
type: feature
area: frontend
class: design-ready
priority: P3
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Implement Annex B HTML-like comment syntax used by legacy JavaScript source.

## Problem

The issue 060 limit-300 test262 classification window found 8 unsupported cases under `annexB/language/comments/`. These cases use `<!--` single-line HTML open comments and `-->` HTML close comment forms after multiline comments.

## Desired final state

Annex B HTML-like comments are accepted or rejected according to ECMAScript Annex B rules, and unsupported diagnostics for this family use the `html-comment` feature label until implemented.

## Scope

In scope:

- [x] Lex and parse Annex B `<!--` single-line HTML open comments.
- [x] Lex and parse Annex B `-->` HTML close comment forms where allowed.
- [x] Add regression fixtures for accepted and rejected HTML-like comment forms.
- [x] Preserve normal operator parsing for non-comment uses of `<`, `!`, and `-`.

Out of scope:

- [x] Implementing unrelated Annex B runtime semantics.
- [x] Broad parser-syntax cleanup outside the HTML-like comment family.

## Affected paths

Expected:

- `crates/frontend/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [x] test262 Annex B HTML-like comment cases in the classified window no longer report `html-comment`.
- [x] Regression fixtures cover `<!--`, `-->` after multiline comments, and ordinary expression/operator cases that must not become comments.
- [x] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --limit 300
```

Not run:

- none

## Notes

Created from issue 060 classification evidence on 2026-04-28.

Reference-backed affected files in the limit-300 window:

- `reference/test262/test/annexB/language/comments/multi-line-html-close.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-asi.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-first-line-1.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-first-line-2.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-first-line-3.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-unicode-separators.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close.js`
- `reference/test262/test/annexB/language/comments/single-line-html-open.js`

## Completion evidence

Commits:

- branch commit: `issue-224: implement Annex B HTML-like comments`

Validation result:

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-28

command: cargo nextest run -p ts2wasm-frontend html --no-tests warn
result: passed; 8 tests passed
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli html --no-tests warn
result: passed; 2 tests passed
date: 2026-04-28

command: TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/comments --detail
result: passed; executed=8, unsupported_features=name-resolution:8, html-comment:0
date: 2026-04-28

command: TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --limit 300
result: passed; unsupported_features=name-resolution:101,string-builtin:63,eval:51,regexp-literal:47,legacy-global-builtin:20,date:16,array-builtin:1,builtin-api:1; html-comment:0
date: 2026-04-28

command: mise run update-coverage-matrix -- --check
result: passed; coverage matrix OK (up to date)
date: 2026-04-28

command: mise run update-issue-index -- --check
result: passed; issues/index.md OK (up to date)
date: 2026-04-28

command: mise run check-issue-health
result: passed; issues/index.md queue OK, check_issue_health: OK
date: 2026-04-28

command: cargo nextest run
result: passed; 265 tests passed, 4 skipped
date: 2026-04-28
```

Remaining risks:

- The listed test262 files now parse through the Annex B comment forms and fail later on existing `UnresolvedName` support for test262 runtime constructors such as `Test262Error` and `EvalError`. This is outside issue 224.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/224-implement-annexb-html-comments.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
