---
id: 756
title: "Implement Asyncfunctionwithforstatementnoinitializer"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage asyncFunctionWithForStatementNoInitializer across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncFunctionWithForStatementNoInitializer` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncFunctionWithForStatementNoInitializer has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts
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

- `reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage runtime subset: asyncFunctionWithForStatementNoInitializer

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 330,
  "lines": 25,
  "extension": ".ts",
  "first_code_line": "async function test1() {"
}
```

Failure location:

```json
{
  "code": "UnsupportedRuntimeSubset",
  "message": "issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 20..34",
  "span_start": 20,
  "span_end": 34,
  "line": 2,
  "column": 2,
  "feature_label": "runtime-subset",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | async function test1() {
3 |     let i = 0
4 |     let limit = 10
5 |     for (; i < limit; ++i) {
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/754-implement-asyncFunctionReturnType-runtime-subset.md",
    "title": "Implement Asyncfunctionreturntype Runtime Subset",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Async,
        span: Span {
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 26,
            end: 34,
        },
    },
    SpannedToken {
        kind: Ident(
            "test1",
        ),
        span: Span {
            start: 35,
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
        kind: RightParen,
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 50,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 65,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "limit",
        ),
        span: Span {
            start: 69,
            end: 74,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Number(
            10,
        ),
        span: Span {
            start: 77,
            end: 79,
        },
    },
    SpannedToken {
        kind: For,
        span: Span {
            start: 85,
            end: 88,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "limit",
        ),
        span: Span {
            start: 96,
            end: 101,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: Increment,
        span: Span {
            start: 103,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 115,
            end: 116,
        },
    },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 20..34
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 20..34
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
        "typeText": "Promise<void>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts",
        "start": 35,
        "length": 5,
        "line": 2,
        "character": 16,
        "name": "test1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts",
        "start": 54,
        "length": 1,
        "line": 3,
        "character": 9,
        "name": "i"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts",
        "start": 69,
        "length": 5,
        "line": 4,
        "character": 9,
        "name": "limit"
      },
      {
        "kind": "function",
        "typeText": "Promise<void>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts",
        "start": 138,
        "length": 5,
        "line": 9,
        "character": 16,
        "name": "test2"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts",
        "start": 157,
        "length": 1,
        "line": 10,
        "character": 9,
        "name": "i"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts",
        "start": 172,
        "length": 5,
        "line": 11,
        "character": 9,
        "name": "limit"
      },
      {
        "kind": "function",
        "typeText": "Promise<void>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts",
        "start": 246,
        "length": 5,
        "line": 16,
        "character": 16,
        "name": "test3"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts",
        "start": 265,
        "length": 1,
        "line": 17,
        "character": 9,
        "name": "i"
      },
      {
        "kind": "function",
        "typeText": "Promise<void>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionWithForStatementNoInitializer.ts",
        "start": 319,
        "length": 5,
        "line": 22,
        "character": 16,
        "name": "test4"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "async function test1() {\r\n    let i = 0\r\n    let limit = 10\r\n    for (; i < limit; ++i) {\r\n    }\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function test2() {\r\n    let i = 0\r\n    let limit = 10\r\n    for (i = 1; i < limit; ++i) {\r\n    }\r\n}",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function test3() {\r\n    let i = 0\r\n    for (;; ++i) {\r\n    }\r\n}",
        "line": 16,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function test4() {\r\n    for (;;) {\r\n    }\r\n}",
        "line": 22,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "async function test1() {\r\n    let i = 0\r\n    let limit = 10\r\n    for (; i < limit; ++i) {\r\n    }\r\n}\r\n\r\nasync function te",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function test1() {\r\n    let i = 0\r\n    let limit = 10\r\n    for (; i < limit; ++i) {\r\n    }\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "AsyncKeyword",
        "text": "async",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 20..34
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
