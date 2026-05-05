---
id: 739
title: "Implement Assignmentstricterconstraints"
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

Triage assignmentStricterConstraints across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentStricterConstraints` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentStricterConstraints has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts
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

- `reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignmentStricterConstraints

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 148,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "var f = function <T, S extends T>(x: T, y: S): void {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Less) at 37..38",
  "span_start": 37,
  "span_end": 38,
  "line": 2,
  "column": 19,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | var f = function <T, S extends T>(x: T, y: S): void {
3 |     x = y
4 | }
5 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "f",
    "line": 2,
    "column": 1,
    "initializer": "function <"
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
    "state": "open",
    "path": "issues/open/734-implement-assignmentCompatability-parser-syntax.md",
    "title": "Implement Assignmentcompatability Parser Syntax",
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
            "f",
        ),
        span: Span {
            start: 24,
            end: 25,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 28,
            end: 36,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 37,
            end: 38,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "S",
        ),
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 43,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 52,
            end: 53,
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
            "x",
        ),
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: Ident(
            "S",
        ),
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 67,
            end: 71,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 83,
            end: 84,
        }
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Less) at 37..38
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Less) at 37..38
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
        "message": "Type '<T, S extends T>(x: T, y: S) => void' is not assignable to type '<T, S>(x: T, y: S) => void'.\n  Types of parameters 'y' and 'y' are incompatible.\n    Type 'S' is not assignable to type 'T'.\n      'T' could be instantiated with an arbitrary type which could be unrelated to 'S'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts",
        "start": 140,
        "length": 1,
        "line": 8,
        "character": 1
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "<T, S extends T>(x: T, y: S) => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts",
        "start": 24,
        "length": 1,
        "line": 2,
        "character": 5,
        "name": "f"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts",
        "start": 54,
        "length": 1,
        "line": 2,
        "character": 35,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "S",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts",
        "start": 60,
        "length": 1,
        "line": 2,
        "character": 41,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "<T, S>(x: T, y: S) => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts",
        "start": 95,
        "length": 1,
        "line": 6,
        "character": 5,
        "name": "g"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts",
        "start": 115,
        "length": 1,
        "line": 6,
        "character": 25,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "S",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentStricterConstraints.ts",
        "start": 121,
        "length": 1,
        "line": 6,
        "character": 31,
        "name": "y"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var f = function <T, S extends T>(x: T, y: S): void {\r\n    x = y\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var g = function <T, S>(x: T, y: S): void { }",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "g = f",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "g(1, \"\")",
        "line": 9,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var f = function <T, S extends T>(x: T, y: S): void {\r\n    x = y\r\n}\r\n\r\nvar g = function <T, S>(x: T, y: S): void { }\r\n\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var f = function <T, S extends T>(x: T, y: S): void {\r\n    x = y\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var f = function <T, S extends T>(x: T, y: S): void {\r\n    x = y\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "f = function <T, S extends T>(x: T, y: S): void {\r\n    x = y\r\n}",
        "line": 2,
        "character": 5
      },
      {
        "kind": "FunctionExpression",
        "text": "function <T, S extends T>(x: T, y: S): void {\r\n    x = y\r\n}",
        "line": 2,
        "character": 9
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Less) at 37..38
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
