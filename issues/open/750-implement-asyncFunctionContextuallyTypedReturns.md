---
id: 750
title: "Implement Asyncfunctioncontextuallytypedreturns"
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

Triage asyncFunctionContextuallyTypedReturns across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncFunctionContextuallyTypedReturns` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncFunctionContextuallyTypedReturns has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts
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

- `reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: asyncFunctionContextuallyTypedReturns

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 975,
  "lines": 33,
  "extension": ".ts",
  "first_code_line": "declare function f(cb: (v: boolean) => [0] | PromiseLike<[0]>): void;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 145, end: 150 } }) at 151..152",
  "span_start": 151,
  "span_end": 152,
  "line": 5,
  "column": 13,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @strict: true
3 | declare function f(cb: (v: boolean) => [0] | PromiseLike<[0]>): void;
4 | f(v => v ? [0] : Promise.reject());
5 | f(async v => v ? [0] : Promise.reject());
6 | 
7 | declare function g(cb: (v: boolean) => "contextuallyTypable" | PromiseLike<"contextuallyTypable">): void;
8 | g(v => v ? "contextuallyTypable" : Promise.reject());
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "f",
    "line": 3,
    "column": 9,
    "params": "cb: (v: boolean"
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 35,
            end: 42,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 43,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "cb",
        ),
        span: Span {
            start: 54,
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
        kind: LeftParen,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "v",
        ),
        span: Span {
            start: 59,
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
        kind: Ident(
            "boolean",
        ),
        span: Span {
            start: 62,
            end: 69,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 71,
            end: 73,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Pipe,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Ident(
            "PromiseLike",
        ),
        span: Span {
            start: 80,
            end: 91,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 99,
            end: 103,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 103,
            end: 104,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 145, end: 150 } }) at 151..152
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 145, end: 150 } }) at 151..152
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
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 52,
        "length": 1,
        "line": 3,
        "character": 18,
        "name": "f"
      },
      {
        "kind": "parameter",
        "typeText": "(v: boolean) => [0] | PromiseLike<[0]>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 54,
        "length": 2,
        "line": 3,
        "character": 20,
        "name": "cb"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 59,
        "length": 1,
        "line": 3,
        "character": 25,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 108,
        "length": 1,
        "line": 4,
        "character": 3,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 151,
        "length": 1,
        "line": 5,
        "character": 9,
        "name": "v"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 205,
        "length": 1,
        "line": 7,
        "character": 18,
        "name": "g"
      },
      {
        "kind": "parameter",
        "typeText": "(v: boolean) => \"contextuallyTypable\" | PromiseLike<\"contextuallyTypable\">",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 207,
        "length": 2,
        "line": 7,
        "character": 20,
        "name": "cb"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 212,
        "length": 1,
        "line": 7,
        "character": 25,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 297,
        "length": 1,
        "line": 8,
        "character": 3,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 358,
        "length": 1,
        "line": 9,
        "character": 9,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 432,
        "length": 5,
        "line": 11,
        "character": 20,
        "name": "thing"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 474,
        "length": 1,
        "line": 12,
        "character": 18,
        "name": "h"
      },
      {
        "kind": "parameter",
        "typeText": "(v: boolean) => MyCallback | PromiseLike<MyCallback>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 476,
        "length": 2,
        "line": 12,
        "character": 20,
        "name": "cb"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 481,
        "length": 1,
        "line": 12,
        "character": 25,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 544,
        "length": 1,
        "line": 13,
        "character": 3,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 554,
        "length": 3,
        "line": 13,
        "character": 13,
        "name": "abc"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 596,
        "length": 1,
        "line": 14,
        "character": 9,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 606,
        "length": 3,
        "line": 14,
        "character": 19,
        "name": "def"
      },
      {
        "kind": "binding",
        "typeText": "(num: number, str: string) => string | Promise<string | ((s: string) => any)>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 670,
        "length": 9,
        "line": 17,
        "character": 7,
        "name": "increment"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 686,
        "length": 3,
        "line": 18,
        "character": 3,
        "name": "num"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 702,
        "length": 3,
        "line": 19,
        "character": 3,
        "name": "str"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 730,
        "length": 1,
        "line": 20,
        "character": 16,
        "name": "s"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 777,
        "length": 3,
        "line": 20,
        "character": 63,
        "name": "num"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncFunctionContextuallyTypedReturns.ts",
        "start": 782,
        "length": 3,
        "line": 20,
        "character": 68,
        "name": "str"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 145, end: 150 } }) at 151..152
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
