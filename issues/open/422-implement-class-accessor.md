---
id: 422
title: "Implement class-accessor support"
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

Triage class-accessor feature across 90 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 90 cases fail with class-accessor diagnostic.

### Triage breakdown (6 sampled files)

| # | File | True cause | Category |
|---|------|-----------|----------|
| 1 | accessorAccidentalCallDiagnostic | Resolver: type parameter method call (issue-211) | `frontend/resolver` |
| 2 | accessorDeclarationOrder | Parser: `class_private_element()` skips `#name: type;` | **`frontend/syntax`** (5-10 line fix) |
| 3 | accessorInferredReturnTypeErrorInReturnStatement | Module: `export var` → UnsupportedModule | `module` resolution pending |
| 4 | accessorWithoutBody1 | Parser: object literal `get foo()` (no body) | **`frontend/syntax`** (30-50 line fix) |
| 5 | accessorWithoutBody2 | Parser: object literal `set foo(a)` (no body) | **`frontend/syntax`** (30-50 line fix) |
| 6 | aliasUsageInAccessorsOfClass | Module: `import ... = require(...)` → UnsupportedModule | `module` resolution pending |

**Result: 3 of 6 sampled files are true parser issues.**
Remaining 3 are resolver (1) and module (2) issues — reclassified to their respective areas.

The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: class-accessor feature has 90 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/harness/verifyProperty-restore-accessor-symbol.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/harness/verifyProperty-restore-accessor-symbol.js --detail
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

Note: After triage, 3 of 6 sampled issues are not frontend/syntax — reclassify to `frontend/resolver` and `module` as appropriate.

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
mise run reference-coverage -- test262 --limit 180
mise run reference-coverage -- test262 --path-filter reference/test262/test/harness/verifyProperty-restore-accessor-symbol.js --detail
mise run reference-triage -- test262 reference/test262/test/harness/verifyProperty-restore-accessor-symbol.js
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

- `reference/test262/test/harness/verifyProperty-restore-accessor-symbol.js`
- `reference/test262/test/harness/verifyProperty-restore-accessor.js`
- `reference/test262/test/language/arguments-object/mapped/enumerable-configurable-accessor-descriptor.js`
- `reference/test262/test/language/computed-property-names/object/accessor/getter-duplicates.js`
- `reference/test262/test/language/computed-property-names/object/accessor/getter-super.js`
- `reference/test262/test/language/computed-property-names/object/accessor/getter.js`
- `reference/test262/test/language/computed-property-names/object/accessor/setter-duplicates.js`
- `reference/test262/test/language/computed-property-names/object/accessor/setter-super.js`
- `reference/test262/test/language/computed-property-names/object/accessor/setter.js`
- `reference/test262/test/language/expressions/compound-assignment/left-hand-side-private-reference-accessor-property-add.js`
- ... and 80 more files

## Duplicate detection

- `issues/open/078-implement-MemberAccessorDeclaration.md` - Implement Memberaccessordeclaration (same feature label, same group key, title overlap)
- `issues/open/094-implement-accessorAccidentalCallDiagnostic.md` - Implement Accessoraccidentalcalldiagnostic (same feature label, same group key, title overlap)
- `issues/open/098-implement-accessorDeclarationOrder.md` - Implement Accessordeclarationorder (same feature label, same group key, title overlap)
- `issues/open/099-implement-accessorInAmbientContextES.md` - Implement Accessorinambientcontextes (same feature label, same group key, title overlap)
- `issues/open/100-implement-accessorInferredReturnTypeErrorInReturnStatement.md` - Implement Accessorinferredreturntypeerrorinreturnstatement (same feature label, same group key, title overlap)
- `issues/open/101-implement-accessorParameterAccessibilityModifier.md` - Implement Accessorparameteraccessibilitymodifier (same feature label, same group key, title overlap)
- `issues/open/102-implement-accessorWithInitializer.md` - Implement Accessorwithinitializer (same feature label, same group key, title overlap)
- `issues/open/103-implement-accessorWithLineTerminator.md` - Implement Accessorwithlineterminator (same feature label, same group key, title overlap)
- `issues/open/104-implement-accessorWithRestParam.md` - Implement Accessorwithrestparam (same feature label, same group key, title overlap)
- `issues/open/105-implement-accessorWithoutBody.md` - Implement Accessorwithoutbody (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage unknown unsupported: verifyProperty restore accessor symbol

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/harness/verifyProperty-restore-accessor-symbol.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/harness/verifyProperty-restore-accessor-symbol.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 935,
  "lines": 43,
  "extension": ".js",
  "first_code_line": "description: >",
  "test262_metadata": {
    "description": ">",
    "includes": "[propertyHelper.js]",
    "features": "[Symbol]"
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
[]
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
        "message": "File '/tmp/tmpqslk0ega/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
