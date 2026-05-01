---
id: 754
title: "Implement Asyncfunctionreturntype Runtime Subset"
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

Triage asyncFunctionReturnType-runtime-subset across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncFunctionReturnType-runtime-subset` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncFunctionReturnType-runtime-subset has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts
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

- `reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage runtime subset: asyncFunctionReturnType

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 2638,
  "lines": 76,
  "extension": ".ts",
  "first_code_line": "async function fAsync() {"
}
```

Failure location:

```json
{
  "code": "UnsupportedRuntimeSubset",
  "message": "issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 17..31",
  "span_start": 17,
  "span_end": 31,
  "line": 2,
  "column": 2,
  "feature_label": "runtime-subset",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: ES6
2 | async function fAsync() {
3 |     // Without explicit type annotation, this is just an array.
4 |     return [1, true];
5 | }
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
            start: 17,
            end: 22,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 23,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "fAsync",
        ),
        span: Span {
            start: 32,
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
        kind: RightParen,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 113,
            end: 119,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: True,
        span: Span {
            start: 124,
            end: 128,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Async,
        span: Span {
            start: 137,
            end: 142,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 143,
            end: 151,
        },
    },
    SpannedToken {
        kind: Ident(
            "fAsyncExplicit",
        ),
        span: Span {
            start: 152,
            end: 166,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 167,
            end: 168,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: Ident(
            "Promise",
        ),
        span: Span {
            start: 170,
            end: 177,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 177,
            end: 178,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 178,
            end: 179,
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
        kind: Comma,
        span: Span {
            start: 185,
            end: 186,
        },
    },
    SpannedToken {
        kind: Ident(
            "boolean",
        ),
        span: Span {
            start: 187,
            end: 194,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 17..31
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 17..31
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
        "typeText": "Promise<(number | boolean)[]>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 32,
        "length": 6,
        "line": 2,
        "character": 16,
        "name": "fAsync"
      },
      {
        "kind": "function",
        "typeText": "Promise<[number, boolean]>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 152,
        "length": 14,
        "line": 7,
        "character": 16,
        "name": "fAsyncExplicit"
      },
      {
        "kind": "function",
        "typeText": "Promise<string>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 413,
        "length": 25,
        "line": 18,
        "character": 16,
        "name": "fIndexedTypeForStringProp"
      },
      {
        "kind": "parameter",
        "typeText": "Obj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 439,
        "length": 3,
        "line": 18,
        "character": 42,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<string>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 528,
        "length": 34,
        "line": 22,
        "character": 16,
        "name": "fIndexedTypeForPromiseOfStringProp"
      },
      {
        "kind": "parameter",
        "typeText": "Obj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 563,
        "length": 3,
        "line": 22,
        "character": 51,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<string>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 669,
        "length": 42,
        "line": 26,
        "character": 16,
        "name": "fIndexedTypeForExplicitPromiseOfStringProp"
      },
      {
        "kind": "parameter",
        "typeText": "Obj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 712,
        "length": 3,
        "line": 26,
        "character": 59,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<any>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 837,
        "length": 22,
        "line": 30,
        "character": 16,
        "name": "fIndexedTypeForAnyProp"
      },
      {
        "kind": "parameter",
        "typeText": "Obj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 860,
        "length": 3,
        "line": 30,
        "character": 39,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<any>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 943,
        "length": 31,
        "line": 34,
        "character": 16,
        "name": "fIndexedTypeForPromiseOfAnyProp"
      },
      {
        "kind": "parameter",
        "typeText": "Obj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 975,
        "length": 3,
        "line": 34,
        "character": 48,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<any>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1075,
        "length": 39,
        "line": 38,
        "character": 16,
        "name": "fIndexedTypeForExplicitPromiseOfAnyProp"
      },
      {
        "kind": "parameter",
        "typeText": "Obj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1115,
        "length": 3,
        "line": 38,
        "character": 56,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<TObj[\"stringProp\"]>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1231,
        "length": 32,
        "line": 42,
        "character": 16,
        "name": "fGenericIndexedTypeForStringProp"
      },
      {
        "kind": "parameter",
        "typeText": "TObj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1282,
        "length": 3,
        "line": 42,
        "character": 67,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<TObj[\"stringProp\"]>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1373,
        "length": 41,
        "line": 46,
        "character": 16,
        "name": "fGenericIndexedTypeForPromiseOfStringProp"
      },
      {
        "kind": "parameter",
        "typeText": "TObj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1433,
        "length": 3,
        "line": 46,
        "character": 76,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<TObj[\"stringProp\"]>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1541,
        "length": 49,
        "line": 50,
        "character": 16,
        "name": "fGenericIndexedTypeForExplicitPromiseOfStringProp"
      },
      {
        "kind": "parameter",
        "typeText": "TObj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1609,
        "length": 3,
        "line": 50,
        "character": 84,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<TObj[\"anyProp\"]>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1737,
        "length": 29,
        "line": 54,
        "character": 16,
        "name": "fGenericIndexedTypeForAnyProp"
      },
      {
        "kind": "parameter",
        "typeText": "TObj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1785,
        "length": 3,
        "line": 54,
        "character": 64,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<TObj[\"anyProp\"]>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1870,
        "length": 38,
        "line": 58,
        "character": 16,
        "name": "fGenericIndexedTypeForPromiseOfAnyProp"
      },
      {
        "kind": "parameter",
        "typeText": "TObj",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionReturnType.ts",
        "start": 1927,
        "length": 3,
        "line": 58,
        "character": 73,
        "name": "obj"
      },
      {
        "kind": "function",
        "typeText": "Promise<TObj[\"anyProp\"]>",
        "file": "/h
```

Stack trace:

```text
error: [UnsupportedRuntimeSubset] issue-230: async function declarations require Promise and async iterator runtime semantics for `for await...of`, which are not supported in this milestone at 17..31
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
