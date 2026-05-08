---
id: 453
title: "Implement type-system support"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5002]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage type-system feature across 8 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 8 cases fail with type-system diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: type-system feature has 8 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/expressions/conditional/coalesce-expr-ternary.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/conditional/coalesce-expr-ternary.js --detail
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
mise run reference-coverage -- test262 --limit 16
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/conditional/coalesce-expr-ternary.js --detail
mise run reference-triage -- test262 reference/test262/test/language/expressions/conditional/coalesce-expr-ternary.js
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

- `reference/test262/test/language/expressions/conditional/coalesce-expr-ternary.js`
- `reference/test262/test/language/expressions/conditional/in-branch-1.js`
- `reference/test262/test/language/expressions/conditional/symbol-conditional-evaluation.js`
- `reference/test262/test/language/expressions/conditional/tco-cond.js`
- `reference/test262/test/language/expressions/conditional/tco-pos.js`
- `reference/test262/test/language/statements/for-of/generic-iterable.js`
- `reference/test262/test/staging/sm/Date/toString-generic.js`
- `reference/test262/test/staging/sm/String/normalize-generic.js`

## Duplicate detection

- `issues/done/187-implement-anyInferenceAnonymousFunctions.md` - Implement Anyinferenceanonymousfunctions (same feature label, same group key, title overlap)
- `issues/done/345-implement-tsc-type-alias-coverage.md` - Implement TypeScript type alias coverage for tsc suite (23 cases) (same feature label, same group key, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/399-define-typescript-parse-erase-emit-boundary.md` - Define TypeScript parse, erase, and emit boundary contract (same feature label, same group key)

## Smart triage

### Smart triage: Triage unknown unsupported: coalesce expr ternary

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/language/expressions/conditional/coalesce-expr-ternary.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/expressions/conditional/coalesce-expr-ternary.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1802,
  "lines": 73,
  "extension": ".js",
  "first_code_line": "description: >",
  "test262_metadata": {
    "description": ">",
    "ShortCircuitExpression in the Conditional Expression (?": ")",
    "esid": "sec-conditional-operator",
    "info": "|",
    "ShortCircuitExpression": "",
    "CoalesceExpression": "",
    "CoalesceExpressionHead": "",
    "ConditionalExpression": "",
    "ShortCircuitExpression ? AssignmentExpression": "AssignmentExpression",
    "features": "[coalesce-expression]"
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
    "name": "x",
    "line": 69,
    "column": 1
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
        "message": "File '/tmp/tmp970ni_d0/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
