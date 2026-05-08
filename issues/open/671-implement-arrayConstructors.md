---
id: 671
title: "Implement Arrayconstructors"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrayConstructors across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayConstructors` with diagnostics: new-expression. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayConstructors has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayConstructors1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayConstructors1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayConstructors1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayConstructors1.ts
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

- `reference/typescript/tests/cases/compiler/arrayConstructors1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage new expression: arrayConstructors1

- Issue class: `triage-needed`
- Feature label: `new-expression`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayConstructors1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayConstructors1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 203,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "var x: string[];"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-405: new Array(length) currently supports exactly one small non-negative integer length",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "new-expression",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
// @target: es2015
var x: string[];
x = new Array(1);
x = new Array('hi', 'bye'); 
x = new Array<string>('hi', 'bye');

var y: number[];
y = new Array(1);
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x",
    "line": 2,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "y",
    "line": 7,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/439-implement-new-expression.md",
    "title": "Implement new expression",
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
            start: 20,
            end: 23,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 24,
            end: 25,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 25,
            end: 26,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 27,
            end: 33,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 34,
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
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 42,
            end: 45,
        },
    },
    SpannedToken {
        kind: Ident(
            "Array",
        ),
        span: Span {
            start: 46,
            end: 51,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 61,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "Array",
        ),
        span: Span {
            start: 65,
            end: 70,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: String(
            "hi",
        ),
        span: Span {
            start: 71,
            end: 75,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: String(
            "bye",
        ),
        span: Span {
            start: 77,
            end: 82,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            sta
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "x",
        expr: Undefined {
            span: Span {
                start: 24,
                end: 25,
            },
        },
        span: Span {
            start: 20,
            end: 36,
        },
    },
    Assign {
        name: "x",
        expr: New {
            expr: Ident {
                name: "Array",
                span: Span {
                    start: 46,
                    end: 51,
                },
            },
            args: [
                Number {
                    value: 1,
                    span: Span {
                        start: 52,
                        end: 53,
                    },
                },
            ],
            span: Span {
                start: 42,
                end: 54,
            },
        },
        span: Span {
            start: 38,
            end: 55,
        },
    },
    Assign {
        name: "x",
        expr: New {
            expr: Ident {
                name: "Array",
                span: Span {
                    start: 65,
                    end: 70,
                },
            },
            args: [
                String {
                    value: "hi",
                    span: Span {
                        start: 71,
                        end: 75,
                    },
                },
                String {
                    value: "bye",
                    span: Span {
                        start: 77,
                        end: 82,
                    },
                },
            ],
            span: Span {
                start: 61,
                end: 83,
            },
        },
        span: Span {
            start: 57,
            end: 84,
        },
    },
    Assign {
        name: "x",
        expr: New {
            expr: Ident {
                name: "Array",
                span: Span {
                    start: 95,
                    end: 100,
                },
            },
            args: [
                String {
                    value: "hi",
                    span: Span {
                        start: 109,
                        end: 113,
                    },
                },
                String {
                    value: "bye",
                    span: Span {
                        start: 115,
                        end: 120,
                    },
                },
            ],
            span: Span {
                start: 91,
                end: 121,
            },
        },
        span: Span {
            start: 87,
            end: 122,
        },
    },
    Let {
        name: "y",
        expr: Undefined {
            span: Span {
                start: 130,
                end: 131,
            },
        },
        span: Span {
            start: 126,
            end: 142,
        },
    },
    Assign {
        name: "y",
        expr: New {
            expr: Ident {
                name: "Array",
                span: Span {
                    start: 152,
                    end: 157,
                },
            },
            args: [
                Number {
                    value: 1,
                    span: Span {
                        start: 158,
                        end: 159,
                    },
                },
            ],
            span: Span {
                start: 148,
                end: 160,
            },
        },
        span: Span {
            start: 144,
            e
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-405: new Array(length) currently supports exactly one small non-negative integer length
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
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayConstructors1.ts",
        "start": 24,
        "length": 1,
        "line": 2,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "number[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayConstructors1.ts",
        "start": 130,
        "length": 1,
        "line": 7,
        "character": 5,
        "name": "y"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
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
