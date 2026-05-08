---
id: 664
title: "Implement Arrayaugment"
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

Triage arrayAugment across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayAugment` with diagnostics: duplicate-local. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayAugment has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayAugment.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayAugment.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayAugment.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayAugment.ts
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

- `reference/typescript/tests/cases/compiler/arrayAugment.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage duplicate local: arrayAugment

- Issue class: `triage-needed`
- Feature label: `duplicate-local`
- Diagnostic: `DuplicateLocal` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/arrayAugment.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayAugment.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 157,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "interface Array<T> {"
}
```

Failure location:

```json
{
  "code": "DuplicateLocal",
  "message": "duplicate local binding: `y` at 121..139",
  "span_start": 121,
  "span_end": 139,
  "line": 8,
  "column": 8,
  "feature_label": "duplicate-local",
  "error_type": "compiler-diagnostic"
}
```

Source context:

```text
5 | 
6 | var x = [''];
7 | var y = x.split(4);
8 | var y: string[][]; // Expect no error here
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x",
    "line": 6,
    "column": 1,
    "initializer": "['']"
  },
  {
    "kind": "binding",
    "name": "y",
    "line": 7,
    "column": 1,
    "initializer": "x.split(4)"
  },
  {
    "kind": "binding",
    "name": "y",
    "line": 8,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "done",
    "path": "issues/done/298-allow-reused-for-loop-local-names.md",
    "title": "Allow reused for-loop local names in separate loop scopes",
    "reason": "same feature label, title overlap"
  }
]
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
            "Array",
        ),
        span: Span {
            start: 30,
            end: 35,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 37,
            end: 38,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "split",
        ),
        span: Span {
            start: 46,
            end: 51,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 51,
            end: 52,
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
            "parts",
        ),
        span: Span {
            start: 54,
            end: 59,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 61,
            end: 67,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 69,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: LeftBracket,
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
        kind: Semicolon,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 85,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: String(
            "",
        ),
        span: Span {
            start:
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "x",
        expr: Array {
            elements: [
                Present(
                    String {
                        value: "",
                        span: Span {
                            start: 94,
                            end: 96,
                        },
                    },
                ),
            ],
            span: Span {
                start: 93,
                end: 97,
            },
        },
        span: Span {
            start: 85,
            end: 98,
        },
    },
    Let {
        name: "y",
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "x",
                    span: Span {
                        start: 108,
                        end: 109,
                    },
                },
                property: "split",
                span: Span {
                    start: 108,
                    end: 115,
                },
            },
            args: [
                Number {
                    value: 4,
                    span: Span {
                        start: 116,
                        end: 117,
                    },
                },
            ],
            span: Span {
                start: 108,
                end: 118,
            },
        },
        span: Span {
            start: 100,
            end: 119,
        },
    },
    Let {
        name: "y",
        expr: Undefined {
            span: Span {
                start: 125,
                end: 126,
            },
        },
        span: Span {
            start: 121,
            end: 139,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [DuplicateLocal] duplicate local binding: `y` at 121..139
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
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAugment.ts",
        "start": 54,
        "length": 5,
        "line": 3,
        "character": 13,
        "name": "parts"
      },
      {
        "kind": "binding",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAugment.ts",
        "start": 89,
        "length": 1,
        "line": 6,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "string[][]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAugment.ts",
        "start": 104,
        "length": 1,
        "line": 7,
        "character": 5,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "string[][]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAugment.ts",
        "start": 125,
        "length": 1,
        "line": 8,
        "character": 5,
        "name": "y"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface Array<T> {\r\n    split: (parts: number) => T[][];\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x = [''];",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var y = x.split(4);",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var y: string[][];",
        "line": 8,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface Array<T> {\r\n    split: (parts: number) => T[][];\r\n}\r\n\r\nvar x = [''];\r\nvar y = x.split(4);\r\nvar y: string[][]; ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var y: string[][];",
        "line": 8,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var y: string[][]",
        "line": 8,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [DuplicateLocal] duplicate local binding: `y` at 121..139
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
