---
id: 761
title: "Implement Asynciteratorextraparameters"
type: spike
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage asyncIteratorExtraParameters across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncIteratorExtraParameters` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncIteratorExtraParameters has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts
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

- `reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: asyncIteratorExtraParameters

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 325,
  "lines": 18,
  "extension": ".ts",
  "first_code_line": "const iter = {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected identifier or string literal as object key, got Some(Async) at 146..151",
  "span_start": 146,
  "span_end": 151,
  "line": 7,
  "column": 11,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 4 | 
 5 | // https://github.com/microsoft/TypeScript/issues/57130
 6 | const iter = {
 7 |     async *[Symbol.asyncIterator](_: number) {
 8 |         yield 0;
 9 |     }
10 | };
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "iter",
    "line": 6,
    "column": 1,
    "initializer": "{"
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
    "path": "issues/done/464-implement-FunctionDeclaration-parser-syntax.md",
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
    "state": "open",
    "path": "issues/open/734-implement-assignmentCompatability-parser-syntax.md",
    "title": "Implement Assignmentcompatability Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md",
    "title": "Implement Asyncfunctionreturntype Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
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
            start: 126,
            end: 131,
        },
    },
    SpannedToken {
        kind: Ident(
            "iter",
        ),
        span: Span {
            start: 132,
            end: 136,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 139,
            end: 140,
        },
    },
    SpannedToken {
        kind: Async,
        span: Span {
            start: 146,
            end: 151,
        },
    },
    SpannedToken {
        kind: Star,
        span: Span {
            start: 152,
            end: 153,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 153,
            end: 154,
        },
    },
    SpannedToken {
        kind: Ident(
            "Symbol",
        ),
        span: Span {
            start: 154,
            end: 160,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 160,
            end: 161,
        },
    },
    SpannedToken {
        kind: Ident(
            "asyncIterator",
        ),
        span: Span {
            start: 161,
            end: 174,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 174,
            end: 175,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 175,
            end: 176,
        },
    },
    SpannedToken {
        kind: Ident(
            "_",
        ),
        span: Span {
            start: 176,
            end: 177,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 177,
            end: 178,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 179,
            end: 185,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 185,
            end: 186,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 187,
            end: 188,
        },
    },
    SpannedToken {
        kind: Ident(
            "yield",
        ),
        span: Span {
            start: 198,
            end: 203,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 204,
            end: 205,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 205,
            end: 206,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 212,
            end: 213,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 215,
            end: 216,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 216,
            end: 217,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 221,
            end: 228,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 229,
            end: 237,
        },
    },
    SpannedToken {
        kind: Ident(
            "g
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected identifier or string literal as object key, got Some(Async) at 146..151
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected identifier or string literal as object key, got Some(Async) at 146..151
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
        "code": 2504,
        "category": "Error",
        "message": "Type '{ [Symbol.asyncIterator](_: number): AsyncGenerator<number, void, unknown>; }' must have a '[Symbol.asyncIterator]()' method that returns an async iterator.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts",
        "start": 312,
        "length": 4,
        "line": 15,
        "character": 27
      },
      {
        "code": 2504,
        "category": "Error",
        "message": "Type '{ [Symbol.asyncIterator](_: number): AsyncGenerator<number, void, unknown>; }' must have a '[Symbol.asyncIterator]()' method that returns an async iterator.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts",
        "start": 333,
        "length": 4,
        "line": 17,
        "character": 12
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ [Symbol.asyncIterator](_: number): AsyncGenerator<number, void, unknown>; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts",
        "start": 132,
        "length": 4,
        "line": 6,
        "character": 7,
        "name": "iter"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts",
        "start": 176,
        "length": 1,
        "line": 7,
        "character": 35,
        "name": "_"
      },
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts",
        "start": 238,
        "length": 1,
        "line": 12,
        "character": 18,
        "name": "g"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts",
        "start": 243,
        "length": 4,
        "line": 12,
        "character": 23,
        "name": "args"
      },
      {
        "kind": "function",
        "typeText": "AsyncGenerator<any, void, unknown>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts",
        "start": 279,
        "length": 1,
        "line": 14,
        "character": 17,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncIteratorExtraParameters.ts",
        "start": 307,
        "length": 1,
        "line": 15,
        "character": 22,
        "name": "_"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const iter = {\r\n    async *[Symbol.asyncIterator](_: number) {\r\n        yield 0;\r\n    }\r\n};",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function g(...args: any): any;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function* f() {\r\n    for await (const _ of iter);\r\n\r\n    yield* iter;\r\n}",
        "line": 14,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const iter = {\r\n    async *[Symbol.asyncIterator](_: number) {\r\n        yield 0;\r\n    }\r\n};\r\n\r\ndeclare function g(...arg",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const iter = {\r\n    async *[Symbol.asyncIterator](_: number) {\r\n        yield 0;\r\n    }\r\n};",
        "line": 6,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const iter = {\r\n    async *[Symbol.asyncIterator](_: number) {\r\n        yield 0;\r\n    }\r\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "iter = {\r\n    async *[Symbol.asyncIterator](_: number) {\r\n        yield 0;\r\n    }\r\n}",
        "line": 6,
        "character": 7
      },
      {
        "kind": "ObjectLiteralExpression",
        "text": "{\r\n    async *[Symbol.asyncIterator](_: number) {\r\n        yield 0;\r\n    }\r\n}",
        "line": 6,
        "character": 14
      },
      {
        "kind": "MethodDeclaration",
        "text": "async *[Symbol.asyncIterator](_: number) {\r\n        yield 0;\r\n    }",
        "line": 7,
        "character": 5
      },
      {
        "kind": "AsyncKeyword",
        "text": "async",
        "line": 7,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected identifier or string literal as object key, got Some(Async) at 146..151
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
