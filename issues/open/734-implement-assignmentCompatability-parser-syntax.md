---
id: 734
title: "Implement Assignmentcompatability Parser Syntax"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assignmentCompatability-parser-syntax across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentCompatability-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompatability-parser-syntax has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability46.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatability46.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatability46.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability46.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompatability46.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/open/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/done/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)
- `issues/done/200-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/243-implement-numeric-literal-separator-parser.md` - Implement numeric literal separator parser support (same feature label, title overlap)
- `issues/done/244-implement-bigint-literal-parser-classification.md` - Implement BigInt literal parser classification (same feature label, title overlap)
- `issues/done/246-implement-optional-chaining-parser-support.md` - Implement optional chaining parser support (same feature label, title overlap)
- `issues/done/247-implement-destructuring-binding-pattern-parser.md` - Implement destructuring binding pattern parser support (same feature label, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: assignmentCompatability46

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentCompatability46.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability46.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 90,
  "lines": 5,
  "extension": ".ts",
  "first_code_line": "declare function fn(x: never): void;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"fn\")) at 71..73",
  "span_start": 71,
  "span_end": 73,
  "line": 5,
  "column": 1,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | declare function fn(x: never): void;
3 | 
4 | fn([1, 2, 3])
5 | fn({ a: 1, b: 2 })
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "fn",
    "line": 2,
    "column": 9,
    "params": "x: never"
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 19,
            end: 26,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 27,
            end: 35,
        },
    },
    SpannedToken {
        kind: Ident(
            "fn",
        ),
        span: Span {
            start: 36,
            end: 38,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "never",
        ),
        span: Span {
            start: 42,
            end: 47,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 50,
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
            "fn",
        ),
        span: Span {
            start: 57,
            end: 59,
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
        kind: LeftBracket,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 61,
            end: 62,
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
        kind: Number(
            2,
        ),
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Number(
            3,
        ),
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "fn",
        ),
        span: Span {
            start: 71,
            end: 73,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            st
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("fn")) at 71..73
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("fn")) at 71..73
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
        "code": 2345,
        "category": "Error",
        "message": "Argument of type 'number[]' is not assignable to parameter of type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability46.ts",
        "start": 60,
        "length": 9,
        "line": 4,
        "character": 4
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type '{ a: number; b: number; }' is not assignable to parameter of type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability46.ts",
        "start": 74,
        "length": 14,
        "line": 5,
        "character": 4
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability46.ts",
        "start": 36,
        "length": 2,
        "line": 2,
        "character": 18,
        "name": "fn"
      },
      {
        "kind": "parameter",
        "typeText": "never",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability46.ts",
        "start": 39,
        "length": 1,
        "line": 2,
        "character": 21,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "declare function fn(x: never): void;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "fn([1, 2, 3])",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "fn({ a: 1, b: 2 })",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare function fn(x: never): void;\n\nfn([1, 2, 3])\nfn({ a: 1, b: 2 })\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "fn({ a: 1, b: 2 })",
        "line": 5,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "fn({ a: 1, b: 2 })",
        "line": 5,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "fn",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("fn")) at 71..73
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
