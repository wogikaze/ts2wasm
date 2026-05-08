---
id: 647
title: "Implement Argumentsaspropertyname Arguments Object"
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

Triage argumentsAsPropertyName-arguments-object across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsAsPropertyName-arguments-object` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsAsPropertyName-arguments-object has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts
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

- `reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts`

## Duplicate detection

- `issues/open/194-implement-argumentsAsPropertyName.md` - Implement Argumentsaspropertyname (same reference path, same feature label, title overlap)
- `issues/open/311-fix-test262-arguments-object-index-assignment.md` - Fix test262 arguments object index assignment semantics (same feature label, title overlap)

## Smart triage

### Smart triage: Triage arguments object: argumentsAsPropertyName2

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 264,
  "lines": 13,
  "extension": ".ts",
  "first_code_line": "function foo() {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Colon, got Some(RightBrace) at 225..226",
  "span_start": 225,
  "span_end": 226,
  "line": 11,
  "column": 6,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 8 |         [].forEach(function () { i });
 9 |         ({ arguments: 0 });
10 |         ({ arguments });
11 |         ({ arguments: arguments });
12 |     }
13 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "foo",
    "line": 5,
    "column": 1,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 6,
    "column": 10,
    "initializer": "0"
  },
  {
    "kind": "binding",
    "name": "i",
    "line": 7,
    "column": 9
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/194-implement-argumentsAsPropertyName.md",
    "title": "Implement Argumentsaspropertyname",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/311-fix-test262-arguments-object-index-assignment.md",
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
    "state": "done",
    "path": "issues/open/291-provide-object-global-binding-for-test262.md",
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
            start: 57,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 66,
            end: 69,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: For,
        span: Span {
            start: 79,
            end: 82,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 84,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 88,
            end: 89,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: Increment,
        span: Span {
            start: 102,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 118,
            end: 121,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 126,
            end: 132,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 143,
            end: 144,
        }
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(RightBrace) at 225..226
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(RightBrace) at 225..226
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
        "code": 2454,
        "category": "Error",
        "message": "Variable 'i' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts",
        "start": 168,
        "length": 1,
        "line": 8,
        "character": 34
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts",
        "start": 66,
        "length": 3,
        "line": 5,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts",
        "start": 88,
        "length": 1,
        "line": 6,
        "character": 14,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName2.ts",
        "start": 122,
        "length": 1,
        "line": 7,
        "character": 13,
        "name": "i"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function foo() {\r\n    for (let x = 0; x < 1; ++x) {\r\n        let i : number;\r\n        [].forEach(function () { i });\r\n  ",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function foo() {\r\n    for (let x = 0; x < 1; ++x) {\r\n        let i : number;\r\n        [].forEach(function () { i });\r\n  ",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo() {\r\n    for (let x = 0; x < 1; ++x) {\r\n        let i : number;\r\n        [].forEach(function () { i });\r\n  ",
        "line": 5,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    for (let x = 0; x < 1; ++x) {\r\n        let i : number;\r\n        [].forEach(function () { i });\r\n        ({ argume",
        "line": 5,
        "character": 16
      },
      {
        "kind": "ForStatement",
        "text": "for (let x = 0; x < 1; ++x) {\r\n        let i : number;\r\n        [].forEach(function () { i });\r\n        ({ arguments: 0 ",
        "line": 6,
        "character": 5
      },
      {
        "kind": "Block",
        "text": "{\r\n        let i : number;\r\n        [].forEach(function () { i });\r\n        ({ arguments: 0 });\r\n        ({ arguments })",
        "line": 6,
        "character": 33
      },
      {
        "kind": "ExpressionStatement",
        "text": "({ arguments });",
        "line": 10,
        "character": 9
      },
      {
        "kind": "ParenthesizedExpression",
        "text": "({ arguments })",
        "line": 10,
        "character": 9
      },
      {
        "kind": "ObjectLiteralExpression",
        "text": "{ arguments }",
        "line": 10,
        "character": 10
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Colon, got Some(RightBrace) at 225..226
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
