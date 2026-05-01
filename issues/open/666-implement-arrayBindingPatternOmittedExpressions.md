---
id: 666
title: "Implement Arraybindingpatternomittedexpressions"
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

Triage arrayBindingPatternOmittedExpressions across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayBindingPatternOmittedExpressions` with diagnostics: destructuring. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayBindingPatternOmittedExpressions has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts
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

- `reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage destructuring: arrayBindingPatternOmittedExpressions

- Issue class: `triage-needed`
- Feature label: `destructuring`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 197,
  "lines": 17,
  "extension": ".ts",
  "first_code_line": "var results: string[];"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected identifier or string literal as object key, got Some(Let) at 52..55",
  "span_start": 52,
  "span_end": 55,
  "line": 6,
  "column": 10,
  "feature_label": "destructuring",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | var results: string[];
4 | 
5 | {
6 |     let [, b, , a] = results;
7 |     let x = {
8 |         a,
9 |         b
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "results",
    "line": 3,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/425-implement-destructuring.md",
    "title": "Implement destructuring",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/247-implement-destructuring-binding-pattern-parser.md",
    "title": "Implement destructuring binding pattern parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/251-implement-destructuring-binding-runtime-semantics.md",
    "title": "Implement destructuring binding runtime semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/252-implement-destructuring-assignment-pattern-parser.md",
    "title": "Implement destructuring assignment pattern parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/289-resolve-callcount-binding-in-class-destructuring.md",
    "title": "Resolve callCount binding in class destructuring tests",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/292-resolve-initcount-binding-in-class-destructuring.md",
    "title": "Resolve initCount binding in class destructuring defaults",
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
            start: 19,
            end: 22,
        },
    },
    SpannedToken {
        kind: Ident(
            "results",
        ),
        span: Span {
            start: 23,
            end: 30,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 32,
            end: 38,
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
        kind: LeftBrace,
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 52,
            end: 55,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "results",
        ),
        span: Span {
            start: 69,
            end: 76,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 83,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 114,
            end: 115,
        },
    },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected identifier or string literal as object key, got Some(Let) at 52..55
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected identifier or string literal as object key, got Some(Let) at 52..55
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
        "code": 2454,
        "category": "Error",
        "message": "Variable 'results' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts",
        "start": 69,
        "length": 7,
        "line": 6,
        "character": 22
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts",
        "start": 23,
        "length": 7,
        "line": 3,
        "character": 5,
        "name": "results"
      },
      {
        "kind": "binding",
        "typeText": "{ a: string; b: string; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts",
        "start": 87,
        "length": 1,
        "line": 7,
        "character": 9,
        "name": "x"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBindingPatternOmittedExpressions.ts",
        "start": 140,
        "length": 1,
        "line": 14,
        "character": 10,
        "name": "f"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var results: string[];",
        "line": 3,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    let [, b, , a] = results;\r\n    let x = {\r\n        a,\r\n        b\r\n    }\r\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f([, a, , b, , , , s, , , ] = results) {\r\n    a = s[1];\r\n    b = s[2];\r\n}",
        "line": 14,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var results: string[];\r\n\r\n{\r\n    let [, b, , a] = results;\r\n    let x = {\r\n        a,\r\n        b\r\n    }\r\n}\r\n\r\n\r\nfunction",
        "line": 3,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    let [, b, , a] = results;\r\n    let x = {\r\n        a,\r\n        b\r\n    }\r\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let [, b, , a] = results;",
        "line": 6,
        "character": 5
      },
      {
        "kind": "VariableDeclarationList",
        "text": "let [, b, , a] = results",
        "line": 6,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected identifier or string literal as object key, got Some(Let) at 52..55
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
