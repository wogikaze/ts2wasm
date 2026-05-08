---
id: 686
title: "Implement Arrayliteralcontextualtype"
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

Triage arrayLiteralContextualType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayLiteralContextualType` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayLiteralContextualType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts
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

- `reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: arrayLiteralContextualType

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 700,
  "lines": 30,
  "extension": ".ts",
  "first_code_line": "interface IAnimal {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Equal) at 91..92",
  "span_start": 91,
  "span_end": 92,
  "line": 7,
  "column": 16,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 4 | }
 5 | 
 6 | class Giraffe {
 7 |     name = "Giraffe";
 8 |     neckLength = "3m";
 9 | }
10 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Giraffe",
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
    "path": "issues/done/464-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/550-implement-FunctionDeclaration-parser-syntax.md",
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
            "IAnimal",
        ),
        span: Span {
            start: 30,
            end: 37,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Ident(
            "name",
        ),
        span: Span {
            start: 45,
            end: 49,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 51,
            end: 57,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 65,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "Giraffe",
        ),
        span: Span {
            start: 71,
            end: 78,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "name",
        ),
        span: Span {
            start: 86,
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
        kind: String(
            "Giraffe",
        ),
        span: Span {
            start: 93,
            end: 102,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "neckLength",
        ),
        span: Span {
            start: 109,
            end: 119,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: String(
            "3m",
        ),
        span: Span {
            start: 122,
            end: 126,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 134,
            end: 139,
        },
    },
    SpannedToken {
        kind: Ident(
            "Elephant",
        ),
        span: Span {
            start: 140,
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
        kind: Ident(
            "name",
        ),
        span: Span {
            start: 156,
            end: 160,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 161,
            end: 162,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 91..92
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 91..92
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
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts",
        "start": 219,
        "length": 3,
        "line": 16,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "IAnimal[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts",
        "start": 223,
        "length": 7,
        "line": 16,
        "character": 14,
        "name": "animals"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts",
        "start": 257,
        "length": 3,
        "line": 17,
        "character": 10,
        "name": "bar"
      },
      {
        "kind": "parameter",
        "typeText": "{ [n: number]: IAnimal; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts",
        "start": 261,
        "length": 7,
        "line": 17,
        "character": 14,
        "name": "animals"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts",
        "start": 273,
        "length": 1,
        "line": 17,
        "character": 26,
        "name": "n"
      },
      {
        "kind": "binding",
        "typeText": "(Giraffe | Elephant)[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralContextualType.ts",
        "start": 561,
        "length": 3,
        "line": 28,
        "character": 5,
        "name": "arr"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface IAnimal {\r\n    name: string;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Giraffe {\r\n    name = \"Giraffe\";\r\n    neckLength = \"3m\";\r\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Elephant {\r\n    name = \"Elephant\";\r\n    trunkDiameter = \"20cm\";\r\n}",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo(animals: IAnimal[]) { }",
        "line": 16,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function bar(animals: { [n: number]: IAnimal }) { }",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo([\r\n    new Giraffe(),\r\n    new Elephant()\r\n]);",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "bar([\r\n    new Giraffe(),\r\n    new Elephant()\r\n]);",
        "line": 23,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var arr = [new Giraffe(), new Elephant()];",
        "line": 28,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo(arr);",
        "line": 29,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "bar(arr);",
        "line": 30,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface IAnimal {\r\n    name: string;\r\n}\r\n\r\nclass Giraffe {\r\n    name = \"Giraffe\";\r\n    neckLength = \"3m\";\r\n}\r\n\r\nclass ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Giraffe {\r\n    name = \"Giraffe\";\r\n    neckLength = \"3m\";\r\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "name = \"Giraffe\";",
        "line": 7,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 91..92
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
