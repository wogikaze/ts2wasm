---
id: 417
title: "Implement async-iteration support"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage async-iteration feature across 1141 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1141 cases fail with async-iteration diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: async-iteration feature has 1141 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js --detail
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
mise run reference-coverage -- test262 --limit 2282
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js
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

- `reference/test262/test/annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-assignment.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-evaluation.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-arrow.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-class.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-cover.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-fn.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-fn-name-gen.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-in.js`
- `reference/test262/test/language/statements/for-await-of/async-func-decl-dstr-array-elem-init-order.js`
- ... and 1131 more files

## Duplicate detection

- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/230-implement-async-iteration-for-await-of.md` - Implement async iteration and for-await-of (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage unknown unsupported: iterator close return emulates undefined throws when called

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1309,
  "lines": 39,
  "extension": ".js",
  "first_code_line": "esid: sec-getiterator",
  "test262_metadata": {
    "esid": "sec-getiterator",
    "description": ">",
    "features": "[async-iteration, IsHTMLDDA]",
    "flags": "[async]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "UnsupportedTest262Metadata/test262-metadata: test262 feature `async-iteration` is not supported by this runner slice",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "unknown-unsupported",
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
    "kind": "function",
    "name": "f",
    "line": 61,
    "column": 7,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "IsHTMLDDA",
    "line": 62,
    "column": 3,
    "initializer": "$262.IsHTMLDDA"
  },
  {
    "kind": "binding",
    "name": "iter",
    "line": 63,
    "column": 3,
    "initializer": "{"
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 73,
    "column": 14
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "done",
    "path": "issues/done/060a-close-unknown-unsupported-fixed-window-spike.md",
    "title": "Close unknown-unsupported fixed-window spike",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/230-implement-async-iteration-for-await-of.md",
    "title": "Implement async iteration and for-await-of",
    "reason": "same reference path, same feature label"
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
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `async-iteration` is not supported by this runner slice
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `async-iteration` is not supported by this runner slice
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `async-iteration` is not supported by this runner slice
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
        "message": "File '/tmp/tmpalaivtzj/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
