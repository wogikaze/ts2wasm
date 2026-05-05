---
id: 5012
title: "Implement Date object support"
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

Triage date feature across 20 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 20 cases fail with date diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: date feature has 20 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Date/prototype/getYear/length.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Date/prototype/getYear/length.js --detail
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
mise run reference-coverage -- test262 --limit 40
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Date/prototype/getYear/length.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Date/prototype/getYear/length.js
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

- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/length.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/nan.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/return-value.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/name.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/date-value-read-before-tonumber-when-date-is-valid.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/B.2.5.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/date-value-read-before-tonumber-when-date-is-invalid.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/this-time-valid.js`
- ... and 10 more files

## Duplicate detection

- `issues/done/017b-implement-gc-strategy.md` - Implement GC strategy (same feature label, same group key, title overlap)
- `issues/open/021-implement-full-wasm-backend.md` - Implement full wasm backend (same feature label, same group key, title overlap)
- `issues/open/050-implement-date.md` - Implement Date (same feature label, same group key, title overlap)
- `issues/open/052-implement-json.md` - Implement JSON (same feature label, same group key, title overlap)
- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same reference path, same feature label, same group key, title overlap)
- `issues/done/066-implement-regexp-literal.md` - Implement RegExp literal support (same feature label, same group key, title overlap)
- `issues/done/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/done/068-implement-unsupported-expression.md` - Implement unsupported expression types (same feature label, same group key, title overlap)
- `issues/done/069-implement-APILibCheck.md` - Implement Apilibcheck (same feature label, same group key, title overlap)
- `issues/open/070-implement-APISample.md` - Implement Apisample (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage unknown unsupported: length

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/built-ins/Date/prototype/getYear/length.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Date/prototype/getYear/length.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1052,
  "lines": 30,
  "extension": ".js",
  "first_code_line": "es6id: B.2.4.1",
  "test262_metadata": {
    "es6id": "B.2.4.1",
    "description": ">",
    "info": "|",
    "17 ECMAScript Standard Built-in Objects": "",
    "object has the attributes { [[Writable]]": "false, [[Enumerable]]: false,",
    "[[Configurable]]": "true }.",
    "includes": "[propertyHelper.js]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 3763, end: 3764 } }) at 3764..3765",
  "span_start": 3764,
  "span_end": 3765,
  "line": 133,
  "column": 6,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
130 |         names[i] === "get" ||
131 |         names[i] === "set",
132 |       "Invalid descriptor field: " + names[i],
133 |     );
134 |   }
135 | 
136 |   var failures = [];
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
    "name": "__isArray",
    "line": 68,
    "column": 1,
    "initializer": "Array.isArray"
  },
  {
    "kind": "binding",
    "name": "__defineProperty",
    "line": 69,
    "column": 1,
    "initializer": "Object.defineProperty"
  },
  {
    "kind": "binding",
    "name": "__getOwnPropertyDescriptor",
    "line": 70,
    "column": 1,
    "initializer": "Object.getOwnPropertyDescriptor"
  },
  {
    "kind": "binding",
    "name": "__getOwnPropertyNames",
    "line": 71,
    "column": 1,
    "initializer": "Object.getOwnPropertyNames"
  },
  {
    "kind": "binding",
    "name": "__join",
    "line": 72,
    "column": 1,
    "initializer": "Function.prototype.call.bind(Array.prototype.join)"
  },
  {
    "kind": "binding",
    "name": "__push",
    "line": 73,
    "column": 1,
    "initializer": "Function.prototype.call.bind(Array.prototype.push)"
  },
  {
    "kind": "binding",
    "name": "__hasOwnProperty",
    "line": 74,
    "column": 1,
    "initializer": "Function.prototype.call.bind(Object.prototype.hasOwnProperty)"
  },
  {
    "kind": "binding",
    "name": "__propertyIsEnumerable",
    "line": 75,
    "column": 1,
    "initializer": "Function.prototype.call.bind(Object.prototype.propertyIsEnumerable)"
  },
  {
    "kind": "binding",
    "name": "nonIndexNumericPropertyName",
    "line": 76,
    "column": 1,
    "initializer": "Math.pow(2, 32) - 1"
  },
  {
    "kind": "function",
    "name": "verifyProperty",
    "line": 85,
    "column": 1,
    "params": "obj, name, desc, options"
  },
  {
    "kind": "binding",
    "name": "originalDesc",
    "line": 91,
    "column": 3,
    "initializer": "__getOwnPropertyDescriptor(obj, name)"
  },
  {
    "kind": "binding",
    "name": "nameStr",
    "line": 92,
    "column": 3,
    "initializer": "String(name)"
  },
  {
    "kind": "binding",
    "name": "names",
    "line": 123,
    "column": 3,
    "initializer": "__getOwnPropertyNames(desc)"
  },
  {
    "kind": "binding",
    "name": "i",
    "line": 124,
    "column": 8,
    "initializer": "0"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/064-implement-name-resolution.md",
    "title": "Implement name resolution (triaged - superseded by test262 metadata issues)",
    "reason": "same reference path"
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
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Function,
        span: Span {
            start: 1,
            end: 9,
        },
    },
    SpannedToken {
        kind: Ident(
            "print",
        ),
        span: Span {
            start: 10,
            end: 15,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 15,
            end: 16,
        },
    },
    SpannedToken {
        kind: Ident(
            "message",
        ),
        span: Span {
            start: 16,
            end: 23,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 23,
            end: 24,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 25,
            end: 26,
        },
    },
    SpannedToken {
        kind: Ident(
            "console",
        ),
        span: Span {
            start: 29,
            end: 36,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "log",
        ),
        span: Span {
            start: 37,
            end: 40,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "message",
        ),
        span: Span {
            start: 41,
            end: 48,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 85,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "NaN",
        ),
        span: Span {
            start: 89,
            end: 92,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Slash,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 100,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "Infinity",
        ),
        span: Span {
            start: 104,
            end: 112,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Slash,
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 3763, end: 3764 } }) at 3764..3765
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 3763, end: 3764 } }) at 3764..3765
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
        "message": "File '/tmp/tmpge6iut5c/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
        "kind": "FunctionExpression",
        "text": "function (expectedErrorConstructor, func, message) {\n  var expectedName, actualName;\n  if (typeof func !== \"function\") {",
        "line": 136,
        "character": 17
      },
      {
        "kind": "Block",
        "text": "{\n  var expectedName, actualName;\n  if (typeof func !== \"function\") {\n    throw new Test262Error('assert.throws requires",
        "line": 136,
        "character": 68
      },
      {
        "kind": "TryStatement",
        "text": "try {\n    func();\n  } catch (thrown) {\n    if (typeof thrown !== 'object' || thrown === null) {\n      message += 'Thrown",
        "line": 149,
        "character": 3
      },
      {
        "kind": "CatchClause",
        "text": "catch (thrown) {\n    if (typeof thrown !== 'object' || thrown === null) {\n      message += 'Thrown value was not an obje",
        "line": 151,
        "character": 5
      },
      {
        "kind": "Block",
        "text": "{\n    if (typeof thrown !== 'object' || thrown === null) {\n      message += 'Thrown value was not an object!';\n      thr",
        "line": 151,
        "character": 20
      },
      {
        "kind": "IfStatement",
        "text": "if (typeof thrown !== 'object' || thrown === null) {\n      message += 'Thrown value was not an object!';\n      throw new",
        "line": 152,
        "character": 5
      },
      {
        "kind": "IfStatement",
        "text": "if (thrown.constructor !== expectedErrorConstructor) {\n      expectedName = expectedErrorConstructor.name;\n      actualN",
        "line": 155,
        "character": 12
      },
      {
        "kind": "Block",
        "text": "{\n      expectedName = expectedErrorConstructor.name;\n      actualName = thrown.constructor.name;\n      if (expectedName",
        "line": 155,
        "character": 65
      },
      {
        "kind": "ExpressionStatement",
        "text": "expectedName = expectedErrorConstructor.name;",
        "line": 156,
        "character": 7
      },
      {
        "kind": "BinaryExpression",
        "text": "expectedName = expectedErrorConstructor.name",
        "line": 156,
        "character": 7
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "expectedErrorConstructor.name",
        "line": 156,
        "character": 22
      },
      {
        "kind": "Identifier",
        "text": "name",
        "line": 156,
        "character": 47
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 3763, end: 3764 } }) at 3764..3765
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
