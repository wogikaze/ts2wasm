---
id: 5019
title: "Implement name resolution (dup)"
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

Triage name-resolution feature across 124 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 124 cases fail with name-resolution diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: name-resolution feature has 124 reference failures and needs smart-triage evidence before implementation starts.

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
mise run reference-coverage -- test262 --limit 248
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js
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

- `reference/test262/test/annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/this-not-date.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-not-date.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/year-to-number-err.js`
- `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/index/this-not-regexp-constructor.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-leading-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/index/this-cross-realm-constructor.js`
- ... and 114 more files

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same reference path, same feature label, same group key, title overlap)
- `issues/open/066-implement-regexp-literal.md` - Implement RegExp literal support (same reference path, title overlap)
- `issues/open/089-implement-acceptSymbolAsWeakType.md` - Implement Acceptsymbolasweaktype (same feature label, same group key, title overlap)
- `issues/open/1010-implement-autoTypeAssignedUsingDestructuringFromNeverNoCrash.md` - Implement Autotypeassignedusingdestructuringfromnevernocrash (same feature label, same group key, title overlap)
- `issues/open/1033-implement-baseCheck.md` - Implement Basecheck (same feature label, same group key, title overlap)
- `issues/open/1044-implement-bestCommonTypeWithContextualTyping.md` - Implement Bestcommontypewithcontextualtyping (same feature label, same group key, title overlap)
- `issues/open/1048-implement-bigint.md` - Implement Bigint (same feature label, same group key, title overlap)
- `issues/open/1051-implement-bigintIndex.md` - Implement Bigintindex (same feature label, same group key, title overlap)
- `issues/open/1061-implement-bindingPatternContextualTypeDoesNotCauseWidening.md` - Implement Bindingpatterncontextualtypedoesnotcausewidening (same feature label, same group key, title overlap)
- `issues/open/1068-implement-blockScopedBindingsReassignedInLoop-name-resolution.md` - Implement Blockscopedbindingsreassignedinloop Name Resolution (same feature label, same group key, title overlap)

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
        "message": "File '/tmp/tmp0_7fglej/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  }
}
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/437-implement-name-resolution.md` に統合されました。
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
