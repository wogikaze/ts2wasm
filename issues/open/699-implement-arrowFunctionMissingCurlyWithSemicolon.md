---
id: 699
title: "Implement Arrowfunctionmissingcurlywithsemicolon"
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

Triage arrowFunctionMissingCurlyWithSemicolon across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrowFunctionMissingCurlyWithSemicolon` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrowFunctionMissingCurlyWithSemicolon has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts
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

- `reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: arrowFunctionMissingCurlyWithSemicolon

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 122,
  "lines": 5,
  "extension": ".ts",
  "first_code_line": "var f = () => ;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 65, end: 66 } }) at 68..71",
  "span_start": 68,
  "span_end": 71,
  "line": 4,
  "column": 4,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // Should error at semicolon.
3 | var f = () => ;
4 | var b = 1 * 2 * 3 * 4;
5 | var square = (x: number) => x * x;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "f",
    "line": 3,
    "column": 1,
    "initializer": "() =>"
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
        kind: Var,
        span: Span {
            start: 51,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 62,
            end: 64,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 68,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Star,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Star,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Number(
            3,
        ),
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Star,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Number(
            4,
        ),
        span: Span {
            start: 88,
            end: 89,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 92,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "square",
        ),
        span: Span {
            start: 96,
            end: 102,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 109,
            end: 115,
        },
    },
    SpannedToken {
        kind: RightParen,
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
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 65, end: 66 } }) at 68..71
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 65, end: 66 } }) at 68..71
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
        "code": 1109,
        "category": "Error",
        "message": "Expression expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts",
        "start": 65,
        "length": 1,
        "line": 3,
        "character": 15
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "() => any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts",
        "start": 55,
        "length": 1,
        "line": 3,
        "character": 5,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts",
        "start": 72,
        "length": 1,
        "line": 4,
        "character": 5,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "(x: number) => number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts",
        "start": 96,
        "length": 6,
        "line": 5,
        "character": 5,
        "name": "square"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionMissingCurlyWithSemicolon.ts",
        "start": 106,
        "length": 1,
        "line": 5,
        "character": 15,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var f = () => ;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var b = 1 * 2 * 3 * 4;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var square = (x: number) => x * x;",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var f = () => ;\r\nvar b = 1 * 2 * 3 * 4;\r\nvar square = (x: number) => x * x;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var b = 1 * 2 * 3 * 4;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var b = 1 * 2 * 3 * 4",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 65, end: 66 } }) at 68..71
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
