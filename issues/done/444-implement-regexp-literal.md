---
id: 444
title: "Implement RegExp literal support"
type: spike
area: runtime/builtins
class: blocked
priority: P1
depends_on: [5004]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage regexp-literal feature across 1221 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1221 cases fail with regexp-literal diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: regexp-literal feature has 1221 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js --detail
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

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- parser/resolver code unless `reference-triage` proves the failure happens before runtime lowering

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
mise run reference-coverage -- test262 --limit 2442
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js
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
- `reference/test262/test/annexB/built-ins/RegExp/incomplete_hex_unicode_escape.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/index/prop-desc.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/index/this-cross-realm-constructor.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/index/this-not-regexp-constructor.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/index/this-subclass-constructor.js`
- ... and 1211 more files

## Duplicate detection

- `issues/open/066-implement-regexp-literal.md` - Implement RegExp literal support (same reference path, same feature label, same group key, title overlap)
- `issues/done/005-add-fine-grained-unsupported-feature-breakdown.md` - issues/done/005-add-fine-grained-unsupported-feature-breakdown.md (same feature label, same group key)
- `issues/done/009-select-first-coverage-improvement-feature-slice.md` - issues/done/009-select-first-coverage-improvement-feature-slice.md (same feature label, same group key)
- `issues/done/022-expand-test262-differential-coverage.md` - issues/done/022-expand-test262-differential-coverage.md (same feature label, same group key)
- `issues/done/051-implement-regexp.md` - Implement RegExp (same feature label, same group key, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/202-implement-regexp-literal-support.md` - issues/done/202-implement-regexp-literal-support.md (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same feature label, same group key, title overlap)
- `issues/done/406-direct-eval-annexb-existing-binding-residuals.md` - Direct eval Annex B existing binding residuals (same feature label, same group key)

## Smart triage

### Smart triage: Triage async: RegExp control escape russian letter

- Issue class: `triage-needed`
- Feature label: `async`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1597,
  "lines": 53,
  "extension": ".js",
  "first_code_line": "info: \"CharacterEscape :: c ControlLetter\"",
  "test262_metadata": {
    "info": "\"CharacterEscape :: c ControlLetter\"",
    "es5id": "15.10.2.10_A2.1_T3",
    "es6id": "B.1.4",
    "description": ">",
    "\"ControlLetter": ": RUSSIAN ALPHABET is incorrect\"",
    "features": "[generators]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "UnsupportedTest262Metadata/test262-metadata: test262 feature `generators` is not supported by this runner slice",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "async",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text

function print(message) {
  console.log(message);
}


/* standard globals shim */
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "print",
    "line": 2,
    "column": 1,
    "params": "message"
  },
  {
    "kind": "binding",
    "name": "NaN",
    "line": 10,
    "column": 1,
    "initializer": "0/0"
  },
  {
    "kind": "binding",
    "name": "Infinity",
    "line": 11,
    "column": 1,
    "initializer": "1/0"
  },
  {
    "kind": "binding",
    "name": "$262",
    "line": 17,
    "column": 1,
    "initializer": "{}"
  },
  {
    "kind": "function",
    "name": "$ERROR",
    "line": 26,
    "column": 1,
    "params": "message"
  },
  {
    "kind": "function",
    "name": "$DONOTEVALUATE",
    "line": 30,
    "column": 1,
    "params": ""
  },
  {
    "kind": "function",
    "name": "assert",
    "line": 34,
    "column": 1,
    "params": "mustBeTrue, message"
  },
  {
    "kind": "binding",
    "name": "alpha",
    "line": 59,
    "column": 8,
    "initializer": "0x0410"
  },
  {
    "kind": "binding",
    "name": "letter",
    "line": 71,
    "column": 5,
    "initializer": "String.fromCharCode(alpha)"
  },
  {
    "kind": "binding",
    "name": "letter",
    "line": 81,
    "column": 6
  },
  {
    "kind": "binding",
    "name": "source",
    "line": 82,
    "column": 3,
    "initializer": "\"\\\\c\" + letter"
  },
  {
    "kind": "binding",
    "name": "re",
    "line": 83,
    "column": 3,
    "initializer": "new RegExp(source)"
  },
  {
    "kind": "binding",
    "name": "char",
    "line": 86,
    "column": 5,
    "initializer": "letter.charCodeAt(0)"
  },
  {
    "kind": "binding",
    "name": "str",
    "line": 87,
    "column": 5,
    "initializer": "String.fromCharCode(char % 32)"
  },
  {
    "kind": "binding",
    "name": "arr",
    "line": 88,
    "column": 5,
    "initializer": "re.exec(str)"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/066-implement-regexp-literal.md",
    "title": "Implement RegExp literal support",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/416-implement-async.md",
    "title": "Implement async/await support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/230-implement-async-iteration-for-await-of.md",
    "title": "Implement async iteration and for-await-of",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/284-support-test262-async-flag-runner-coverage.md",
    "title": "Support test262 async flag in reference coverage",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Compiler dumps:

#### tokens

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `generators` is not supported by this runner slice
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `generators` is not supported by this runner slice
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `generators` is not supported by this runner slice
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": false,
    "diagnostics": [
      {
        "code": 6504,
        "category": "Error",
        "message": "File '/tmp/tmp9ryvxgjj/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  }
}
```

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

## Close note

Closed as duplicate of `issues/open/066-implement-regexp-literal.md`. All work tracked under issue 066.

superseded-by: 066
