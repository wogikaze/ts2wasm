---
id: 744
title: "Implement Assignmenttoinstantiationexpression"
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

Triage assignmentToInstantiationExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentToInstantiationExpression` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentToInstantiationExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts
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

- `reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: assignmentToInstantiationExpression

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 222,
  "lines": 13,
  "extension": ".ts",
  "first_code_line": "let obj: { fn?: <T>() => T } = {};"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Equal, span: Span { start: 87, end: 88 } }) at 89..90",
  "span_start": 89,
  "span_end": 90,
  "line": 5,
  "column": 18,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @strict: true
3 | 
4 | let obj: { fn?: <T>() => T } = {};
5 | obj.fn<number> = () => 1234;
6 | 
7 | 
8 | let getValue: <T>() => T;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "obj",
    "line": 4,
    "column": 1
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
        kind: Let,
        span: Span {
            start: 37,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "obj",
        ),
        span: Span {
            start: 41,
            end: 44,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "fn",
        ),
        span: Span {
            start: 48,
            end: 50,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 50,
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
        kind: Less,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 59,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "obj",
        ),
        span: Span {
            start: 72,
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
            "fn",
        ),
        span: Span {
            start: 76,
            end: 78,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 79,
            end: 85,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: LeftParen,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Equal, span: Span { start: 87, end: 88 } }) at 89..90
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Equal, span: Span { start: 87, end: 88 } }) at 89..90
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
        "code": 2364,
        "category": "Error",
        "message": "The left-hand side of an assignment expression must be a variable or a property access.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts",
        "start": 72,
        "length": 14,
        "line": 5,
        "character": 1
      },
      {
        "code": 2454,
        "category": "Error",
        "message": "Variable 'getValue' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts",
        "start": 129,
        "length": 8,
        "line": 9,
        "character": 1
      },
      {
        "code": 2364,
        "category": "Error",
        "message": "The left-hand side of an assignment expression must be a variable or a property access.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts",
        "start": 129,
        "length": 16,
        "line": 9,
        "character": 1
      },
      {
        "code": 2364,
        "category": "Error",
        "message": "The left-hand side of an assignment expression must be a variable or a property access.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts",
        "start": 190,
        "length": 17,
        "line": 13,
        "character": 1
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ fn?: (<T>() => T) | undefined; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts",
        "start": 41,
        "length": 3,
        "line": 4,
        "character": 5,
        "name": "obj"
      },
      {
        "kind": "binding",
        "typeText": "<T>() => T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts",
        "start": 107,
        "length": 8,
        "line": 8,
        "character": 5,
        "name": "getValue"
      },
      {
        "kind": "binding",
        "typeText": "<T>() => T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToInstantiationExpression.ts",
        "start": 166,
        "length": 9,
        "line": 12,
        "character": 5,
        "name": "getValue2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "let obj: { fn?: <T>() => T } = {};",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "obj.fn<number> = () => 1234;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let getValue: <T>() => T;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "getValue<number> = () => 1234;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let getValue2!: <T>() => T;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "getValue2<number> = () => 1234;",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "let obj: { fn?: <T>() => T } = {};\nobj.fn<number> = () => 1234;\n\n\nlet getValue: <T>() => T;\ngetValue<number> = () => 123",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "obj.fn<number> = () => 1234;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "obj.fn<number> = () => 1234",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ArrowFunction",
        "text": "() => 1234",
        "line": 5,
        "character": 18
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Equal, span: Span { start: 87, end: 88 } }) at 89..90
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
