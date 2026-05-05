---
id: 704
title: "Implement Asiabstract"
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

Triage asiAbstract across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asiAbstract` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asiAbstract has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiAbstract.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiAbstract.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiAbstract.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiAbstract.ts
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

- `reference/typescript/tests/cases/compiler/asiAbstract.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: asiAbstract

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asiAbstract.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiAbstract.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 177,
  "lines": 16,
  "extension": ".ts",
  "first_code_line": "abstract"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"s\")) at 86..87",
  "span_start": 86,
  "span_end": 87,
  "line": 5,
  "column": 16,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @strict: false
3 | abstract
4 | class NonAbstractClass {
5 |   abstract s();
6 | }
7 | 
8 | class C2 {
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "NonAbstractClass",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 39,
            end: 47,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 49,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "NonAbstractClass",
        ),
        span: Span {
            start: 55,
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
            "abstract",
        ),
        span: Span {
            start: 77,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "s",
        ),
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: RightParen,
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
        kind: RightBrace,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 97,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "C2",
        ),
        span: Span {
            start: 103,
            end: 105,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 113,
            end: 121,
        },
    },
    SpannedToken {
        kind: Ident(
            "nonAbstractFunction",
        ),
        span: Span {
            start: 127,
            end: 146,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 156,
            end: 157,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 164,
            end: 169,
        },
    },
    SpannedToken {
        kind: Ident(
            "C3",
        ),
        span: Span {
            start: 170,
            end: 172,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 173,
            end: 174,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 180,
            end: 188,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 190,
            end: 191,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("s")) at 86..87
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("s")) at 86..87
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
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'abstract'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiAbstract.ts",
        "start": 39,
        "length": 8,
        "line": 3,
        "character": 1
      },
      {
        "code": 1244,
        "category": "Error",
        "message": "Abstract methods can only appear within an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiAbstract.ts",
        "start": 77,
        "length": 8,
        "line": 5,
        "character": 3
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExpressionStatement",
        "text": "abstract",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class NonAbstractClass {\r\n  abstract s();\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C2 {\r\n    abstract\r\n    nonAbstractFunction() {\r\n    }\r\n}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C3 {\r\n    abstract\r\n}",
        "line": 14,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "abstract\r\nclass NonAbstractClass {\r\n  abstract s();\r\n}\r\n\r\nclass C2 {\r\n    abstract\r\n    nonAbstractFunction() {\r\n    }\r\n",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class NonAbstractClass {\r\n  abstract s();\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "MethodDeclaration",
        "text": "abstract s();",
        "line": 5,
        "character": 3
      },
      {
        "kind": "Identifier",
        "text": "s",
        "line": 5,
        "character": 12
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("s")) at 86..87
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
