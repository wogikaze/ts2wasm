---
id: 726
title: "Implement Assignmentcompatbug"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assignmentCompatBug across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentCompatBug` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompatBug has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignmentCompatBug3

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 474,
  "lines": 26,
  "extension": ".ts",
  "first_code_line": "function makePoint(x: number, y: number) {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Colon, got Some(Ident(\"x\")) at 90..91",
  "span_start": 90,
  "span_end": 91,
  "line": 4,
  "column": 16,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | function makePoint(x: number, y: number) {
3 |     return {
4 |         get x() { return x;}, // shouldn't be "void"
5 |         get y() { return y;}, // shouldn't be "void"
6 |         //x: "yo",
7 |         //y: "boo",
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "makePoint",
    "line": 2,
    "column": 1,
    "params": "x: number, y: number"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/442-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/464-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/550-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/663-implement-arrayAssignmentTest-parser-syntax.md",
    "title": "Implement Arrayassignmenttest Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/243-implement-numeric-literal-separator-parser.md",
    "title": "Implement numeric literal separator parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/244-implement-bigint-literal-parser-classification.md",
    "title": "Implement BigInt literal parser classification",
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
            start: 20,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "makePoint",
        ),
        span: Span {
            start: 29,
            end: 38,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 42,
            end: 48,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 53,
            end: 59,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 68,
            end: 74,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 86,
            end: 89,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
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
        kind: LeftBrace,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 96,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 140,
            end: 143,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("x")) at 90..91
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("x")) at 90..91
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [
      {
        "kind": "function",
        "typeText": "{ readonly x: number; readonly y: number; dist: () => number; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts",
        "start": 29,
        "length": 9,
        "line": 2,
        "character": 10,
        "name": "makePoint"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts",
        "start": 39,
        "length": 1,
        "line": 2,
        "character": 20,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts",
        "start": 50,
        "length": 1,
        "line": 2,
        "character": 31,
        "name": "y"
      },
      {
        "kind": "binary-expression",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts",
        "start": 276,
        "length": 7,
        "line": 9,
        "character": 21,
        "operator": "+",
        "leftType": "number",
        "rightType": "number",
        "candidate": "number-add-fast-path"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts",
        "start": 426,
        "length": 3,
        "line": 20,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts",
        "start": 430,
        "length": 4,
        "line": 20,
        "character": 14,
        "name": "test"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts",
        "start": 455,
        "length": 1,
        "line": 22,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts",
        "start": 468,
        "length": 1,
        "line": 23,
        "character": 5,
        "name": "y"
      },
      {
        "kind": "binary-expression",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatBug3.ts",
        "start": 492,
        "length": 5,
        "line": 26,
        "character": 5,
        "operator": "+",
        "leftType": "any",
        "rightType": "any"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function makePoint(x: number, y: number) {\r\n    return {\r\n        get x() { return x;}, // shouldn't be \"void\"\r\n        ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n    get x() {\r\n        return 0;\r\n    }\r\n}",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo(test: string) { }",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x: any;",
        "line": 22,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var y: any;",
        "line": 23,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo(x);",
        "line": 25,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo(x + y);",
        "line": 26,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function makePoint(x: number, y: number) {\r\n    return {\r\n        get x() { return x;}, // shouldn't be \"void\"\r\n        ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function makePoint(x: number, y: number) {\r\n    return {\r\n        get x() { return x;}, // shouldn't be \"void\"\r\n        ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    return {\r\n        get x() { return x;}, // shouldn't be \"void\"\r\n        get y() { return y;}, // shouldn't be \"vo",
        "line": 2,
        "character": 42
      },
      {
        "kind": "ReturnStatement",
        "text": "return {\r\n        get x() { return x;}, // shouldn't be \"void\"\r\n        get y() { return y;}, // shouldn't be \"void\"\r\n  ",
        "line": 3,
        "character": 5
      },
      {
        "kind": "ObjectLiteralExpression",
        "text": "{\r\n        get x() { return x;}, // shouldn't be \"void\"\r\n        get y() { return y;}, // shouldn't be \"void\"\r\n        /",
        "line": 3,
        "character": 12
      },
      {
        "kind": "GetAccessor",
        "text": "get x() { return x;}",
        "line": 4,
        "character": 9
      },
      {
        "kind": "Identifier",
        "text": "x",
        "line": 4,
        "character": 13
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("x")) at 90..91
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
