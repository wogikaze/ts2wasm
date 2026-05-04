---
id: 5016
title: "Implement function resolution (dup)"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-04
---

## Summary

Triage function-resolution feature across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3 cases fail with function-resolution diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: function-resolution feature has 3 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js --detail
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
mise run reference-coverage -- test262 --limit 6
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js
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

- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js`
- `reference/test262/test/annexB/built-ins/escape/to-string-err-symbol.js`
- `reference/test262/test/annexB/built-ins/unescape/to-string-err-symbol.js`

## Duplicate detection

- `issues/open/1073-implement-blockScopedFunctionDeclarationInStrictClass.md` - Implement Blockscopedfunctiondeclarationinstrictclass (same feature label, same group key, title overlap)
- `issues/done/109-implement-addMoreCallSignaturesToBaseSignature.md` - Implement Addmorecallsignaturestobasesignature (same feature label, same group key, title overlap)
- `issues/open/1095-implement-callOnClass.md` - Implement Callonclass (same feature label, same group key, title overlap)
- `issues/done/132-implement-allowJsClassThisTypeCrash.md` - Implement Allowjsclassthistypecrash (same feature label, same group key, title overlap)
- `issues/open/2507-implement-genericCapturingFunctionNarrowing.md` - Implement Genericcapturingfunctionnarrowing (same feature label, same group key, title overlap)
- `issues/open/429-implement-eval.md` - Implement eval support (same feature label, same group key, title overlap)
- `issues/done/430-implement-function.md` - Implement function support (same feature label, same group key, title overlap)
- `issues/open/431-implement-function-resolution.md` - Implement function resolution (same feature label, same group key, title overlap)
- `issues/done/433-implement-legacy-global-builtin.md` - Implement legacy-global-builtin support (same reference path, title overlap)
- `issues/open/4442-implement-thisReferencedInFunctionInsideArrowFunction.md` - Implement Thisreferencedinfunctioninsidearrowfunction (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage unknown unsupported: this not object

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/prototype/compile/this-not-object.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1051,
  "lines": 42,
  "extension": ".js",
  "first_code_line": "esid: sec-regexp.prototype.compile",
  "test262_metadata": {
    "esid": "sec-regexp.prototype.compile",
    "es6id": "B.2.5.1",
    "description": "Behavior when \"this\" value is not an Object",
    "info": "|",
    "features": "[Symbol]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "UnsupportedTest262Metadata/test262-metadata: test262 feature `Symbol` is not supported by this runner slice",
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
    "name": "compile",
    "line": 58,
    "column": 1,
    "initializer": "RegExp.prototype.compile"
  },
  {
    "kind": "binding",
    "name": "symbol",
    "line": 59,
    "column": 1,
    "initializer": "Symbol('')"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/1189-implement-classExpressionWithStaticProperties-unknown-unsupported.md",
    "title": "Implement Classexpressionwithstaticproperties Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1191-implement-classExpressionWithStaticPropertiesES-unknown-unsupported.md",
    "title": "Implement Classexpressionwithstaticpropertieses Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1200-implement-classExtendsInterface-unknown-unsupported.md",
    "title": "Implement Classextendsinterface Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1432-implement-conflictMarkerDiff-unknown-unsupported.md",
    "title": "Implement Conflictmarkerdiff Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1434-implement-conflictMarkerTrivia-unknown-unsupported.md",
    "title": "Implement Conflictmarkertrivia Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1444-implement-constDeclarations-unknown-unsupported.md",
    "title": "Implement Constdeclarations Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1496-implement-contextualReturnTypeOfIIFE-unknown-unsupported.md",
    "title": "Implement Contextualreturntypeofiife Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1502-implement-contextualSignatureInstantiation-unknown-unsupported.md",
    "title": "Implement Contextualsignatureinstantiation Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1508-implement-contextualTypeBasedOnIntersectionWithAnyInTheMix-unknown-unsupported.md",
    "title": "Implement Contextualtypebasedonintersectionwithanyinthemix Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1520-implement-contextualTyping-unknown-unsupported.md",
    "title": "Implement Contextualtyping Unknown Unsupported",
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
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `Symbol` is not supported by this runner slice
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `Symbol` is not supported by this runner slice
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `Symbol` is not supported by this runner slice
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
        "message": "File '/tmp/tmpfrmdeq_y/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  }
}
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/431-implement-function-resolution.md` に統合されました。
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
