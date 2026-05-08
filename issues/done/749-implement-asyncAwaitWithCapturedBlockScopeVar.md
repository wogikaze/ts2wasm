---
id: 749
title: "Implement Asyncawaitwithcapturedblockscopevar"
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

Triage asyncAwaitWithCapturedBlockScopeVar across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncAwaitWithCapturedBlockScopeVar` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncAwaitWithCapturedBlockScopeVar has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts
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

- `reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage runtime subset: asyncAwaitWithCapturedBlockScopeVar

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 632,
  "lines": 37,
  "extension": ".ts",
  "first_code_line": "async function fn1() {"
}
```

Failure location:

```json
{
  "code": "UnsupportedRuntimeSubset",
  "message": "issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 64..78",
  "span_start": 64,
  "span_end": 78,
  "line": 4,
  "column": 4,
  "feature_label": "runtime-subset",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es5, es2015
2 | // @lib: es6
3 | // @noEmitHelpers: true
4 | async function fn1() {
5 |     let ar = [];
6 |     for (let i = 0; i < 1; i++) {
7 |         await 1;
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
        kind: Async,
        span: Span {
            start: 64,
            end: 69,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 70,
            end: 78,
        },
    },
    SpannedToken {
        kind: Ident(
            "fn1",
        ),
        span: Span {
            start: 79,
            end: 82,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 92,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "ar",
        ),
        span: Span {
            start: 96,
            end: 98,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: For,
        span: Span {
            start: 110,
            end: 113,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 115,
            end: 118,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
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
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 133,
            end: 134,
        },
    },
    SpannedToken {
        kind: Increment,
        span: Span {
            start: 134,
            end: 136,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    Span
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 64..78
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 64..78
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
        "message": "Argument of type '() => number' is not assignable to parameter of type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 175,
        "length": 7,
        "line": 8,
        "character": 17
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type '() => number' is not assignable to parameter of type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 309,
        "length": 7,
        "line": 16,
        "character": 17
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type '() => number' is not assignable to parameter of type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 459,
        "length": 7,
        "line": 25,
        "character": 17
      },
      {
        "code": 2366,
        "category": "Error",
        "message": "Function lacks ending return statement and return type does not include 'undefined'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 523,
        "length": 15,
        "line": 30,
        "character": 23
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type '() => number' is not assignable to parameter of type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 629,
        "length": 7,
        "line": 34,
        "character": 17
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "Promise<void>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 79,
        "length": 3,
        "line": 4,
        "character": 16,
        "name": "fn1"
      },
      {
        "kind": "binding",
        "typeText": "never[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 96,
        "length": 2,
        "line": 5,
        "character": 9,
        "name": "ar"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 119,
        "length": 1,
        "line": 6,
        "character": 14,
        "name": "i"
      },
      {
        "kind": "function",
        "typeText": "Promise<void>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 213,
        "length": 3,
        "line": 12,
        "character": 16,
        "name": "fn2"
      },
      {
        "kind": "binding",
        "typeText": "never[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 230,
        "length": 2,
        "line": 13,
        "character": 9,
        "name": "ar"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 253,
        "length": 1,
        "line": 14,
        "character": 14,
        "name": "i"
      },
      {
        "kind": "function",
        "typeText": "Promise<void>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 363,
        "length": 3,
        "line": 21,
        "character": 16,
        "name": "fn3"
      },
      {
        "kind": "binding",
        "typeText": "never[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 380,
        "length": 2,
        "line": 22,
        "character": 9,
        "name": "ar"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 403,
        "length": 1,
        "line": 23,
        "character": 14,
        "name": "i"
      },
      {
        "kind": "function",
        "typeText": "Promise<number>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 516,
        "length": 3,
        "line": 30,
        "character": 16,
        "name": "fn4"
      },
      {
        "kind": "binding",
        "typeText": "never[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 550,
        "length": 2,
        "line": 31,
        "character": 9,
        "name": "ar"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncAwaitWithCapturedBlockScopeVar.ts",
        "start": 573,
        "length": 1,
        "line": 32,
        "character": 14,
        "name": "i"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "async function fn1() {\r\n    let ar = [];\r\n    for (let i = 0; i < 1; i++) {\r\n        await 1;\r\n        ar.push(() => i);",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function fn2() {\r\n    let ar = [];\r\n    for (let i = 0; i < 1; i++) {\r\n        await 1;\r\n        ar.push(() => i);",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function fn3() {\r\n    let ar = [];\r\n    for (let i = 0; i < 1; i++) {\r\n        await 1;\r\n        ar.push(() => i);",
        "line": 21,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function fn4(): Promise<number> {\r\n    let ar = [];\r\n    for (let i = 0; i < 1; i++) {\r\n        await 1;\r\n        ",
        "line": 30,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "async function fn1() {\r\n    let ar = [];\r\n    for (let i = 0; i < 1; i++) {\r\n        await 1;\r\n        ar.push(() => i);",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function fn1() {\r\n    let ar = [];\r\n    for (let i = 0; i < 1; i++) {\r\n        await 1;\r\n        ar.push(() => i);",
        "line": 4,
        "character": 1
      },
      {
        "kind": "AsyncKeyword",
        "text": "async",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 64..78
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
