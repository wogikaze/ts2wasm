---
id: 694
title: "Implement Arraytolocalestringes Unknown Unsupported"
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

Triage arrayToLocaleStringES-unknown-unsupported across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `arrayToLocaleStringES-unknown-unsupported` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayToLocaleStringES-unknown-unsupported has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts
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

- `reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts`
- `reference/typescript/tests/cases/compiler/arrayToLocaleStringES2020.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: arrayToLocaleStringES2015

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 2753,
  "lines": 63,
  "extension": ".ts",
  "first_code_line": "let str: string;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 662, end: 663 } }) at 663..664",
  "span_start": 663,
  "span_end": 664,
  "line": 18,
  "column": 46,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
15 | str = mixed.toLocaleString(); // OK
16 | str = mixed.toLocaleString('fr'); // OK
17 | str = mixed.toLocaleString('de', { style: 'currency', currency: 'EUR' }); // OK
18 | str = (mixed as ReadonlyArray<number | Date>).toLocaleString('de', { currency: 'EUR', style: 'currency', timeZone: 'UTC' }); // OK
19 | 
20 | const int8Array = new Int8Array(3);
21 | str = int8Array.toLocaleString(); // OK
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "str",
    "line": 3,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "arr",
    "line": 4,
    "column": 1,
    "initializer": "[1, 2, 3]"
  },
  {
    "kind": "binding",
    "name": "dates",
    "line": 9,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "mixed",
    "line": 14,
    "column": 1,
    "initializer": "[1, new Date(), 59782, new Date()]"
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
        kind: Let,
        span: Span {
            start: 20,
            end: 23,
        },
    },
    SpannedToken {
        kind: Ident(
            "str",
        ),
        span: Span {
            start: 24,
            end: 27,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 27,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 29,
            end: 35,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 37,
            end: 42,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr",
        ),
        span: Span {
            start: 43,
            end: 46,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Number(
            3,
        ),
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "str",
        ),
        span: Span {
            start: 60,
            end: 63,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr",
        ),
        span: Span {
            start: 66,
            end: 69,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "toLocaleString",
        ),
        span: Span {
            start: 70,
            end: 84,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "str",
        ),
        span: Span {
            start: 94,
            end: 97,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            st
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 662, end: 663 } }) at 663..664
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 662, end: 663 } }) at 663..664
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
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 24,
        "length": 3,
        "line": 3,
        "character": 5,
        "name": "str"
      },
      {
        "kind": "binding",
        "typeText": "number[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 43,
        "length": 3,
        "line": 4,
        "character": 7,
        "name": "arr"
      },
      {
        "kind": "binding",
        "typeText": "readonly Date[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 223,
        "length": 5,
        "line": 9,
        "character": 7,
        "name": "dates"
      },
      {
        "kind": "binding",
        "typeText": "(number | Date)[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 418,
        "length": 5,
        "line": 14,
        "character": 7,
        "name": "mixed"
      },
      {
        "kind": "binding",
        "typeText": "Int8Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 756,
        "length": 9,
        "line": 20,
        "character": 7,
        "name": "int8Array"
      },
      {
        "kind": "binding",
        "typeText": "Uint8Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 967,
        "length": 10,
        "line": 25,
        "character": 7,
        "name": "uint8Array"
      },
      {
        "kind": "binding",
        "typeText": "Uint8ClampedArray<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 1183,
        "length": 17,
        "line": 30,
        "character": 7,
        "name": "uint8ClampedArray"
      },
      {
        "kind": "binding",
        "typeText": "Int16Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 1434,
        "length": 10,
        "line": 35,
        "character": 7,
        "name": "int16Array"
      },
      {
        "kind": "binding",
        "typeText": "Uint16Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 1650,
        "length": 11,
        "line": 40,
        "character": 7,
        "name": "uint16Array"
      },
      {
        "kind": "binding",
        "typeText": "Int32Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 1871,
        "length": 10,
        "line": 45,
        "character": 7,
        "name": "int32Array"
      },
      {
        "kind": "binding",
        "typeText": "Uint32Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 2087,
        "length": 11,
        "line": 50,
        "character": 7,
        "name": "uint32Array"
      },
      {
        "kind": "binding",
        "typeText": "Float32Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 2308,
        "length": 12,
        "line": 55,
        "character": 7,
        "name": "float32Array"
      },
      {
        "kind": "binding",
        "typeText": "Float64Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES2015.ts",
        "start": 2534,
        "length": 12,
        "line": 60,
        "character": 7,
        "name": "float64Array"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "let str: string;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const arr = [1, 2, 3];",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = arr.toLocaleString();",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = arr.toLocaleString('en-US');",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = arr.toLocaleString('en-US', { style: 'currency', currency: 'EUR' });",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const dates: readonly Date[] = [new Date(), new Date()];",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = dates.toLocaleString();",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = dates.toLocaleString('fr');",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = dates.toLocaleString('fr', { timeZone: 'UTC' });",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const mixed = [1, new Date(), 59782, new Date()];",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = mixed.toLocaleString();",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = mixed.toLocaleString('fr');",
        "line": 16,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = mixed.toLocaleString('de', { style: 'currency', currency: 'EUR' });",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = (mixed as ReadonlyArray<number | Date>).toLocaleString('de', { currency: 'EUR', style: 'currency', timeZone: 'UTC'",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const int8Array = new Int8Array(3);",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = int8Array.toLocaleString();",
        "line": 21,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = int8Array.toLocaleString('en-US');",
        "line": 22,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = int8Array.toLocaleString('en-US', { style: 'currency', currency: 'EUR' });",
        "line": 23,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const uint8Array = new Uint8Array(3);",
        "line": 25,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = uint8Array.toLocaleString();",
        "line": 26,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "let str: string;\nconst arr = [1, 2, 3];\nstr = arr.toLocaleString(); // OK\nstr = arr.toLocaleString('en-US'); // OK\nstr =",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = (mixed as ReadonlyArray<number | Date>).toLoca
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 662, end: 663 } }) at 663..664
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
