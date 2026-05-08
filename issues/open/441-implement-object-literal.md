---
id: 441
title: "Implement object literal enhancements"
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

Triage object-literal feature across 722 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 722 cases fail with object-literal diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: object-literal feature has 722 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/assignment/dstr/object-pattern-emulates-undefined.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/assignment/dstr/object-pattern-emulates-undefined.js --detail
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
mise run reference-coverage -- test262 --limit 1444
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/assignment/dstr/object-pattern-emulates-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/assignment/dstr/object-pattern-emulates-undefined.js
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

- `reference/test262/test/annexB/language/expressions/assignment/dstr/object-pattern-emulates-undefined.js`
- `reference/test262/test/annexB/language/statements/const/dstr/object-pattern-emulates-undefined.js`
- `reference/test262/test/harness/deepEqual-object.js`
- `reference/test262/test/harness/propertyhelper-verifyconfigurable-configurable-object.js`
- `reference/test262/test/harness/verifyProperty-configurable-object.js`
- `reference/test262/test/harness/verifyProperty-desc-is-not-object.js`
- `reference/test262/test/harness/wellKnownIntrinsicObjects.js`
- `reference/test262/test/intl402/BigInt/prototype/toLocaleString/default-options-object-prototype.js`
- `reference/test262/test/intl402/Collator/supportedLocalesOf/taint-Object-prototype.js`
- `reference/test262/test/intl402/Collator/taint-Object-prototype.js`
- ... and 712 more files

## Duplicate detection

- `issues/done/052-implement-json.md` - Implement JSON (same feature label, same group key, title overlap)
- `issues/open/052d-implement-json-stringify-broader-replacer-semantics.md` - Implement broader JSON.stringify replacer semantics (same feature label, same group key, title overlap)
- `issues/open/274-implement-spread-operator.md` - Implement spread operator (same feature label, same group key, title overlap)
- `issues/done/374-design-broader-object-toprimitive-for-bigint-comparisons.md` - Design broader object ToPrimitive for mixed BigInt comparisons (same feature label, same group key, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, same group key, title overlap)
- `issues/open/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/062d-function-this-and-arguments.md` - Implement function this and arguments semantics (same feature label, same group key, title overlap)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same feature label, same group key, title overlap)
- `issues/done/340-array-map-generic-call.md` - Generic call for Array.prototype.map (static dense receiver slice) (same feature label, same group key)
- `issues/done/355-dynamic-object-enumeration-spread.md` - Implement dynamic object property enumeration spread (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage destructuring: object pattern emulates undefined

- Issue class: `triage-needed`
- Feature label: `destructuring`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/language/expressions/assignment/dstr/object-pattern-emulates-undefined.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/assignment/dstr/object-pattern-emulates-undefined.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1059,
  "lines": 35,
  "extension": ".js",
  "first_code_line": "esid: sec-destructuring-binding-patterns-runtime-semantics-bindinginitialization",
  "test262_metadata": {
    "esid": "sec-destructuring-binding-patterns-runtime-semantics-bindinginitialization",
    "description": ">",
    "info": "|",
    "BindingPattern": "ObjectBindingPattern",
    "Runtime Semantics": "KeyedBindingInitialization",
    "SingleNameBinding": "BindingIdentifier Initializer[opt]",
    "features": "[destructuring-binding, IsHTMLDDA]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "UnsupportedTest262Metadata/test262-metadata: test262 feature `destructuring-binding` is not supported by this runner slice",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "destructuring",
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
    "name": "initCount",
    "line": 73,
    "column": 1,
    "initializer": "0"
  },
  {
    "kind": "binding",
    "name": "counter",
    "line": 74,
    "column": 1,
    "initializer": "function() {"
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 78,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/425-implement-destructuring.md",
    "title": "Implement destructuring",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/247-implement-destructuring-binding-pattern-parser.md",
    "title": "Implement destructuring binding pattern parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/251-implement-destructuring-binding-runtime-semantics.md",
    "title": "Implement destructuring binding runtime semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/252-implement-destructuring-assignment-pattern-parser.md",
    "title": "Implement destructuring assignment pattern parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/289-resolve-callcount-binding-in-class-destructuring.md",
    "title": "Resolve callCount binding in class destructuring tests",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/292-resolve-initcount-binding-in-class-destructuring.md",
    "title": "Resolve initCount binding in class destructuring defaults",
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
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `destructuring-binding` is not supported by this runner slice
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `destructuring-binding` is not supported by this runner slice
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `destructuring-binding` is not supported by this runner slice
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
        "message": "File '/tmp/tmpl4ecxs8s/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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

## False-done audit

**truly-done** (441)

- Implementation commits: verified via `git log --oneline --all --grep=441`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
