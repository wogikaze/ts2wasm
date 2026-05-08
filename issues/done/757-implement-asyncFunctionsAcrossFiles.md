---
id: 757
title: "Implement Asyncfunctionsacrossfiles"
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

Triage asyncFunctionsAcrossFiles across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncFunctionsAcrossFiles` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncFunctionsAcrossFiles has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts
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

- `reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: asyncFunctionsAcrossFiles

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 243,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "import { b } from './b';"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 90, end: 95 } }) at 96..97",
  "span_start": 96,
  "span_end": 97,
  "line": 5,
  "column": 18,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @filename: a.ts
3 | import { b } from './b';
4 | export const a = {
5 |     f: async () => {
6 |         await b.f();
7 |     }
8 | };
```

Visible symbols before failure:

```json
[
  {
    "kind": "import",
    "name": "./b",
    "line": 3,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "a",
    "line": 4,
    "column": 8,
    "initializer": "{"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md",
    "title": "Implement Arraytolocalestringes Unknown Unsupported",
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
        kind: Import,
        span: Span {
            start: 37,
            end: 43,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "from",
        ),
        span: Span {
            start: 50,
            end: 54,
        },
    },
    SpannedToken {
        kind: String(
            "./b",
        ),
        span: Span {
            start: 55,
            end: 60,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 63,
            end: 69,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 70,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 88,
            end: 89,
        },
    },
    SpannedToken {
        kind: Async,
        span: Span {
            start: 90,
            end: 95,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 99,
            end: 101,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Await,
        span: Span {
            start: 113,
            end: 118,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 90, end: 95 } }) at 96..97
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 90, end: 95 } }) at 96..97
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
        "code": 2395,
        "category": "Error",
        "message": "Individual declarations in merged declaration 'b' must be all exported or all local.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts",
        "start": 46,
        "length": 1,
        "line": 3,
        "character": 10
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './b' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts",
        "start": 55,
        "length": 5,
        "line": 3,
        "character": 19
      },
      {
        "code": 2395,
        "category": "Error",
        "message": "Individual declarations in merged declaration 'a' must be all exported or all local.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts",
        "start": 76,
        "length": 1,
        "line": 4,
        "character": 14
      },
      {
        "code": 2395,
        "category": "Error",
        "message": "Individual declarations in merged declaration 'a' must be all exported or all local.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts",
        "start": 167,
        "length": 1,
        "line": 10,
        "character": 10
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './a' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts",
        "start": 176,
        "length": 5,
        "line": 10,
        "character": 19
      },
      {
        "code": 2395,
        "category": "Error",
        "message": "Individual declarations in merged declaration 'b' must be all exported or all local.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts",
        "start": 197,
        "length": 1,
        "line": 11,
        "character": 14
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ f: () => Promise<void>; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts",
        "start": 76,
        "length": 1,
        "line": 4,
        "character": 14,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "{ f: () => Promise<void>; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionsAcrossFiles.ts",
        "start": 197,
        "length": 1,
        "line": 11,
        "character": 14,
        "name": "b"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ImportDeclaration",
        "text": "import { b } from './b';",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export const a = {\r\n    f: async () => {\r\n        await b.f();\r\n    }\r\n};",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import { a } from './a';",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export const b = {\r\n    f: async () => {\r\n        await a.f();\r\n    }\r\n};",
        "line": 11,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "import { b } from './b';\r\nexport const a = {\r\n    f: async () => {\r\n        await b.f();\r\n    }\r\n};\r\n// @filename: b.ts\r",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export const a = {\r\n    f: async () => {\r\n        await b.f();\r\n    }\r\n};",
        "line": 4,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const a = {\r\n    f: async () => {\r\n        await b.f();\r\n    }\r\n}",
        "line": 4,
        "character": 8
      },
      {
        "kind": "VariableDeclaration",
        "text": "a = {\r\n    f: async () => {\r\n        await b.f();\r\n    }\r\n}",
        "line": 4,
        "character": 14
      },
      {
        "kind": "ObjectLiteralExpression",
        "text": "{\r\n    f: async () => {\r\n        await b.f();\r\n    }\r\n}",
        "line": 4,
        "character": 18
      },
      {
        "kind": "PropertyAssignment",
        "text": "f: async () => {\r\n        await b.f();\r\n    }",
        "line": 5,
        "character": 5
      },
      {
        "kind": "ArrowFunction",
        "text": "async () => {\r\n        await b.f();\r\n    }",
        "line": 5,
        "character": 8
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 90, end: 95 } }) at 96..97
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
