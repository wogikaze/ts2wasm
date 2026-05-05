---
id: 727
title: "Implement Assignmentcompatforenums"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assignmentCompatForEnums across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentCompatForEnums` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompatForEnums has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignmentCompatForEnums

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 218,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "enum TokenType { One, Two };"
}
```

Failure location:

```json
{
  "code": "UnsupportedTypeScriptSyntax",
  "message": "TypeScript enum declarations require an explicit frontend transform before runtime lowering at 39..43",
  "span_start": 39,
  "span_end": 43,
  "line": 3,
  "column": 3,
  "feature_label": "parser-syntax",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @strict: false
3 | enum TokenType { One, Two };
4 | 
5 | var list = {};
6 |
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/442-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/464-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/550-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/663-implement-arrayAssignmentTest-parser-syntax.md",
    "title": "Implement Arrayassignmenttest Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/243-implement-numeric-literal-separator-parser.md",
    "title": "Implement numeric literal separator parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/244-implement-bigint-literal-parser-classification.md",
    "title": "Implement BigInt literal parser classification",
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
            "enum",
        ),
        span: Span {
            start: 39,
            end: 43,
        },
    },
    SpannedToken {
        kind: Ident(
            "TokenType",
        ),
        span: Span {
            start: 44,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Ident(
            "One",
        ),
        span: Span {
            start: 56,
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
        kind: Ident(
            "Two",
        ),
        span: Span {
            start: 61,
            end: 64,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 71,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "list",
        ),
        span: Span {
            start: 75,
            end: 79,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 91,
            end: 99,
        },
    },
    SpannedToken {
        kind: Ident(
            "returnType",
        ),
        span: Span {
            start: 100,
            end: 110,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "TokenType",
        ),
        span: Span {
            start: 114,
            end: 123,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 126,
            end: 132,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 133,
            end: 137,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 139,
            end: 140,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 144,
            end: 152,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 39..43
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 39..43
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
        "code": 2322,
        "category": "Error",
        "message": "Type 'null' is not assignable to type 'TokenType'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts",
        "start": 126,
        "length": 6,
        "line": 8,
        "character": 36
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{}",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts",
        "start": 75,
        "length": 4,
        "line": 5,
        "character": 5,
        "name": "list"
      },
      {
        "kind": "function",
        "typeText": "TokenType",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts",
        "start": 100,
        "length": 10,
        "line": 8,
        "character": 10,
        "name": "returnType"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts",
        "start": 153,
        "length": 3,
        "line": 10,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "binding",
        "typeText": "TokenType",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts",
        "start": 170,
        "length": 1,
        "line": 11,
        "character": 9,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "TokenType",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatForEnums.ts",
        "start": 199,
        "length": 1,
        "line": 13,
        "character": 9,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "EnumDeclaration",
        "text": "enum TokenType { One, Two }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "EmptyStatement",
        "text": ";",
        "line": 3,
        "character": 28
      },
      {
        "kind": "FirstStatement",
        "text": "var list = {};",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function returnType(): TokenType { return null; }",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo() {\r\n    var x = returnType();\r\n\r\n    var x: TokenType = list['one'];\r\n}",
        "line": 10,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "enum TokenType { One, Two };\r\n\r\nvar list = {};\r\n\r\n\r\nfunction returnType(): TokenType { return null; }\r\n\r\nfunction foo() ",
        "line": 3,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum TokenType { One, Two }",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 39..43
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
