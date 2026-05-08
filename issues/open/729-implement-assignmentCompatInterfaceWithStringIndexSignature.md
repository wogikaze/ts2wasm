---
id: 729
title: "Implement Assignmentcompatinterfacewithstringindexsignature"
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

Triage assignmentCompatInterfaceWithStringIndexSignature across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentCompatInterfaceWithStringIndexSignature` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompatInterfaceWithStringIndexSignature has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignmentCompatInterfaceWithStringIndexSignature

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 230,
  "lines": 17,
  "extension": ".ts",
  "first_code_line": "interface IHandler {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"Boz\")) at 170..173",
  "span_start": 170,
  "span_end": 173,
  "line": 12,
  "column": 23,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 9 | }
10 | 
11 | class Foo {
12 |     public Boz(): void { }
13 | }
14 | 
15 | function Biz(map: IHandlerMap) { }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Foo",
    "line": 11,
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
            "interface",
        ),
        span: Span {
            start: 39,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "IHandler",
        ),
        span: Span {
            start: 49,
            end: 57,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Ident(
            "e",
        ),
        span: Span {
            start: 66,
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
        kind: Colon,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Ident(
            "boolean",
        ),
        span: Span {
            start: 70,
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
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 85,
            end: 94,
        },
    },
    SpannedToken {
        kind: Ident(
            "IHandlerMap",
        ),
        span: Span {
            start: 95,
            end: 106,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Ident(
            "type",
        ),
        span: Span {
            start: 115,
            end: 119,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 121,
            end: 127,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "IHandler",
        ),
        span: Span {
            start: 130,
            end: 138,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 138,
            end: 139,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 141,
            end: 142,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 146,
            end: 151,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 152,
            end: 155,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 156,
            end: 157,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("Boz")) at 170..173
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("Boz")) at 170..173
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
        "message": "Argument of type 'Foo' is not assignable to parameter of type 'IHandlerMap'.\n  Index signature for type 'string' is missing in type 'Foo'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts",
        "start": 234,
        "length": 9,
        "line": 17,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts",
        "start": 66,
        "length": 1,
        "line": 4,
        "character": 6,
        "name": "e"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts",
        "start": 115,
        "length": 4,
        "line": 8,
        "character": 6,
        "name": "type"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts",
        "start": 201,
        "length": 3,
        "line": 15,
        "character": 10,
        "name": "Biz"
      },
      {
        "kind": "parameter",
        "typeText": "IHandlerMap",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatInterfaceWithStringIndexSignature.ts",
        "start": 205,
        "length": 3,
        "line": 15,
        "character": 14,
        "name": "map"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface IHandler {\r\n    (e): boolean;\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface IHandlerMap {\r\n    [type: string]: IHandler;\r\n}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Foo {\r\n    public Boz(): void { }\r\n}",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function Biz(map: IHandlerMap) { }",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Biz(new Foo());",
        "line": 17,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface IHandler {\r\n    (e): boolean;\r\n}\r\n\r\ninterface IHandlerMap {\r\n    [type: string]: IHandler;\r\n}\r\n\r\nclass Foo {\r\n",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Foo {\r\n    public Boz(): void { }\r\n}",
        "line": 11,
        "character": 1
      },
      {
        "kind": "MethodDeclaration",
        "text": "public Boz(): void { }",
        "line": 12,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "Boz",
        "line": 12,
        "character": 12
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("Boz")) at 170..173
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
