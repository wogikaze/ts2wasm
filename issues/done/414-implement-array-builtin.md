---
id: 414
title: "Implement array-builtin support (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage array-builtin feature across 1755 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1755 cases fail with array-builtin diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: array-builtin feature has 1755 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
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
mise run reference-coverage -- test262 --limit 3510
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js
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

- `reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`
- `reference/test262/test/built-ins/Array/15.4.5-1.js`
- `reference/test262/test/built-ins/Array/15.4.5.1-5-1.js`
- `reference/test262/test/built-ins/Array/15.4.5.1-5-2.js`
- `reference/test262/test/built-ins/Array/S15.4.1_A1.1_T3.js`
- `reference/test262/test/built-ins/Array/S15.4.2.1_A1.1_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.2.1_A1.1_T3.js`
- `reference/test262/test/built-ins/Array/S15.4.2.1_A2.1_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.2.1_A2.2_T1.js`
- `reference/test262/test/built-ins/Array/S15.4.3_A1.1_T3.js`
- ... and 1745 more files

## Duplicate detection

- `issues/open/313-implement-array-builtin.md` - Implement array-builtin support (same feature label, same group key, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)
- `issues/done/406-direct-eval-annexb-existing-binding-residuals.md` - Direct eval Annex B existing binding residuals (same feature label, same group key)

## Smart triage

### Smart triage: Triage unknown unsupported: iterator method emulates undefined

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 812,
  "lines": 29,
  "extension": ".js",
  "first_code_line": "esid: sec-array.from",
  "test262_metadata": {
    "esid": "sec-array.from",
    "description": ">",
    "info": "|",
    "features": "[Symbol.iterator, IsHTMLDDA]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "UnsupportedTest262Metadata/test262-metadata: test262 feature `Symbol.iterator` is not supported by this runner slice",
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
    "kind": "binding",
    "name": "items",
    "line": 71,
    "column": 1,
    "initializer": "{}"
  }
]
```

Duplicate candidates:

```json
[]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Compiler dumps:

#### tokens

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `Symbol.iterator` is not supported by this runner slice
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `Symbol.iterator` is not supported by this runner slice
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `Symbol.iterator` is not supported by this runner slice
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
        "message": "File '/tmp/tmpmsgzpi_m/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  }
}
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/313-implement-array-builtin.md` に統合されました。
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
