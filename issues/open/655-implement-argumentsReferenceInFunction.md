---
id: 655
title: "Implement Argumentsreferenceinfunction"
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

Triage argumentsReferenceInFunction across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsReferenceInFunction` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsReferenceInFunction has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts
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

- `reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage arguments object: argumentsReferenceInFunction1 Js

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 368,
  "lines": 21,
  "extension": ".ts",
  "first_code_line": "const format = function(f) {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-268: for-loop increment/decrement updates currently require an identifier target at 245..248",
  "span_start": 245,
  "span_end": 248,
  "line": 13,
  "column": 43,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
10 |   var i = 1;
11 |   var args = arguments;
12 |   var len = args.length;
13 |   for (var x = args[i]; i < len; x = args[++i]) {
14 |     str += ' ' + x;
15 |   }
16 |   return str;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "format",
    "line": 8,
    "column": 1,
    "initializer": "function(f) {"
  },
  {
    "kind": "binding",
    "name": "str",
    "line": 9,
    "column": 3,
    "initializer": "''"
  },
  {
    "kind": "binding",
    "name": "i",
    "line": 10,
    "column": 3,
    "initializer": "1"
  },
  {
    "kind": "binding",
    "name": "args",
    "line": 11,
    "column": 3,
    "initializer": "arguments"
  },
  {
    "kind": "binding",
    "name": "len",
    "line": 12,
    "column": 3,
    "initializer": "args.length"
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 13,
    "column": 8,
    "initializer": "args[i]"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/311-fix-test262-arguments-object-index-assignment.md",
    "title": "Fix test262 arguments object index assignment semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/646-implement-arguments.md",
    "title": "Implement Arguments",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/647-implement-argumentsAsPropertyName-arguments-object.md",
    "title": "Implement Argumentsaspropertyname Arguments Object",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/653-implement-argumentsReferenceInConstructor-arguments-object.md",
    "title": "Implement Argumentsreferenceinconstructor Arguments Object",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/291-provide-object-global-binding-for-test262.md",
    "title": "Provide Object global binding for test262 cases",
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
        kind: Const,
        span: Span {
            start: 96,
            end: 101,
        },
    },
    SpannedToken {
        kind: Ident(
            "format",
        ),
        span: Span {
            start: 102,
            end: 108,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 111,
            end: 119,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 127,
            end: 130,
        },
    },
    SpannedToken {
        kind: Ident(
            "str",
        ),
        span: Span {
            start: 131,
            end: 134,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: String(
            "",
        ),
        span: Span {
            start: 137,
            end: 139,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 139,
            end: 140,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 143,
            end: 146,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 151,
            end: 152,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 152,
            end: 153,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 156,
            end: 159,
        },
    },
    SpannedToken {
        kind: Ident(
            "args",
        ),
        span: Span {
            start: 160,
            end: 164,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 165,
            end: 166,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 167,
            end: 176,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 176,
            end: 177,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 180,
            end: 183,
        },
    },
    SpannedToken {
        kind: Ident(
            "len",
        ),
        span: Span {
            start: 184,
            end: 187,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "format",
        expr: FunctionExpr {
            name: "",
            params: [
                (
                    "f",
                    None,
                    false,
                ),
            ],
            body: [
                Let {
                    name: "str",
                    expr: String {
                        value: "",
                        span: Span {
                            start: 137,
                            end: 139,
                        },
                    },
                    span: Span {
                        start: 127,
                        end: 140,
                    },
                },
                Let {
                    name: "i",
                    expr: Number {
                        value: 1,
                        span: Span {
                            start: 151,
                            end: 152,
                        },
                    },
                    span: Span {
                        start: 143,
                        end: 153,
                    },
                },
                Let {
                    name: "args",
                    expr: Ident {
                        name: "arguments",
                        span: Span {
                            start: 167,
                            end: 176,
                        },
                    },
                    span: Span {
                        start: 156,
                        end: 177,
                    },
                },
                Let {
                    name: "len",
                    expr: Member {
                        object: Ident {
                            name: "args",
                            span: Span {
                                start: 190,
                                end: 194,
                            },
                        },
                        property: "length",
                        span: Span {
                            start: 190,
                            end: 201,
                        },
                    },
                    span: Span {
                        start: 180,
                        end: 202,
                    },
                },
                For {
                    init: Some(
                        Let {
                            name: "x",
                            expr: Index {
                                object: Ident {
                                    name: "args",
                                    span: Span {
                                        start: 218,
                                        end: 222,
                                    },
                                },
                                index: Ident {
                                    name: "i",
                                    span: Span {
                                        start: 223,
                                        end: 224,
                                    },
                                },
                                span: Span {
                                    start: 218,
                                    end: 225,
                                },
                            },
                            span: Span {
                                start: 210,
                                end: 226,
                            },
                        },
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-268: for-loop increment/decrement updates currently require an identifier target at 245..248
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
        "code": 2345,
        "category": "Error",
        "message": "Argument of type 'IArguments' is not assignable to parameter of type '[f: any]'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts",
        "start": 353,
        "length": 9,
        "line": 20,
        "character": 29
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "(f: any) => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts",
        "start": 102,
        "length": 6,
        "line": 8,
        "character": 7,
        "name": "format"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts",
        "start": 120,
        "length": 1,
        "line": 8,
        "character": 25,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts",
        "start": 131,
        "length": 3,
        "line": 9,
        "character": 7,
        "name": "str"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts",
        "start": 147,
        "length": 1,
        "line": 10,
        "character": 7,
        "name": "i"
      },
      {
        "kind": "binding",
        "typeText": "IArguments",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts",
        "start": 160,
        "length": 4,
        "line": 11,
        "character": 7,
        "name": "args"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts",
        "start": 184,
        "length": 3,
        "line": 12,
        "character": 7,
        "name": "len"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts",
        "start": 214,
        "length": 1,
        "line": 13,
        "character": 12,
        "name": "x"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts",
        "start": 264,
        "length": 7,
        "line": 14,
        "character": 12,
        "operator": "+",
        "leftType": "\" \"",
        "rightType": "any"
      },
      {
        "kind": "binding",
        "typeText": "() => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInFunction1_Js.ts",
        "start": 301,
        "length": 8,
        "line": 19,
        "character": 7,
        "name": "debuglog"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const format = function(f) {\n  var str = '';\n  var i = 1;\n  var args = arguments;\n  var len = args.length;\n  for (var x ",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const debuglog = function() {\n  return format.apply(null, arguments);\n};",
        "line": 19,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const format = function(f) {\n  var str = '';\n  var i = 1;\n  var args = arguments;\n  var len = args.length;\n  for (var x ",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const format = function(f) {\n  var str = '';\n  var i = 1;\n  var args = arguments;\n  var len = args.length;\n  for (var x ",
        "line": 8,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const format = function(f) {\n  var str = '';\n  var i = 1;\n  var args = arguments;\n  var len = args.length;\n  for (var x ",
        "line": 8,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "format = function(f) {\n  var str = '';\n  var i = 1;\n  var args = arguments;\n  var len = args.length;\n  for (var x = args",
        "line": 8,
        "character": 7
      },
      {
        "kind": "FunctionExpression",
        "text": "function(f) {\n  var str = '';\n  var i = 1;\n  var args = arguments;\n  var len = args.length;\n  for (var x = args[i]; i < ",
        "line": 8,
        "character": 16
      },
      {
        "kind": "Block",
        "text": "{\n  var str = '';\n  var i = 1;\n  var args = arguments;\n  var len = args.length;\n  for (var x = args[i]; i < len; x = arg",
        "line": 8,
        "character": 28
      },
      {
        "kind": "ForStatement",
        "text": "for (var x = args[i]; i < len; x = args[++i]) {\n    str += ' ' + x;\n  }",
        "line": 13,
        "character": 3
      },
      {
        "kind": "BinaryExpression",
        "text": "x = args[++i]",
        "line": 13,
        "character": 34
      },
      {
        "kind": "ElementAccessExpression",
        "text": "args[++i]",
        "line": 13,
        "character": 38
      },
      {
        "kind": "PrefixUnaryExpression",
        "text": "++i",
        "line": 13,
        "character": 43
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-268: for-loop increment/decrement updates currently require an identifier target at 245..248
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
