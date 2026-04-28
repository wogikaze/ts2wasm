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
---

## Summary

Implement Annex B HTML-like comment syntax used by legacy JavaScript source.

## Problem

The issue 060 limit-300 test262 classification window found 8 unsupported cases under `annexB/language/comments/`. These cases use `<!--` single-line HTML open comments and `-->` HTML close comment forms after multiline comments.

## Desired final state

Annex B HTML-like comments are accepted or rejected according to ECMAScript Annex B rules, and unsupported diagnostics for this family use the `html-comment` feature label until implemented.

## Scope

In scope:

- [ ] Lex and parse Annex B `<!--` single-line HTML open comments.
- [ ] Lex and parse Annex B `-->` HTML close comment forms where allowed.
- [ ] Add regression fixtures for accepted and rejected HTML-like comment forms.
- [ ] Preserve normal operator parsing for non-comment uses of `<`, `!`, and `-`.

Out of scope:

- [ ] Implementing unrelated Annex B runtime semantics.
- [ ] Broad parser-syntax cleanup outside the HTML-like comment family.

## Affected paths

Expected:

- `crates/frontend/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] test262 Annex B HTML-like comment cases in the classified window no longer report `html-comment`.
- [ ] Regression fixtures cover `<!--`, `-->` after multiline comments, and ordinary expression/operator cases that must not become comments.
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 300
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
