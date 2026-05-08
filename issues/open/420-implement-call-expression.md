---
id: 420
title: "Implement call expression support"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage call-expression feature across 17 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 17 cases fail with call-expression diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: call-expression feature has 17 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/expressions/super/prop-expr-cls-ref-this.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/super/prop-expr-cls-ref-this.js --detail
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
mise run reference-coverage -- test262 --limit 34
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/super/prop-expr-cls-ref-this.js --detail
mise run reference-triage -- test262 reference/test262/test/language/expressions/super/prop-expr-cls-ref-this.js
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

- `reference/test262/test/language/expressions/super/prop-expr-cls-ref-this.js`
- `reference/test262/test/language/function-code/10.4.3-1-76-s.js`
- `reference/test262/test/language/function-code/10.4.3-1-76gs.js`
- `reference/test262/test/language/function-code/10.4.3-1-77-s.js`
- `reference/test262/test/language/function-code/10.4.3-1-77gs.js`
- `reference/test262/test/language/function-code/10.4.3-1-78-s.js`
- `reference/test262/test/language/function-code/10.4.3-1-78gs.js`
- `reference/test262/test/language/function-code/10.4.3-1-80-s.js`
- `reference/test262/test/language/function-code/10.4.3-1-80gs.js`
- `reference/test262/test/language/statements/for-of/head-const-fresh-binding-per-iteration.js`
- ... and 7 more files

## Duplicate detection

- `issues/done/274-implement-spread-operator.md` - Implement spread operator (same feature label, same group key, title overlap)
- `issues/done/311-fix-test262-arguments-object-index-assignment.md` - Fix test262 arguments object index assignment semantics (same feature label, same group key)

## Smart triage

### Smart triage: Triage call expression: prop expr cls ref this

- Issue class: `triage-needed`
- Feature label: `call-expression`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/language/expressions/super/prop-expr-cls-ref-this.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/expressions/super/prop-expr-cls-ref-this.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1861,
  "lines": 58,
  "extension": ".js",
  "first_code_line": "esid: sec-super-keyword",
  "test262_metadata": {
    "esid": "sec-super-keyword",
    "es6id": "12.3.5",
    "description": "SuperProperty's \"this\" value",
    "info": "|",
    "12.3.5.3 Runtime Semantics": "MakeSuperPropertyReference",
    "i. Assert": "In this case, base will never be null or undefined."
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "only identifier calls are supported in expression context at 2605..2623",
  "span_start": 2605,
  "span_end": 2623,
  "line": 93,
  "column": 15,
  "feature_label": "call-expression",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
90 | }
91 | class C extends Parent {
92 |   method() {
93 |     viaCall = super['getThis']();
94 |     viaMember = super['This'];
95 |   }
96 | }
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
    "name": "strict",
    "line": 53,
    "column": 24
  },
  {
    "kind": "binding",
    "name": "strict",
    "line": 53,
    "column": 49
  },
  {
    "kind": "binding",
    "name": "viaCall",
    "line": 81,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "viaMember",
    "line": 82,
    "column": 1
  },
  {
    "kind": "class",
    "name": "Parent",
    "line": 83,
    "column": 1
  },
  {
    "kind": "class",
    "name": "C",
    "line": 91,
    "column": 1
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

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Function {
        name: "print",
        params: [
            (
                "message",
                None,
                false,
            ),
        ],
        body: [
            Expr {
                expr: Call {
                    callee: Member {
                        object: Ident {
                            name: "console",
                            span: Span {
                                start: 29,
                                end: 36,
                            },
                        },
                        property: "log",
                        span: Span {
                            start: 29,
                            end: 40,
                        },
                    },
                    args: [
                        Ident {
                            name: "message",
                            span: Span {
                                start: 41,
                                end: 48,
                            },
                        },
                    ],
                    span: Span {
                        start: 29,
                        end: 49,
                    },
                },
                span: Span {
                    start: 29,
                    end: 50,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 1,
            end: 50,
        },
    },
    Let {
        name: "NaN",
        expr: Binary {
            left: Number {
                value: 0,
                span: Span {
                    start: 95,
                    end: 96,
                },
            },
            op: Divide,
            right: Number {
                value: 0,
                span: Span {
                    start: 97,
                    end: 98,
                },
            },
            span: Span {
                start: 95,
                end: 98,
            },
        },
        span: Span {
            start: 85,
            end: 99,
        },
    },
    Let {
        name: "Infinity",
        expr: Binary {
            left: Number {
                value: 1,
                span: Span {
                    start: 115,
                    end: 116,
                },
            },
            op: Divide,
            right: Number {
                value: 0,
                span: Span {
                    start: 117,
                    end: 118,
                },
            },
            span: Span {
                start: 115,
                end: 118,
            },
        },
        span: Span {
            start: 100,
            end: 119,
        },
    },
    Let {
        name: "$262",
        expr: Object {
            props: [],
            span: Span {
                start: 182,
                end: 184,
            },
        },
        span: Span {
            start: 171,
            end: 185,
        },
    },
    Expr {
        expr: PropertyAssign {
            object: Ident {
                name: "$262",
                span: Span {
                    start: 186,
                    end: 190,
                },
            },
            property: "gc",
            value: FunctionExpr {
                name: "",
                params: [],
                body: [],
                span: Span {
                    start: 196,
                    end: 204,
                },
            },
            span: Span {
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] only identifier calls are supported in expression context at 2605..2623
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
        "message": "File '/tmp/tmpw5x0lefu/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
        "kind": "BinaryExpression",
        "text": "assert.sameValue = function (actual, expected, message) {\n  try {\n    if (assert._isSameValue(actual, expected)) {\n     ",
        "line": 99,
        "character": 1
      },
      {
        "kind": "FunctionExpression",
        "text": "function (actual, expected, message) {\n  try {\n    if (assert._isSameValue(actual, expected)) {\n      return;\n    }\n  } ",
        "line": 99,
        "character": 20
      },
      {
        "kind": "Block",
        "text": "{\n  try {\n    if (assert._isSameValue(actual, expected)) {\n      return;\n    }\n  } catch (error) {\n    throw new Test262",
        "line": 99,
        "character": 57
      },
      {
        "kind": "ExpressionStatement",
        "text": "message += 'Expected SameValue(«' + assert._toString(actual) + '», «' + assert._toString(expected) + '») to be true';",
        "line": 115,
        "character": 3
      },
      {
        "kind": "BinaryExpression",
        "text": "message += 'Expected SameValue(«' + assert._toString(actual) + '», «' + assert._toString(expected) + '») to be true'",
        "line": 115,
        "character": 3
      },
      {
        "kind": "BinaryExpression",
        "text": "'Expected SameValue(«' + assert._toString(actual) + '», «' + assert._toString(expected) + '») to be true'",
        "line": 115,
        "character": 14
      },
      {
        "kind": "BinaryExpression",
        "text": "'Expected SameValue(«' + assert._toString(actual) + '», «' + assert._toString(expected)",
        "line": 115,
        "character": 14
      },
      {
        "kind": "BinaryExpression",
        "text": "'Expected SameValue(«' + assert._toString(actual) + '», «'",
        "line": 115,
        "character": 14
      },
      {
        "kind": "BinaryExpression",
        "text": "'Expected SameValue(«' + assert._toString(actual)",
        "line": 115,
        "character": 14
      },
      {
        "kind": "CallExpression",
        "text": "assert._toString(actual)",
        "line": 115,
        "character": 39
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "assert._toString",
        "line": 115,
        "character": 39
      },
      {
        "kind": "Identifier",
        "text": "_toString",
        "line": 115,
        "character": 46
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] only identifier calls are supported in expression context at 2605..2623
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

**truly-done** (420)

- Implementation commits: verified via `git log --oneline --all --grep=420`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
