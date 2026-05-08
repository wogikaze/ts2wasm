---
id: 710
title: "Implement Asipublicprivateprotected"
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

Triage asiPublicPrivateProtected across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asiPublicPrivateProtected` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asiPublicPrivateProtected has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts
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

- `reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: asiPublicPrivateProtected

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 582,
  "lines": 43,
  "extension": ".ts",
  "first_code_line": "public"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"s\")) at 142..143",
  "span_start": 142,
  "span_end": 143,
  "line": 8,
  "column": 1,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 5 | public
 6 | class NonPublicClass {
 7 |     public s() {
 8 |     }
 9 | }
10 | 
11 | class NonPublicClass2 {
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "NonPublicClass",
    "line": 6,
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
    "path": "issues/open/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/243-implement-numeric-literal-separator-parser.md",
    "title": "Implement numeric literal separator parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/244-implement-bigint-literal-parser-classification.md",
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
            "public",
        ),
        span: Span {
            start: 99,
            end: 105,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 107,
            end: 112,
        },
    },
    SpannedToken {
        kind: Ident(
            "NonPublicClass",
        ),
        span: Span {
            start: 113,
            end: 127,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 135,
            end: 141,
        },
    },
    SpannedToken {
        kind: Ident(
            "s",
        ),
        span: Span {
            start: 142,
            end: 143,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 153,
            end: 154,
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
        kind: Class,
        span: Span {
            start: 161,
            end: 166,
        },
    },
    SpannedToken {
        kind: Ident(
            "NonPublicClass2",
        ),
        span: Span {
            start: 167,
            end: 182,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 183,
            end: 184,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 190,
            end: 196,
        },
    },
    SpannedToken {
        kind: Ident(
            "private",
        ),
        span: Span {
            start: 202,
            end: 209,
        },
    },
    SpannedToken {
        kind: Ident(
            "nonPublicFunction",
        ),
        span: Span {
            start: 210,
            end: 227,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 227,
            end: 228,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 228,
            end: 229,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 230,
            end: 231,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 237,
            end: 238,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 240,
            end: 241,
        },
    },
    SpannedToken {
        kind: Ident(
            "private",
        ),
        span: Span {
            start: 243,
            end: 250,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 252,
            end: 257,
        },
    },
    SpannedToken {
        kind: Ident(
            "NonPrivateClass",
        ),
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("s")) at 142..143
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("s")) at 142..143
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
        "code": 1212,
        "category": "Error",
        "message": "Identifier expected. 'public' is a reserved word in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts",
        "start": 99,
        "length": 6,
        "line": 5,
        "character": 1
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'public'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts",
        "start": 99,
        "length": 6,
        "line": 5,
        "character": 1
      },
      {
        "code": 1212,
        "category": "Error",
        "message": "Identifier expected. 'private' is a reserved word in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts",
        "start": 243,
        "length": 7,
        "line": 16,
        "character": 1
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'private'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts",
        "start": 243,
        "length": 7,
        "line": 16,
        "character": 1
      },
      {
        "code": 1212,
        "category": "Error",
        "message": "Identifier expected. 'protected' is a reserved word in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts",
        "start": 392,
        "length": 9,
        "line": 27,
        "character": 1
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'protected'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asiPublicPrivateProtected.ts",
        "start": 392,
        "length": 9,
        "line": 27,
        "character": 1
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExpressionStatement",
        "text": "public",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class NonPublicClass {\r\n    public s() {\r\n    }\r\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class NonPublicClass2 {\r\n    public\r\n    private nonPublicFunction() {\r\n    }\r\n}",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "private",
        "line": 16,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class NonPrivateClass {\r\n    private s() {\r\n    }\r\n}",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class NonPrivateClass2 {\r\n    private\r\n    public nonPrivateFunction() {\r\n    }\r\n}",
        "line": 22,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "protected",
        "line": 27,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class NonProtectedClass {\r\n  protected s() {\r\n  }\r\n}",
        "line": 28,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class NonProtectedClass2 {\r\n    protected\r\n    public nonProtectedFunction() {\r\n    }\r\n}",
        "line": 33,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class ClassWithThreeMembers {\r\n    public\r\n    private\r\n    protected\r\n}",
        "line": 39,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "public\r\nclass NonPublicClass {\r\n    public s() {\r\n    }\r\n}\r\n\r\nclass NonPublicClass2 {\r\n    public\r\n    private nonPublic",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class NonPublicClass {\r\n    public s() {\r\n    }\r\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "MethodDeclaration",
        "text": "public s() {\r\n    }",
        "line": 7,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "s",
        "line": 7,
        "character": 12
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("s")) at 142..143
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
