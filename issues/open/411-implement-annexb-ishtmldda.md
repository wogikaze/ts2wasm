---
id: 411
title: "Implement annexb-ishtmldda support"
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

Triage annexb-ishtmldda feature across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail with annexb-ishtmldda diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: annexb-ishtmldda feature has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js --detail
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
mise run reference-coverage -- test262 --limit 8
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js
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

- `reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js`
- `reference/test262/test/annexB/language/expressions/logical-assignment/emulates-undefined-and.js`
- `reference/test262/test/annexB/language/expressions/logical-assignment/emulates-undefined-coalesce.js`
- `reference/test262/test/annexB/language/expressions/logical-assignment/emulates-undefined-or.js`

## Duplicate detection

- `issues/open/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same reference path, same feature label, same group key, title overlap)
- `issues/done/237-implement-annexb-ishtmldda-compatibility.md` - Implement Annex B IsHTMLDDA compatibility (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage unknown unsupported: emulates undefined

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 681,
  "lines": 20,
  "extension": ".js",
  "first_code_line": "esid: sec-binary-bitwise-operators-runtime-semantics-evaluation",
  "test262_metadata": {
    "esid": "sec-binary-bitwise-operators-runtime-semantics-evaluation",
    "description": ">",
    "info": "|",
    "CoalesceExpression": "CoalesceExpressionHead ?? BitwiseORExpression",
    "features": "[IsHTMLDDA, coalesce-expression]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "UnsupportedTest262Metadata/test262-metadata: test262 feature `coalesce-expression` is not supported by this runner slice",
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
    "name": "IsHTMLDDA",
    "line": 65,
    "column": 1,
    "initializer": "$262.IsHTMLDDA"
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
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `coalesce-expression` is not supported by this runner slice
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `coalesce-expression` is not supported by this runner slice
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `coalesce-expression` is not supported by this runner slice
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
        "message": "File '/tmp/tmpylu1y6v9/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
