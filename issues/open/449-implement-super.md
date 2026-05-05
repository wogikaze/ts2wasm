---
id: 449
title: "Implement super keyword"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage super feature across 16 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 16 cases fail with super diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: super feature has 16 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/eval-code/direct/super-call-arrow.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/eval-code/direct/super-call-arrow.js --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 32
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/eval-code/direct/super-call-arrow.js --detail
mise run reference-triage -- test262 reference/test262/test/language/eval-code/direct/super-call-arrow.js
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/test262/test/language/eval-code/direct/super-call-arrow.js`
- `reference/test262/test/language/eval-code/direct/super-call-fn.js`
- `reference/test262/test/language/eval-code/direct/super-call-method.js`
- `reference/test262/test/language/eval-code/direct/super-call.js`
- `reference/test262/test/language/eval-code/direct/super-prop-arrow.js`
- `reference/test262/test/language/eval-code/direct/super-prop-dot-no-home.js`
- `reference/test262/test/language/eval-code/direct/super-prop-expr-no-home-no-eval.js`
- `reference/test262/test/language/eval-code/direct/super-prop-expr-no-home.js`
- `reference/test262/test/language/eval-code/direct/super-prop-method.js`
- `reference/test262/test/language/eval-code/direct/super-prop.js`
- ... and 6 more files

## Duplicate detection

- `issues/open/050-implement-date.md` - Implement Date (same feature label, same group key, title overlap)
- `issues/open/052-implement-json.md` - Implement JSON (same feature label, same group key, title overlap)
- `issues/open/052d-implement-json-stringify-broader-replacer-semantics.md` - Implement broader JSON.stringify replacer semantics (same feature label, same group key, title overlap)
- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, same group key, title overlap)
- `issues/done/066-implement-regexp-literal.md` - Implement RegExp literal support (same feature label, same group key, title overlap)
- `issues/done/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/done/068-implement-unsupported-expression.md` - Implement unsupported expression types (same feature label, same group key, title overlap)
- `issues/done/069-implement-APILibCheck.md` - Implement Apilibcheck (same feature label, same group key, title overlap)
- `issues/open/070-implement-APISample.md` - Implement Apisample (same feature label, same group key, title overlap)
- `issues/done/071-implement-ArrowFunctionExpression.md` - Implement Arrowfunctionexpression (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage super: super call arrow

- Issue class: `triage-needed`
- Feature label: `super`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/language/eval-code/direct/super-call-arrow.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/eval-code/direct/super-call-arrow.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 790,
  "lines": 27,
  "extension": ".js",
  "first_code_line": "esid: sec-scripts-static-semantics-early-errors",
  "test262_metadata": {
    "esid": "sec-scripts-static-semantics-early-errors",
    "es6id": "15.1.1",
    "description": ">",
    "info": "|",
    "features": "[super, arrow-function]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "UnsupportedTest262Metadata/test262-metadata: test262 feature `super` is not supported by this runner slice",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "super",
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
    "name": "caught",
    "line": 60,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "f",
    "line": 61,
    "column": 1,
    "initializer": "() => eval('super()"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/312-triage-test262-blocked-p0-window.md",
    "title": "Triage test262 blocked P0 window",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/415-implement-arrow-function.md",
    "title": "Implement arrow functions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/420-implement-call-expression.md",
    "title": "Implement call expression support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/435-implement-method-call.md",
    "title": "Implement method call support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/047-implement-super-keyword.md",
    "title": "Implement super keyword",
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
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `super` is not supported by this runner slice
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `super` is not supported by this runner slice
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `super` is not supported by this runner slice
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
        "message": "File '/tmp/tmpicp8zkwd/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
