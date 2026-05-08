---
id: 651
title: "Implement Argumentsobjectiterator"
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

Triage argumentsObjectIterator across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail in directory `argumentsObjectIterator` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsObjectIterator has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts --detail
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
mise run reference-coverage -- tsc --limit 8
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts
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

- `reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts`
- `reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES5.ts`
- `reference/typescript/tests/cases/compiler/argumentsObjectIterator02_ES5.ts`
- `reference/typescript/tests/cases/compiler/argumentsObjectIterator02_ES6.ts`

## Duplicate detection

- `issues/done/197-implement-argumentsObjectIterator.md` - Implement Argumentsobjectiterator (same reference path, same feature label, same group key, title overlap)
- `issues/done/412-implement-arguments-object.md` - Implement arguments-object support (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage arguments object: argumentsObjectIterator01 ES6

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 238,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "function doubleAndReturnAsArray(x: number, y: number, z: number): [number, number, number] {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 219, end: 220 } }) at 220..221",
  "span_start": 220,
  "span_end": 221,
  "line": 8,
  "column": 20,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
5 |     for (let arg of arguments) {
6 |         result.push(arg + arg);
7 |     }
8 |     return <[any, any, any]>result;
9 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "doubleAndReturnAsArray",
    "line": 3,
    "column": 1,
    "params": "x: number, y: number, z: number"
  },
  {
    "kind": "binding",
    "name": "result",
    "line": 4,
    "column": 5,
    "initializer": "[]"
  },
  {
    "kind": "binding",
    "name": "arg",
    "line": 5,
    "column": 10
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/197-implement-argumentsObjectIterator.md",
    "title": "Implement Argumentsobjectiterator",
    "reason": "same reference path, same feature label"
  },
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
            start: 18,
            end: 26,
        },
    },
    SpannedToken {
        kind: Ident(
            "doubleAndReturnAsArray",
        ),
        span: Span {
            start: 27,
            end: 49,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
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
        kind: Comma,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 64,
            end: 70,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "z",
        ),
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 75,
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
        kind: Colon,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 85,
            end: 91,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 93,
            end: 99,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 101,
            end: 107,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 116,
            end: 119,
        },
    },
    SpannedToken {
        kind: Ident(
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 219, end: 220 } }) at 220..221
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 219, end: 220 } }) at 220..221
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
        "message": "Argument of type 'any' is not assignable to parameter of type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts",
        "start": 188,
        "length": 9,
        "line": 6,
        "character": 21
      },
      {
        "code": 2352,
        "category": "Error",
        "message": "Conversion of type 'never[]' to type '[any, any, any]' may be a mistake because neither type sufficiently overlaps with the other. If this was intentional, convert the expression to 'unknown' first.\n  Target requires 3 element(s) but source may have fewer.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts",
        "start": 219,
        "length": 23,
        "line": 8,
        "character": 12
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "[number, number, number]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts",
        "start": 27,
        "length": 22,
        "line": 3,
        "character": 10,
        "name": "doubleAndReturnAsArray"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts",
        "start": 50,
        "length": 1,
        "line": 3,
        "character": 33,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts",
        "start": 61,
        "length": 1,
        "line": 3,
        "character": 44,
        "name": "y"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts",
        "start": 72,
        "length": 1,
        "line": 3,
        "character": 55,
        "name": "z"
      },
      {
        "kind": "binding",
        "typeText": "never[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts",
        "start": 120,
        "length": 6,
        "line": 4,
        "character": 9,
        "name": "result"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts",
        "start": 147,
        "length": 3,
        "line": 5,
        "character": 14,
        "name": "arg"
      },
      {
        "kind": "binary-expression",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsObjectIterator01_ES6.ts",
        "start": 188,
        "length": 9,
        "line": 6,
        "character": 21,
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
        "text": "function doubleAndReturnAsArray(x: number, y: number, z: number): [number, number, number] {\r\n    let result = [];\r\n    ",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function doubleAndReturnAsArray(x: number, y: number, z: number): [number, number, number] {\r\n    let result = [];\r\n    ",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function doubleAndReturnAsArray(x: number, y: number, z: number): [number, number, number] {\r\n    let result = [];\r\n    ",
        "line": 3,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    let result = [];\r\n    for (let arg of arguments) {\r\n        result.push(arg + arg);\r\n    }\r\n    return <[any, any",
        "line": 3,
        "character": 92
      },
      {
        "kind": "ReturnStatement",
        "text": "return <[any, any, any]>result;",
        "line": 8,
        "character": 5
      },
      {
        "kind": "TypeAssertionExpression",
        "text": "<[any, any, any]>result",
        "line": 8,
        "character": 12
      },
      {
        "kind": "TupleType",
        "text": "[any, any, any]",
        "line": 8,
        "character": 13
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 219, end: 220 } }) at 220..221
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
