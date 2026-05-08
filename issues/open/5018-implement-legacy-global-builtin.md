---
id: 5018
title: "Implement legacy-global-builtin support"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

Triage legacy-global-builtin feature across 15 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 15 cases fail with legacy-global-builtin diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: legacy-global-builtin feature has 15 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/escape/escape-above-astral.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/escape/escape-above-astral.js --detail
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
mise run reference-coverage -- test262 --limit 30
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/escape/escape-above-astral.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/escape/escape-above-astral.js
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

- `reference/test262/test/annexB/built-ins/escape/escape-above-astral.js`
- `reference/test262/test/annexB/built-ins/escape/length.js`
- `reference/test262/test/annexB/built-ins/escape/name.js`
- `reference/test262/test/annexB/built-ins/escape/to-primitive-observe.js`
- `reference/test262/test/annexB/built-ins/escape/to-primitive-err.js`
- `reference/test262/test/annexB/built-ins/escape/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/escape/prop-desc.js`
- `reference/test262/test/annexB/built-ins/escape/to-string-observe.js`
- `reference/test262/test/annexB/built-ins/unescape/length.js`
- `reference/test262/test/annexB/built-ins/unescape/to-primitive-observe.js`
- ... and 5 more files

## Duplicate detection

- `issues/open/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same reference path)
- `issues/open/2615-implement-global.md` - Implement Global (same feature label, same group key, title overlap)
- `issues/open/433-implement-legacy-global-builtin.md` - Implement legacy-global-builtin support (same reference path, same feature label, same group key, title overlap)
- `issues/open/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/open/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same feature label, same group key, title overlap)
- `issues/open/344-implement-legacy-global-builtin-bindings.md` - Implement legacy global builtin bindings (8 test262 cases) (same feature label, same group key, title overlap)
- `issues/open/406-direct-eval-annexb-existing-binding-residuals.md` - Direct eval Annex B existing binding residuals (same feature label, same group key)

## Smart triage

### Smart triage: Triage unknown unsupported: escape above astral

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/built-ins/escape/escape-above-astral.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/escape/escape-above-astral.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 832,
  "lines": 22,
  "extension": ".js",
  "first_code_line": "esid: sec-escape-string",
  "test262_metadata": {
    "esid": "sec-escape-string",
    "es6id": "B.2.1.1",
    "description": "Escaping of code units above 255 from string with extended Unicode escape sequence",
    "info": "|"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "invalid unicode escape sequence at 1680..1683",
  "span_start": 1680,
  "span_end": 1683,
  "line": 64,
  "column": 13,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
61 | ---*/
62 | 
63 | assert.sameValue(
64 |   escape('\u{10401}'), '%uD801%uDC01', '\\u{10401} => \\uD801\\uDC01 (surrogate pairs encoded in string)'
65 | );
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
    "path": "issues/done/067-implement-unknown-unsupported.md",
    "title": "Investigate and classify unknown-unsupported cases",
    "reason": "same reference path, same feature label"
  },
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
    "path": "issues/done/1432-implement-conflictMarkerDiff-unknown-unsupported.md",
    "title": "Implement Conflictmarkerdiff Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/1434-implement-conflictMarkerTrivia-unknown-unsupported.md",
    "title": "Implement Conflictmarkertrivia Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/1444-implement-constDeclarations-unknown-unsupported.md",
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
error: [UnsupportedSyntax] invalid unicode escape sequence at 1680..1683
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] invalid unicode escape sequence at 1680..1683
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] invalid unicode escape sequence at 1680..1683
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
        "message": "File '/tmp/tmp9vvhsylj/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function print(message) {\n  console.log(message);\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var $262 = {};",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test262_gc() {}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test262_evalScript(source) {\n  throw new Test262Error(\"$262.evalScript is not supported by this harness slice\")",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test262_createRealm() {\n  return {};\n}",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test262_detachArrayBuffer() {\n  throw new Test262Error(\"$262.detachArrayBuffer is not supported by this harness",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test262_agent_start() {\n  throw new Test262Error(\"$262.agent is not supported by this harness slice\");\n}",
        "line": 22,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.global = {};",
        "line": 26,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.gc = test262_gc;",
        "line": 27,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.evalScript = test262_evalScript;",
        "line": 28,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.createRealm = test262_createRealm;",
        "line": 29,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.detachArrayBuffer = test262_detachArrayBuffer;",
        "line": 30,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.IsHTMLDDA = undefined;",
        "line": 31,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.agent = {};",
        "line": 32,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.agent.start = test262_agent_start;",
        "line": 33,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function Test262Error(message) {\n  this.message = message || \"\";\n}",
        "line": 50,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Test262Error.prototype.toString = function () {\n  return \"Test262Error: \" + this.message;\n};",
        "line": 54,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Test262Error.thrower = function (message) {\n  throw new Test262Error(message);\n};",
        "line": 58,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function $DONOTEVALUATE() {\n  throw \"Test262: This statement should not be evaluated.\";\n}",
        "line": 62,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function assert(mustBeTrue, message) {\n  if (mustBeTrue === true) {\n    return;\n  }\n\n  if (message === undefined) {\n    ",
        "line": 78,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function print(message) {\n  console.log(message);\n}\n\nvar $262 = {};\n\nfunction test262_gc() {}\n\nfunction test262_evalScri",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function assert(mustBeTrue, message) {\n  if (mustBeTrue === true) {\n    return;\n  }\n\n  if (message === undefined) {\n    ",
        "line": 78,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] invalid unicode escape sequence at 1680..1683
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
