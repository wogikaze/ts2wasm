---
id: 746
title: "Implement Assignmenttoparenthesizedexpression"
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

Triage assignmentToParenthesizedExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentToParenthesizedExpression` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentToParenthesizedExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToParenthesizedExpression1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToParenthesizedExpression1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToParenthesizedExpression1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToParenthesizedExpression1.ts
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

- `reference/typescript/tests/cases/compiler/assignmentToParenthesizedExpression1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignmentToParenthesizedExpression1

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentToParenthesizedExpression1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToParenthesizedExpression1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 35,
  "lines": 3,
  "extension": ".ts",
  "first_code_line": "var x;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected RightParen, got Some(Comma) at 30..31",
  "span_start": 30,
  "span_end": 31,
  "line": 3,
  "column": 5,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | var x;
3 | (1, x)=0;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x",
    "line": 2,
    "column": 1
  }
]
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
    "path": "issues/open/464-implement-FunctionDeclaration-parser-syntax.md",
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
    "state": "open",
    "path": "issues/open/734-implement-assignmentCompatability-parser-syntax.md",
    "title": "Implement Assignmentcompatability Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065-implement-parser-syntax.md",
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
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `False`

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
        kind: Semicolon,
        span: Span {
            start: 25,
            end: 26,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 29,
            end: 30,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 34,
            end: 35,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 36,
            end: 37,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Comma) at 30..31
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Comma) at 30..31
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToParenthesizedExpression1.ts",
        "start": 28,
        "length": 6,
        "line": 3,
        "character": 1
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToParenthesizedExpression1.ts",
        "start": 29,
        "length": 1,
        "line": 3,
        "character": 2
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToParenthesizedExpression1.ts",
        "start": 24,
        "length": 1,
        "line": 2,
        "character": 5,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var x;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "(1, x)=0;",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var x;\r\n(1, x)=0;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "(1, x)=0;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "(1, x)=0",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ParenthesizedExpression",
        "text": "(1, x)",
        "line": 3,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "1, x",
        "line": 3,
        "character": 2
      },
      {
        "kind": "FirstLiteralToken",
        "text": "1",
        "line": 3,
        "character": 2
      },
      {
        "kind": "CommaToken",
        "text": ",",
        "line": 3,
        "character": 3
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Comma) at 30..31
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
