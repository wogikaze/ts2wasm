---
id: 676
title: "Implement Arrayfind (audit reopened #676)"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: [5125]
created: 2026-05-01
updated: 2026-05-05status: open
---

## Triage complete: child issue created

Child: #5125 (implement as type assertion expression parsing)

Root cause: Array.find() already implemented. Failure is `as` type assertion parsing on line 12. `const readonlyArrayOfStringsNumbersAndBooleans = arrayOfStringsNumbersAndBooleans as ReadonlyArray<string | number | boolean>`.

## Summary

Triage arrayFind across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayFind` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayFind has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFind.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFind.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket — #911 superseded by this issue
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues → #5125
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFind.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFind.ts
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

- `reference/typescript/tests/cases/compiler/arrayFind.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: arrayFind

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayFind.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFind.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 593,
  "lines": 13,
  "extension": ".ts",
  "first_code_line": "function isNumber(x: any): x is number {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 497, end: 498 } }) at 500..505",
  "span_start": 500,
  "span_end": 505,
  "line": 13,
  "column": 13,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
10 | const foundNumber: number | undefined = arrayOfStringsNumbersAndBooleans.find(isNumber);
11 | 
12 | const readonlyArrayOfStringsNumbersAndBooleans = arrayOfStringsNumbersAndBooleans as ReadonlyArray<string | number | boolean>;
13 | const readonlyFoundNumber: number | undefined = readonlyArrayOfStringsNumbersAndBooleans.find(isNumber);
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "isNumber",
    "line": 5,
    "column": 1,
    "params": "x: any"
  },
  {
    "kind": "binding",
    "name": "arrayOfStringsNumbersAndBooleans",
    "line": 9,
    "column": 1,
    "initializer": "[\"string\", false, 0, \"strung\", 1, true]"
  },
  {
    "kind": "binding",
    "name": "foundNumber",
    "line": 10,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "readonlyArrayOfStringsNumbersAndBooleans",
    "line": 12,
    "column": 1,
    "initializer": "arrayOfStringsNumbersAndBooleans as ReadonlyArray<string | number | boolean>"
  },
  {
    "kind": "binding",
    "name": "readon",
    "line": 13,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[]
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
            start: 117,
            end: 125,
        },
    },
    SpannedToken {
        kind: Ident(
            "isNumber",
        ),
        span: Span {
            start: 126,
            end: 134,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 138,
            end: 141,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 141,
            end: 142,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 142,
            end: 143,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "is",
        ),
        span: Span {
            start: 146,
            end: 148,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 149,
            end: 155,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 156,
            end: 157,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 161,
            end: 167,
        },
    },
    SpannedToken {
        kind: TypeOf,
        span: Span {
            start: 168,
            end: 174,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 175,
            end: 176,
        },
    },
    SpannedToken {
        kind: StrictEqual,
        span: Span {
            start: 177,
            end: 180,
        },
    },
    SpannedToken {
        kind: String(
            "number",
        ),
        span: Span {
            start: 181,
            end: 189,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 189,
            end: 190,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 192,
            end: 193,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 197,
            end: 202,
        },
    },
    SpannedToken {
        kind: Ident(
            "arrayOfStringsNumbersAndBooleans",
        ),
        span: Span {
            start: 203,
            end: 235,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 236,
            end: 237,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 238,
            end: 239,
        },
    },
    SpannedToken {
        kind: String(
            "string",
        ),
        span: Span {
            start: 239,
            end: 247,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 247,
            end: 248,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 497, end: 498 } }) at 500..505
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 497, end: 498 } }) at 500..505
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
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFind.ts",
        "start": 126,
        "length": 8,
        "line": 5,
        "character": 10,
        "name": "isNumber"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFind.ts",
        "start": 135,
        "length": 1,
        "line": 5,
        "character": 19,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "(string | number | boolean)[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFind.ts",
        "start": 203,
        "length": 32,
        "line": 9,
        "character": 7,
        "name": "arrayOfStringsNumbersAndBooleans"
      },
      {
        "kind": "binding",
        "typeText": "number | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFind.ts",
        "start": 286,
        "length": 11,
        "line": 10,
        "character": 7,
        "name": "foundNumber"
      },
      {
        "kind": "binding",
        "typeText": "readonly (string | number | boolean)[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFind.ts",
        "start": 378,
        "length": 40,
        "line": 12,
        "character": 7,
        "name": "readonlyArrayOfStringsNumbersAndBooleans"
      },
      {
        "kind": "binding",
        "typeText": "number | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFind.ts",
        "start": 506,
        "length": 19,
        "line": 13,
        "character": 7,
        "name": "readonlyFoundNumber"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function isNumber(x: any): x is number {\r\n  return typeof x === \"number\";\r\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const arrayOfStringsNumbersAndBooleans = [\"string\", false, 0, \"strung\", 1, true];",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const foundNumber: number | undefined = arrayOfStringsNumbersAndBooleans.find(isNumber);",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const readonlyArrayOfStringsNumbersAndBooleans = arrayOfStringsNumbersAndBooleans as ReadonlyArray<string | number | boo",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const readonlyFoundNumber: number | undefined = readonlyArrayOfStringsNumbersAndBooleans.find(isNumber);",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function isNumber(x: any): x is number {\r\n  return typeof x === \"number\";\r\n}\r\n\r\nconst arrayOfStringsNumbersAndBooleans =",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const readonlyFoundNumber: number | undefined = readonlyArrayOfStringsNumbersAndBooleans.find(isNumber);",
        "line": 13,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const readonlyFoundNumber: number | undefined = readonlyArrayOfStringsNumbersAndBooleans.find(isNumber)",
        "line": 13,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 497, end: 498 } }) at 500..505
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

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: triage-needed`; generated triage buckets are not done until split or superseded with evidence.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/676-implement-arrayFind.md` before this move
- `issues/done/676-implement-arrayFind.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
