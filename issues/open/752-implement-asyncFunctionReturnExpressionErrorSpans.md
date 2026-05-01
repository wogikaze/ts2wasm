---
id: 752
title: "Implement Asyncfunctionreturnexpressionerrorspans"
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

Triage asyncFunctionReturnExpressionErrorSpans across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncFunctionReturnExpressionErrorSpans` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncFunctionReturnExpressionErrorSpans has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnExpressionErrorSpans.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionReturnExpressionErrorSpans.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionReturnExpressionErrorSpans.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnExpressionErrorSpans.ts
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

- `reference/typescript/tests/cases/compiler/asyncFunctionReturnExpressionErrorSpans.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage runtime subset: asyncFunctionReturnExpressionErrorSpans

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/asyncFunctionReturnExpressionErrorSpans.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnExpressionErrorSpans.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 337,
  "lines": 22,
  "extension": ".ts",
  "first_code_line": "interface Foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedRuntimeSubset",
  "message": "issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 156..170",
  "span_start": 156,
  "span_end": 170,
  "line": 12,
  "column": 12,
  "feature_label": "runtime-subset",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 9 |     }
10 | }
11 | 
12 | async function asyncFoo(): Promise<Foo> {
13 |     return {
14 |         bar: {
15 |             baz: {
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[]
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
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 20,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 30,
            end: 33,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 34,
            end: 35,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 41,
            end: 44,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "baz",
        ),
        span: Span {
            start: 57,
            end: 60,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Ident(
            "inner",
        ),
        span: Span {
            start: 77,
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
        kind: LeftBrace,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "thing",
        ),
        span: Span {
            start: 103,
            end: 108,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 110,
            end: 116,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 141,
            end: 142,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 151,
            end: 152,
        },
    },
    SpannedToken {
        kind: Async,
        span: Span {
            start: 156,
            end: 161,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 162,
            end: 170,
        },
    },
    SpannedToken {
        kind: Ident(
            "asyncFoo",
        ),
        span: Span {
            start: 171,
            end: 179,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 179,
            end: 180,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 180,
            end: 181,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 181,
            end: 182,
        },
    },
    SpannedToken {
        kind: Ident(
            "Promise",
        ),
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 156..170
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 156..170
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnExpressionErrorSpans.ts",
        "start": 295,
        "length": 5,
        "line": 17,
        "character": 21
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "Promise<Foo>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnExpressionErrorSpans.ts",
        "start": 171,
        "length": 8,
        "line": 12,
        "character": 16,
        "name": "asyncFoo"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface Foo {\r\n    bar: {\r\n        baz: {\r\n            inner: {\r\n                thing: string\r\n            }\r\n       ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function asyncFoo(): Promise<Foo> {\r\n    return {\r\n        bar: {\r\n            baz: {\r\n                inner: {\r\n ",
        "line": 12,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface Foo {\r\n    bar: {\r\n        baz: {\r\n            inner: {\r\n                thing: string\r\n            }\r\n       ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function asyncFoo(): Promise<Foo> {\r\n    return {\r\n        bar: {\r\n            baz: {\r\n                inner: {\r\n ",
        "line": 12,
        "character": 1
      },
      {
        "kind": "AsyncKeyword",
        "text": "async",
        "line": 12,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 156..170
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
