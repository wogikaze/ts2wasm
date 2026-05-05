---
id: 646
title: "Implement Arguments"
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

Triage arguments across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arguments` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arguments has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arguments.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arguments.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arguments.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arguments.ts
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

- `reference/typescript/tests/cases/compiler/arguments.ts`

## Duplicate detection

- `issues/done/193-implement-arguments.md` - Implement Arguments (same reference path, same group key, title overlap)
- `issues/open/194-implement-argumentsAsPropertyName.md` - Implement Argumentsaspropertyname (same feature label, same group key, title overlap)
- `issues/done/197-implement-argumentsObjectIterator.md` - Implement Argumentsobjectiterator (same feature label, same group key, title overlap)
- `issues/done/198-implement-argumentsPropertyNameInJsMode.md` - Implement Argumentspropertynameinjsmode (same feature label, same group key, title overlap)
- `issues/done/311-fix-test262-arguments-object-index-assignment.md` - Fix test262 arguments object index assignment semantics (same feature label, same group key, title overlap)
- `issues/done/412-implement-arguments-object.md` - Implement arguments-object support (same feature label, same group key, title overlap)
- `issues/open/413-implement-arity.md` - Implement arity support (same feature label, same group key, title overlap)
- `issues/open/416-implement-async.md` - Implement async/await support (same feature label, same group key, title overlap)
- `issues/open/420-implement-call-expression.md` - Implement call expression support (same feature label, same group key, title overlap)
- `issues/open/422-implement-class-accessor.md` - Implement class-accessor support (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage arguments object: arguments

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arguments.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arguments.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 334,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "function f() {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-062d: `arguments` is only supported inside non-arrow functions in this milestone at 97..106",
  "span_start": 97,
  "span_end": 106,
  "line": 7,
  "column": 14,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 4 |     (() => arguments)();
 5 | }
 6 | 
 7 | (() => arguments)();
 8 | 
 9 | interface I {
10 |     method(args: typeof arguments): void;
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "f",
    "line": 2,
    "column": 1,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 3,
    "column": 5,
    "initializer": "arguments[12]"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/193-implement-arguments.md",
    "title": "Implement Arguments",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/311-fix-test262-arguments-object-index-assignment.md",
    "title": "Fix test262 arguments object index assignment semantics",
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
        kind: Function,
        span: Span {
            start: 17,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 27,
            end: 28,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 37,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 43,
            end: 52,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Number(
            12,
        ),
        span: Span {
            start: 53,
            end: 55,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 67,
            end: 69,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 70,
            end: 79,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Arrow,
        spa
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Function {
        name: "f",
        params: [],
        body: [
            Let {
                name: "x",
                expr: Index {
                    object: Ident {
                        name: "arguments",
                        span: Span {
                            start: 43,
                            end: 52,
                        },
                    },
                    index: Number {
                        value: 12,
                        span: Span {
                            start: 53,
                            end: 55,
                        },
                    },
                    span: Span {
                        start: 43,
                        end: 56,
                    },
                },
                span: Span {
                    start: 37,
                    end: 57,
                },
            },
            Expr {
                expr: Call {
                    callee: ArrowFn {
                        params: [],
                        body: Ident {
                            name: "arguments",
                            span: Span {
                                start: 70,
                                end: 79,
                            },
                        },
                        span: Span {
                            start: 64,
                            end: 79,
                        },
                    },
                    args: [],
                    span: Span {
                        start: 64,
                        end: 82,
                    },
                },
                span: Span {
                    start: 64,
                    end: 83,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 17,
            end: 83,
        },
    },
    Expr {
        expr: Call {
            callee: ArrowFn {
                params: [],
                body: Ident {
                    name: "arguments",
                    span: Span {
                        start: 97,
                        end: 106,
                    },
                },
                span: Span {
                    start: 91,
                    end: 106,
                },
            },
            args: [],
            span: Span {
                start: 91,
                end: 109,
            },
        },
        span: Span {
            start: 91,
            end: 110,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-062d: `arguments` is only supported inside non-arrow functions in this milestone at 97..106
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
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 97,
        "length": 9,
        "line": 7,
        "character": 8
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 153,
        "length": 9,
        "line": 10,
        "character": 25
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 194,
        "length": 9,
        "line": 11,
        "character": 23
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 233,
        "length": 9,
        "line": 12,
        "character": 19
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 274,
        "length": 9,
        "line": 13,
        "character": 23
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 326,
        "length": 9,
        "line": 14,
        "character": 34
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 26,
        "length": 1,
        "line": 2,
        "character": 10,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 41,
        "length": 1,
        "line": 3,
        "character": 9,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 140,
        "length": 4,
        "line": 10,
        "character": 12,
        "name": "args"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 181,
        "length": 4,
        "line": 11,
        "character": 10,
        "name": "args"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 220,
        "length": 4,
        "line": 12,
        "character": 6,
        "name": "args"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 261,
        "length": 4,
        "line": 13,
        "character": 10,
        "name": "args"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 313,
        "length": 4,
        "line": 14,
        "character": 21,
        "name": "args"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function f() {\r\n    var x=arguments[12];\r\n    (() => arguments)();\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "(() => arguments)();",
        "line": 7,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface I {\r\n    method(args: typeof arguments): void;\r\n    fn: (args: typeof arguments) => void;\r\n    (args: typeof a",
        "line": 9,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function f() {\r\n    var x=arguments[12];\r\n    (() => arguments)();\r\n}\r\n\r\n(() => arguments)();\r\n\r\ninterface I {\r\n    meth",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "(() => arguments)();",
        "line": 7,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "(() => arguments)()",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ParenthesizedExpression",
        "text": "(() => arguments)",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ArrowFunction",
        "text": "() => arguments",
        "line": 7,
        "character": 2
      },
      {
        "kind": "Identifier",
        "text": "arguments",
        "line": 7,
        "character": 8
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-062d: `arguments` is only supported inside non-arrow functions in this milestone at 97..106
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
