---
id: 416
title: "Implement async/await support"
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

Triage async feature across 2054 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2054 cases fail with async diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: async feature has 2054 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/yield/star-iterable-return-emulates-undefined-throws-when-called.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/yield/star-iterable-return-emulates-undefined-throws-when-called.js --detail
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
mise run reference-coverage -- test262 --limit 4108
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/yield/star-iterable-return-emulates-undefined-throws-when-called.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/yield/star-iterable-return-emulates-undefined-throws-when-called.js
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

- `reference/test262/test/annexB/language/expressions/yield/star-iterable-return-emulates-undefined-throws-when-called.js`
- `reference/test262/test/annexB/language/expressions/yield/star-iterable-throw-emulates-undefined-throws-when-called.js`
- `reference/test262/test/annexB/language/statements/for-of/iterator-close-return-emulates-undefined-throws-when-called.js`
- `reference/test262/test/language/arguments-object/async-gen-meth-args-trailing-comma-multiple.js`
- `reference/test262/test/language/arguments-object/async-gen-meth-args-trailing-comma-null.js`
- `reference/test262/test/language/arguments-object/async-gen-meth-args-trailing-comma-single-args.js`
- `reference/test262/test/language/arguments-object/async-gen-meth-args-trailing-comma-spread-operator.js`
- `reference/test262/test/language/arguments-object/async-gen-meth-args-trailing-comma-undefined.js`
- `reference/test262/test/language/arguments-object/async-gen-named-func-expr-args-trailing-comma-multiple.js`
- `reference/test262/test/language/arguments-object/async-gen-named-func-expr-args-trailing-comma-null.js`
- ... and 2044 more files

## Duplicate detection

- `issues/done/005-add-fine-grained-unsupported-feature-breakdown.md` - issues/done/005-add-fine-grained-unsupported-feature-breakdown.md (same feature label, same group key)
- `issues/open/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/062-implement-function.md` - Implement function support (same feature label, same group key, title overlap)
- `issues/done/062c-ordinary-function-declarations-and-calls.md` - Implement ordinary function declarations and direct calls (same feature label, same group key, title overlap)
- `issues/done/062e-function-closures.md` - Implement function closures (same feature label, same group key, title overlap)
- `issues/done/062g-heap-closure-object-abi-and-rooting.md` - Define and implement heap closure object ABI and rooting (same feature label, same group key, title overlap)
- `issues/done/230-implement-async-iteration-for-await-of.md` - Implement async iteration and for-await-of (same feature label, same group key, title overlap)
- `issues/done/249-implement-class-static-block-parser.md` - Implement class static block parser support (same feature label, same group key, title overlap)
- `issues/done/256-lower-returned-immutable-closures-to-heap-values.md` - Lower returned immutable closures to heap closure values (same feature label, same group key)
- `issues/done/257-emit-heap-closure-allocation-and-dispatch.md` - Emit heap closure allocation and dispatch (same feature label, same group key)

## Smart triage

### Smart triage: Triage async: star iterable return emulates undefined throws when called

- Issue class: `triage-needed`
- Feature label: `async`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/language/expressions/yield/star-iterable-return-emulates-undefined-throws-when-called.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/yield/star-iterable-return-emulates-undefined-throws-when-called.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1055,
  "lines": 31,
  "extension": ".js",
  "first_code_line": "esid: sec-generator-function-definitions-runtime-semantics-evaluation",
  "test262_metadata": {
    "esid": "sec-generator-function-definitions-runtime-semantics-evaluation",
    "description": ">",
    "features": "[generators, IsHTMLDDA]"
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
    "name": "IsHTMLDDA",
    "line": 60,
    "column": 1,
    "initializer": "$262.IsHTMLDDA"
  },
  {
    "kind": "binding",
    "name": "iter",
    "line": 61,
    "column": 1,
    "initializer": "{"
  },
  {
    "kind": "binding",
    "name": "outer",
    "line": 67,
    "column": 1,
    "initializer": "(function*() { yield* iter"
  },
  {
    "kind": "binding",
    "name": "emptyString",
    "line": 76,
    "column": 3,
    "initializer": "\"\""
  }
]
```

Duplicate candidates:

```json
[
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
        "message": "File '/tmp/tmphffaj5wn/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
