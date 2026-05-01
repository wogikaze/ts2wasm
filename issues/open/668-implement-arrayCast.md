---
id: 668
title: "Implement Arraycast"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrayCast across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayCast` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayCast has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayCast.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayCast.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayCast.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayCast.ts
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

- `reference/typescript/tests/cases/compiler/arrayCast.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: arrayCast

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayCast.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayCast.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 339,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "<{ id: number; }[]>[{ foo: \"s\" }];"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 190, end: 191 } }) at 191..192",
  "span_start": 191,
  "span_end": 192,
  "line": 4,
  "column": 5,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // Should fail. Even though the array is contextually typed with { id: number }[], it still
3 | // has type { foo: string }[], which is not assignable to { id: number }[].
4 | <{ id: number; }[]>[{ foo: "s" }];
5 | 
6 | // Should succeed, as the {} element causes the type of the array to be {}[]
7 | <{ id: number; }[]>[{ foo: "s" }, {}];
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
        kind: Less,
        span: Span {
            start: 190,
            end: 191,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 191,
            end: 192,
        },
    },
    SpannedToken {
        kind: Ident(
            "id",
        ),
        span: Span {
            start: 193,
            end: 195,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 195,
            end: 196,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 197,
            end: 203,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 203,
            end: 204,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 205,
            end: 206,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 206,
            end: 207,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 207,
            end: 208,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 208,
            end: 209,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 209,
            end: 210,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 210,
            end: 211,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 212,
            end: 215,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 215,
            end: 216,
        },
    },
    SpannedToken {
        kind: String(
            "s",
        ),
        span: Span {
            start: 217,
            end: 220,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 221,
            end: 222,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 222,
            end: 223,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 223,
            end: 224,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 306,
            end: 307,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 307,
            end: 308,
        },
    },
    SpannedToken {
        kind: Ident(
            "id",
        ),
        span: Span {
            start: 309,
            end: 311,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 311,
            end: 312,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 313,
            end: 319,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 319,
            end: 320,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 321,
            end: 322,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 322,
            end: 323,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 190, end: 191 } }) at 191..192
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 190, end: 191 } }) at 191..192
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
        "code": 2352,
        "category": "Error",
        "message": "Conversion of type '{ foo: string; }[]' to type '{ id: number; }[]' may be a mistake because neither type sufficiently overlaps with the other. If this was intentional, convert the expression to 'unknown' first.\n  Object literal may only specify known properties, and 'foo' does not exist in type '{ id: number; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayCast.ts",
        "start": 212,
        "length": 3,
        "line": 4,
        "character": 23
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExpressionStatement",
        "text": "<{ id: number; }[]>[{ foo: \"s\" }];",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "<{ id: number; }[]>[{ foo: \"s\" }, {}];",
        "line": 7,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "<{ id: number; }[]>[{ foo: \"s\" }];\r\n\r\n// Should succeed, as the {} element causes the type of the array to be {}[]\r\n<{ i",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "<{ id: number; }[]>[{ foo: \"s\" }];",
        "line": 4,
        "character": 1
      },
      {
        "kind": "TypeAssertionExpression",
        "text": "<{ id: number; }[]>[{ foo: \"s\" }]",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ArrayType",
        "text": "{ id: number; }[]",
        "line": 4,
        "character": 2
      },
      {
        "kind": "TypeLiteral",
        "text": "{ id: number; }",
        "line": 4,
        "character": 2
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 190, end: 191 } }) at 191..192
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
