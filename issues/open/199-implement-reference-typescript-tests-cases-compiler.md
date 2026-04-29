---
id: 199
title: "Implement Compiler"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage reference/typescript/tests/cases/compiler across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `reference/typescript/tests/cases/compiler` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: reference/typescript/tests/cases/compiler has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/2dArrays.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/2dArrays.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/2dArrays.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/2dArrays.ts
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

- `reference/typescript/tests/cases/compiler/2dArrays.ts`

## Duplicate detection

- `issues/open/069-implement-APILibCheck.md` - Implement Apilibcheck (same feature label, same group key, title overlap)
- `issues/open/070-implement-APISample.md` - Implement Apisample (same feature label, same group key, title overlap)
- `issues/open/071-implement-ArrowFunctionExpression.md` - Implement Arrowfunctionexpression (same feature label, same group key, title overlap)
- `issues/open/072-implement-ClassDeclaration.md` - Implement Classdeclaration (same feature label, same group key, title overlap)
- `issues/open/073-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` - Implement Classdeclarationwithinvalidconstonpropertydeclaration (same feature label, same group key, title overlap)
- `issues/open/074-implement-DeclarationErrorsNoEmitOnError.md` - Implement Declarationerrorsnoemitonerror (same feature label, same group key, title overlap)
- `issues/open/076-implement-FunctionDeclaration.md` - Implement Functiondeclaration (same feature label, same group key, title overlap)
- `issues/done/077-implement-InterfaceDeclaration.md` - Implement Interfacedeclaration (same feature label, same group key, title overlap)
- `issues/open/078-implement-MemberAccessorDeclaration.md` - Implement Memberaccessordeclaration (same feature label, same group key, title overlap)
- `issues/open/079-implement-ParameterList.md` - Implement Parameterlist (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: 2dArrays

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/2dArrays.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/2dArrays.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 255,
  "lines": 16,
  "extension": ".ts",
  "first_code_line": "class Cell {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Colon) at 58..59",
  "span_start": 58,
  "span_end": 59,
  "line": 6,
  "column": 11,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | }
4 | 
5 | class Ship {
6 |     isSunk: boolean = false;
7 | }
8 | 
9 | class Board {
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Cell",
    "line": 2,
    "column": 1
  },
  {
    "kind": "class",
    "name": "Ship",
    "line": 5,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/199-implement-reference-TypeScript-tests-cases-compiler.md",
    "title": "Implement Compiler",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
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
        kind: Class,
        span: Span {
            start: 19,
            end: 24,
        },
    },
    SpannedToken {
        kind: Ident(
            "Cell",
        ),
        span: Span {
            start: 25,
            end: 29,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 35,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "Ship",
        ),
        span: Span {
            start: 41,
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
            "isSunk",
        ),
        span: Span {
            start: 52,
            end: 58,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "boolean",
        ),
        span: Span {
            start: 60,
            end: 67,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: False,
        span: Span {
            start: 70,
            end: 75,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: RightBr
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 58..59
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 58..59
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
        "kind": "parameter",
        "typeText": "Ship",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/2dArrays.ts",
        "start": 214,
        "length": 3,
        "line": 14,
        "character": 43,
        "name": "val"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class Cell {\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Ship {\n    isSunk: boolean = false;\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Board {\n    ships: Ship[] = [];\n    cells: Cell[] = [];\n\n    private allShipsSunk() {\n        return this.ships.ev",
        "line": 9,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Cell {\n}\n\nclass Ship {\n    isSunk: boolean = false;\n}\n\nclass Board {\n    ships: Ship[] = [];\n    cells: Cell[] = [",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Ship {\n    isSunk: boolean = false;\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "isSunk: boolean = false;",
        "line": 6,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "isSunk",
        "line": 6,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 58..59
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
