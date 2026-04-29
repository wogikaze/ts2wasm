---
id: 168
title: "Implement Ambiguousoverload"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage ambiguousOverload across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambiguousOverload` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambiguousOverload has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousOverload.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambiguousOverload.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambiguousOverload.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousOverload.ts
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

- `reference/typescript/tests/cases/compiler/ambiguousOverload.ts`

## Duplicate detection

- `issues/open/169-implement-ambiguousOverloadResolution.md` - Implement Ambiguousoverloadresolution (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: ambiguousOverload

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambiguousOverload.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousOverload.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 422,
  "lines": 13,
  "extension": ".ts",
  "first_code_line": "function foof(bar: string, y): number;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(RightBrace) at 161..162",
  "span_start": 161,
  "span_end": 162,
  "line": 6,
  "column": 2,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | function foof(bar: string, y): number;
4 | function foof(bar: string, x): string;
5 | function foof(bar: any): any { return bar };
6 | var x: number = foof("s", null);
7 | var y: string = foof("s", null);
8 |
9 | function foof2(bar: string, x): string;
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "foof",
    "line": 3,
    "column": 1,
    "params": "bar: string, y"
  },
  {
    "kind": "function",
    "name": "foof",
    "line": 4,
    "column": 1,
    "params": "bar: string, x"
  },
  {
    "kind": "function",
    "name": "foof",
    "line": 5,
    "column": 1,
    "params": "bar: any"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/168-implement-ambiguousOverload.md",
    "title": "Implement Ambiguousoverload",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
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
            start: 39,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "foof",
        ),
        span: Span {
            start: 48,
            end: 52,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 53,
            end: 56,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 58,
            end: 64,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 70,
            end: 76,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 79,
            end: 87,
        },
    },
    SpannedT
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(RightBrace) at 161..162
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(RightBrace) at 161..162
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
        "code": 2322,
        "category": "Error",
        "message": "Type 'number' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 203,
        "length": 1,
        "line": 7,
        "character": 5
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 404,
        "length": 2,
        "line": 13,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 48,
        "length": 4,
        "line": 3,
        "character": 10,
        "name": "foof"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 53,
        "length": 3,
        "line": 3,
        "character": 15,
        "name": "bar"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 66,
        "length": 1,
        "line": 3,
        "character": 28,
        "name": "y"
      },
      {
        "kind": "function",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 88,
        "length": 4,
        "line": 4,
        "character": 10,
        "name": "foof"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 93,
        "length": 3,
        "line": 4,
        "character": 15,
        "name": "bar"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 106,
        "length": 1,
        "line": 4,
        "character": 28,
        "name": "x"
      },
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 128,
        "length": 4,
        "line": 5,
        "character": 10,
        "name": "foof"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 133,
        "length": 3,
        "line": 5,
        "character": 15,
        "name": "bar"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 169,
        "length": 1,
        "line": 6,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 203,
        "length": 1,
        "line": 7,
        "character": 5,
        "name": "y"
      },
      {
        "kind": "function",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 244,
        "length": 5,
        "line": 9,
        "character": 10,
        "name": "foof2"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 250,
        "length": 3,
        "line": 9,
        "character": 16,
        "name": "bar"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 263,
        "length": 1,
        "line": 9,
        "character": 29,
        "name": "x"
      },
      {
        "kind": "function",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 285,
        "length": 5,
        "line": 10,
        "character": 10,
        "name": "foof2"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 291,
        "length": 3,
        "line": 10,
        "character": 16,
        "name": "bar"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 304,
        "length": 1,
        "line": 10,
        "character": 29,
        "name": "y"
      },
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 326,
        "length": 5,
        "line": 11,
        "character": 10,
        "name": "foof2"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 332,
        "length": 3,
        "line": 11,
        "character": 16,
        "name": "bar"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 368,
        "length": 2,
        "line": 12,
        "character": 5,
        "name": "x2"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverload.ts",
        "start": 404,
        "length": 2,
        "line": 13,
        "character": 5,
        "name": "y2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function foof(bar: string, y): number;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foof(bar: string, x): string;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foof(bar: any): any { return bar }",
        "line": 5,
        "character": 1
      },
      {
        "kind": "EmptyStatement",
        "text": ";",
        "line": 5,
        "character": 44
      },
      {
        "kind": "FirstStatement",
        "text": "var x: number = foof(\"s\", null);",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var y: string = foof(\"s\", null);",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foof2(bar: string, x): string;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foof2(bar: string, y): number;",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foof2(bar: any): an
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(RightBrace) at 161..162
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
