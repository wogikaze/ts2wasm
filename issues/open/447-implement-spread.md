---
id: 447
title: "Implement spread operator"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage spread feature across 70 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 70 cases fail with spread diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: spread feature has 70 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/expressions/array/spread-err-mult-err-obj-unresolvable.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/array/spread-err-mult-err-obj-unresolvable.js --detail
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
mise run reference-coverage -- test262 --limit 140
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/array/spread-err-mult-err-obj-unresolvable.js --detail
mise run reference-triage -- test262 reference/test262/test/language/expressions/array/spread-err-mult-err-obj-unresolvable.js
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

- `reference/test262/test/language/expressions/array/spread-err-mult-err-obj-unresolvable.js`
- `reference/test262/test/language/expressions/array/spread-err-sngl-err-obj-unresolvable.js`
- `reference/test262/test/language/expressions/array/spread-mult-obj-null.js`
- `reference/test262/test/language/expressions/array/spread-mult-obj-undefined.js`
- `reference/test262/test/language/expressions/array/spread-obj-getter-init.js`
- `reference/test262/test/language/expressions/array/spread-obj-manipulate-outter-obj-in-getter.js`
- `reference/test262/test/language/expressions/array/spread-obj-mult-spread-getter.js`
- `reference/test262/test/language/expressions/array/spread-obj-mult-spread.js`
- `reference/test262/test/language/expressions/array/spread-obj-null.js`
- `reference/test262/test/language/expressions/array/spread-obj-overrides-prev-properties.js`
- ... and 60 more files

## Duplicate detection

- `issues/open/193-implement-arguments.md` - Implement Arguments (same feature label, same group key, title overlap)
- `issues/open/274-implement-spread-operator.md` - Implement spread operator (same feature label, same group key, title overlap)
- `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md` - Support ABC451 D original submission without source rewrite (same feature label, same group key)
- `issues/open/300-support-abc451-large-integer-number-boundary.md` - Support ABC451 large integer number boundary (same feature label, same group key)
- `issues/open/309-reduce-abc451-depth9-live-allocation-shape.md` - Reduce ABC451 depth-9 live allocation shape (same feature label, same group key)
- `issues/open/311-fix-test262-arguments-object-index-assignment.md` - Fix test262 arguments object index assignment semantics (same feature label, same group key)
- `issues/open/353-spread-iterator-protocol.md` - Implement iterator protocol integration for spread operator (same feature label, same group key, title overlap)
- `issues/open/357-fix-abc451-depth8-iwasm-timeout.md` - Fix ABC451 depth-8 iwasm timeout (same feature label, same group key)
- `issues/open/363-reduce-abc451-allocation-and-sweep-volume-after-bulk-copy-narrowing.md` - Reduce ABC451 allocation and sweep volume after bulk copy narrowing (same feature label, same group key)
- `issues/open/365-reduce-abc451-array-growth-allocation-copy-pressure.md` - Reduce ABC451 array-growth allocation and copy pressure (same feature label, same group key)

## Smart triage

### Smart triage: Triage spread: spread err mult err obj unresolvable

- Issue class: `triage-needed`
- Feature label: `spread`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/language/expressions/array/spread-err-mult-err-obj-unresolvable.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/expressions/array/spread-err-mult-err-obj-unresolvable.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1383,
  "lines": 38,
  "extension": ".js",
  "first_code_line": "description: Object Spread operator results in error when using an unresolvable reference (Array initializer)",
  "test262_metadata": {
    "description": "Object Spread operator results in error when using an unresolvable reference (Array initializer)",
    "esid": "sec-runtime-semantics-arrayaccumulation",
    "features": "[object-spread]",
    "flags": "[generated]",
    "info": "|",
    "SpreadElement": "...AssignmentExpression",
    "e. Assert": "status is true.",
    "Pending Runtime Semantics": "PropertyDefinitionEvaluation",
    "PropertyDefinition": "...AssignmentExpression"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "UnsupportedTest262Metadata/test262-metadata: test262 feature `object-spread` is not supported by this runner slice",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "spread",
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
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/274-implement-spread-operator.md",
    "title": "Implement spread operator",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/353-spread-iterator-protocol.md",
    "title": "Implement iterator protocol integration for spread operator",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/407-map-spread-key-preserving-iterator-storage.md",
    "title": "Implement key-preserving Map entry storage for spread iteration",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/039-implement-spread-arguments.md",
    "title": "Implement spread arguments",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/354-sparse-array-spread-support.md",
    "title": "Implement sparse array spread support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/355-dynamic-object-enumeration-spread.md",
    "title": "Implement dynamic object property enumeration spread",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/401-generator-function-syntax-prerequisite-for-iterator-spread.md",
    "title": "Implement generator function syntax prerequisite for iterator spread",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/402-computed-symbol-iterator-prerequisite-for-spread.md",
    "title": "Implement computed Symbol.iterator prerequisite for spread",
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
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `object-spread` is not supported by this runner slice
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `object-spread` is not supported by this runner slice
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `object-spread` is not supported by this runner slice
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
        "message": "File '/tmp/tmpin3mebph/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  }
}
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/274-implement-spread-operator.md` に統合されました。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/447-implement-spread.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
