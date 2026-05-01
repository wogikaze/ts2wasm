---
id: 436
title: "Implement module-resolution support"
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

Triage module-resolution feature across 40 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 40 cases fail with module-resolution diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: module-resolution feature has 40 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js --detail
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
mise run reference-coverage -- test262 --limit 80
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js --detail
mise run reference-triage -- test262 reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js
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

- `reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-id-identifier-resolution-first.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-id-identifier-resolution-last.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-id-identifier-resolution-lone.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-id-identifier-resolution-middle.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-id-identifier-resolution-trlng.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-prop-identifier-resolution-first.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-prop-identifier-resolution-last.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-prop-identifier-resolution-lone.js`
- `reference/test262/test/language/expressions/assignment/dstr/obj-prop-identifier-resolution-middle.js`
- ... and 30 more files

## Duplicate detection

- `issues/open/133-implement-allowJsCrossMonorepoPackage.md` - Implement Allowjscrossmonorepopackage (same feature label, same group key, title overlap)
- `issues/open/169-implement-ambiguousOverloadResolution.md` - Implement Ambiguousoverloadresolution (same feature label, same group key, title overlap)
- `issues/done/409-implement-tsgo-declaration-emit-package-json-subpath.md` - Implement tsgo declaration emit: package-json exports and subpath reexport cases (same feature label, same group key, title overlap)
- `issues/done/410-implement-tsgo-declaration-emit-subpath-import-links.md` - Implement tsgo declaration emit: subpath import declaration emit cases (same feature label, same group key, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/399-define-typescript-parse-erase-emit-boundary.md` - Define TypeScript parse, erase, and emit boundary contract (same feature label, same group key)

## Smart triage

### Smart triage: Triage destructuring: ident name prop name literal package escaped

- Issue class: `triage-needed`
- Feature label: `destructuring`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/expressions/assignment/dstr/ident-name-prop-name-literal-package-escaped.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1113,
  "lines": 42,
  "extension": ".js",
  "first_code_line": "description: package is a valid identifier name, using escape (PropertyName of an ObjectAssignmentPattern)",
  "test262_metadata": {
    "description": "package is a valid identifier name, using escape (PropertyName of an ObjectAssignmentPattern)",
    "esid": "prod-AssignmentPattern",
    "features": "[destructuring-assignment]",
    "flags": "[generated, noStrict]",
    "info": "|",
    "AssignmentPattern": "",
    "ObjectAssignmentPattern": "",
    "AssignmentPropertyList": "",
    "AssignmentProperty": "",
    "PropertyName": "",
    "LiteralPropertyName": ""
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "UnsupportedTest262Metadata/test262-metadata: test262 feature `destructuring-assignment` is not supported by this runner slice",
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
    "name": "y",
    "line": 82,
    "column": 1,
    "initializer": "{ p\\u0061ckage: x } = { package: 42 }"
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
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `destructuring-assignment` is not supported by this runner slice
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `destructuring-assignment` is not supported by this runner slice
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `destructuring-assignment` is not supported by this runner slice
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
        "message": "File '/tmp/tmp3eba42zo/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
