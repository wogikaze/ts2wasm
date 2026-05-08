---
id: 428
title: "Implement enum support"
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

Triage enum feature across 59 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 59 cases fail with enum diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: enum feature has 59 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js --detail
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
mise run reference-coverage -- test262 --limit 118
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js
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

- `reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-else-decl-a-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-else-decl-b-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-else-stmt-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-no-else-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-stmt-else-decl-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/switch-case-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/switch-dflt-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/harness/propertyhelper-verifyenumerable-enumerable-symbol.js`
- `reference/test262/test/harness/propertyhelper-verifyenumerable-enumerable.js`
- ... and 49 more files

## Duplicate detection

- `issues/open/144-implement-ambientConstLiterals.md` - Implement Ambientconstliterals (same feature label, same group key, title overlap)
- `issues/open/145-implement-ambientEnum.md` - Implement Ambientenum (same feature label, same group key, title overlap)
- `#146` - Implement Ambientenumelementinitializer (same feature label, same group key, title overlap)
- `issues/open/159-implement-ambientModuleWithTemplateLiterals.md` - Implement Ambientmodulewithtemplateliterals (same feature label, same group key, title overlap)
- `issues/open/175-implement-amdModuleConstEnumUsage.md` - Implement Amdmoduleconstenumusage (same feature label, same group key, title overlap)
- `issues/done/255-implement-private-class-element-runtime-semantics.md` - Implement private class element runtime semantics (same feature label, same group key, title overlap)
- `issues/open/274-implement-spread-operator.md` - Implement spread operator (same feature label, same group key, title overlap)
- `issues/open/336-implement-test262-includes-directive.md` - Implement test262 includes directive processing (same feature label, same group key, title overlap)
- `issues/open/353-spread-iterator-protocol.md` - Implement iterator protocol integration for spread operator (same feature label, same group key, title overlap)
- `issues/done/020a-design-javascript-semantic-ir.md` - issues/done/020a-design-javascript-semantic-ir.md (same feature label, same group key)

## Smart triage

### Smart triage: Triage unknown unsupported: block decl global existing non enumerable global init

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1278,
  "lines": 49,
  "extension": ".js",
  "first_code_line": "description: Variable binding is left in place by legacy function hoisting. CreateGlobalVariableBinding leaves the binding as non-enumerable even if it has the ",
  "test262_metadata": {
    "description": "Variable binding is left in place by legacy function hoisting. CreateGlobalVariableBinding leaves the binding as non-enumerable even if it has the chance to change it to be enumerable. (Block statement in the global scope containing a function declaration)",
    "esid": "sec-web-compat-globaldeclarationinstantiation",
    "flags": "[generated, noStrict]",
    "includes": "[fnGlobalObject.js, propertyHelper.js]",
    "info": "|"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 4137, end: 4138 } }) at 4138..4139",
  "span_start": 4138,
  "span_end": 4139,
  "line": 149,
  "column": 6,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
146 |         names[i] === "get" ||
147 |         names[i] === "set",
148 |       "Invalid descriptor field: " + names[i],
149 |     );
150 |   }
151 | 
152 |   var failures = [];
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
    "name": "__globalObject",
    "line": 52,
    "column": 1,
    "initializer": "Function(\"return this"
  },
  {
    "kind": "function",
    "name": "fnGlobalObject",
    "line": 53,
    "column": 1,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "__isArray",
    "line": 84,
    "column": 1,
    "initializer": "Array.isArray"
  },
  {
    "kind": "binding",
    "name": "__defineProperty",
    "line": 85,
    "column": 1,
    "initializer": "Object.defineProperty"
  },
  {
    "kind": "binding",
    "name": "__getOwnPropertyDescriptor",
    "line": 86,
    "column": 1,
    "initializer": "Object.getOwnPropertyDescriptor"
  },
  {
    "kind": "binding",
    "name": "__getOwnPropertyNames",
    "line": 87,
    "column": 1,
    "initializer": "Object.getOwnPropertyNames"
  },
  {
    "kind": "binding",
    "name": "__join",
    "line": 88,
    "column": 1,
    "initializer": "Function.prototype.call.bind(Array.prototype.join)"
  },
  {
    "kind": "binding",
    "name": "__push",
    "line": 89,
    "column": 1,
    "initializer": "Function.prototype.call.bind(Array.prototype.push)"
  },
  {
    "kind": "binding",
    "name": "__hasOwnProperty",
    "line": 90,
    "column": 1,
    "initializer": "Function.prototype.call.bind(Object.prototype.hasOwnProperty)"
  },
  {
    "kind": "binding",
    "name": "__propertyIsEnumerable",
    "line": 91,
    "column": 1,
    "initializer": "Function.prototype.call.bind(Object.prototype.propertyIsEnumerable)"
  },
  {
    "kind": "binding",
    "name": "nonIndexNumericPropertyName",
    "line": 92,
    "column": 1,
    "initializer": "Math.pow(2, 32) - 1"
  },
  {
    "kind": "function",
    "name": "verifyProperty",
    "line": 101,
    "column": 1,
    "params": "obj, name, desc, options"
  },
  {
    "kind": "binding",
    "name": "originalDesc",
    "line": 107,
    "column": 3,
    "initializer": "__getOwnPropertyDescriptor(obj, name)"
  },
  {
    "kind": "binding",
    "name": "nameStr",
    "line": 108,
    "column": 3,
    "initializer": "String(name)"
  },
  {
    "kind": "binding",
    "name": "names",
    "line": 139,
    "column": 3,
    "initializer": "__getOwnPropertyNames(desc)"
  },
  {
    "kind": "binding",
    "name": "i",
    "line": 140,
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
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 4137, end: 4138 } }) at 4138..4139
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 4137, end: 4138 } }) at 4138..4139
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
        "message": "File '/tmp/tmpj_j38de7/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
        "kind": "ExpressionStatement",
        "text": "assert.throws = function (expectedErrorConstructor, func, message) {\n  var expectedName, actualName;\n  if (typeof func !",
        "line": 136,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "assert.throws = function (expectedErrorConstructor, func, message) {\n  var expectedName, actualName;\n  if (typeof func !",
        "line": 136,
        "character": 1
      },
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
        "kind": "ExpressionStatement",
        "text": "message += 'Expected a ' + expectedErrorConstructor.name + ' to be thrown but no exception was thrown at all';",
        "line": 168,
        "character": 3
      },
      {
        "kind": "BinaryExpression",
        "text": "message += 'Expected a ' + expectedErrorConstructor.name + ' to be thrown but no exception was thrown at all'",
        "line": 168,
        "character": 3
      },
      {
        "kind": "BinaryExpression",
        "text": "'Expected a ' + expectedErrorConstructor.name + ' to be thrown but no exception was thrown at all'",
        "line": 168,
        "character": 14
      },
      {
        "kind": "BinaryExpression",
        "text": "'Expected a ' + expectedErrorConstructor.name",
        "line": 168,
        "character": 14
      },
      {
        "kind": "StringLiteral",
        "text": "'Expected a '",
        "line": 168,
        "character": 14
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 4137, end: 4138 } }) at 4138..4139
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

**truly-done** (428)

- Implementation commits: verified via `git log --oneline --all --grep=428`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
