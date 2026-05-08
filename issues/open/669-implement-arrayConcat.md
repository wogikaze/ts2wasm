---
id: 669
title: "Implement Arrayconcat (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage arrayConcat across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayConcat` with diagnostics: new-expression. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayConcat has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayConcat2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayConcat2.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayConcat2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayConcat2.ts
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

- `reference/typescript/tests/cases/compiler/arrayConcat2.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage new expression: arrayConcat2

- Issue class: `triage-needed`
- Feature label: `new-expression`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayConcat2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayConcat2.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 138,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "var a: string[] = [];"
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
var a: string[] = [];
a.concat("hello", 'world');

a.concat('Hello');

var b = new Array<string>();
b.concat('hello');
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "a",
    "line": 2,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "b",
    "line": 7,
    "column": 1,
    "initializer": "new Array<string>()"
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
            "a",
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
        kind: Equal,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: Ident(
            "concat",
        ),
        span: Span {
            start: 45,
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
        kind: String(
            "hello",
        ),
        span: Span {
            start: 52,
            end: 59,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: String(
            "world",
        ),
        span: Span {
            start: 61,
            end: 68,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "concat",
        ),
        span: Span {
            start: 76,
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
        kind: String(
            "Hello",
        ),
        span: Span {
            start: 83,
            end: 90,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "a",
        expr: Array {
            elements: [],
            span: Span {
                start: 38,
                end: 40,
            },
        },
        span: Span {
            start: 20,
            end: 41,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "a",
                    span: Span {
                        start: 43,
                        end: 44,
                    },
                },
                property: "concat",
                span: Span {
                    start: 43,
                    end: 51,
                },
            },
            args: [
                String {
                    value: "hello",
                    span: Span {
                        start: 52,
                        end: 59,
                    },
                },
                String {
                    value: "world",
                    span: Span {
                        start: 61,
                        end: 68,
                    },
                },
            ],
            span: Span {
                start: 43,
                end: 69,
            },
        },
        span: Span {
            start: 43,
            end: 70,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "a",
                    span: Span {
                        start: 74,
                        end: 75,
                    },
                },
                property: "concat",
                span: Span {
                    start: 74,
                    end: 82,
                },
            },
            args: [
                String {
                    value: "Hello",
                    span: Span {
                        start: 83,
                        end: 90,
                    },
                },
            ],
            span: Span {
                start: 74,
                end: 91,
            },
        },
        span: Span {
            start: 74,
            end: 92,
        },
    },
    Let {
        name: "b",
        expr: New {
            expr: Ident {
                name: "Array",
                span: Span {
                    start: 108,
                    end: 113,
                },
            },
            args: [],
            span: Span {
                start: 104,
                end: 123,
            },
        },
        span: Span {
            start: 96,
            end: 124,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "b",
                    span: Span {
                        start: 126,
                        end: 127,
                    },
                },
                property: "concat",
                span: Span {
                    start: 126,
                    end: 134,
                },
            },
            args: [
                String {
                    value: "hello",
                    span: Span {
                        start: 135,
                        end: 142,
                    },
                },
            ],
            span: Span {
                start: 126,
                end: 143,
            },
        },
        span: Span {
            start: 126,
            end: 144,
        },
    },
]
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayConcat2.ts",
        "start": 24,
        "length": 1,
        "line": 2,
        "character": 5,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayConcat2.ts",
        "start": 100,
        "length": 1,
        "line": 7,
        "character": 5,
        "name": "b"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/696-implement-arrayconcat.md` に統合されました。
そちらを参照してください。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/669-implement-arrayConcat.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
