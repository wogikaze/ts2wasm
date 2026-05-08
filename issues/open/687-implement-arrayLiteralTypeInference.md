---
id: 687
title: "Implement Arrayliteraltypeinference"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5002]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrayLiteralTypeInference across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayLiteralTypeInference` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayLiteralTypeInference has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts
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

- `reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage type system: arrayLiteralTypeInference

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 695,
  "lines": 51,
  "extension": ".ts",
  "first_code_line": "﻿// @target: es2015"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Var) at 269..272",
  "span_start": 269,
  "span_end": 272,
  "line": 19,
  "column": 21,
  "feature_label": "type-system",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
16 |     { id: 3, name: "three" }
17 | ]
18 | 
19 | var x2: Action[] = [
20 |     new ActionA(),
21 |     new ActionB()
22 | ]
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Action",
    "line": 2,
    "column": 1
  },
  {
    "kind": "class",
    "name": "ActionA",
    "line": 6,
    "column": 1
  },
  {
    "kind": "class",
    "name": "ActionB",
    "line": 10,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "x1",
    "line": 14,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "x2",
    "line": 19,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/345-implement-tsc-type-alias-coverage.md",
    "title": "Implement TypeScript type alias coverage for tsc suite (23 cases)",
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
            start: 23,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "Action",
        ),
        span: Span {
            start: 29,
            end: 35,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "id",
        ),
        span: Span {
            start: 43,
            end: 45,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 47,
            end: 53,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 61,
            end: 66,
        },
    },
    SpannedToken {
        kind: Ident(
            "ActionA",
        ),
        span: Span {
            start: 67,
            end: 74,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 75,
            end: 82,
        },
    },
    SpannedToken {
        kind: Ident(
            "Action",
        ),
        span: Span {
            start: 83,
            end: 89,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Ident(
            "value",
        ),
        span: Span {
            start: 97,
            end: 102,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 104,
            end: 110,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 118,
            end: 123,
        },
    },
    SpannedToken {
        kind: Ident(
            "ActionB",
        ),
        span: Span {
            start: 124,
            end: 131,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 132,
            end: 139,
        },
    },
    SpannedToken {
        kind: Ident(
            "Action",
        ),
        span: Span {
            start: 140,
            end: 146,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: Ident(
            "trueNess",
        ),
        span: Span {
            start: 154,
            end: 162,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 162,
            end: 163,
        },
    },
    SpannedToken {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 269..272
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 269..272
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
        "code": 2564,
        "category": "Error",
        "message": "Property 'id' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 40,
        "length": 2,
        "line": 3,
        "character": 5
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'value' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 94,
        "length": 5,
        "line": 7,
        "character": 5
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'trueNess' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 151,
        "length": 8,
        "line": 11,
        "character": 5
      },
      {
        "code": 2353,
        "category": "Error",
        "message": "Object literal may only specify known properties, and 'trueness' does not exist in type 'Action'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 211,
        "length": 8,
        "line": 15,
        "character": 14
      },
      {
        "code": 2353,
        "category": "Error",
        "message": "Object literal may only specify known properties, and 'name' does not exist in type 'Action'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 244,
        "length": 4,
        "line": 16,
        "character": 14
      },
      {
        "code": 2353,
        "category": "Error",
        "message": "Object literal may only specify known properties, and 'trueness' does not exist in type '{ id: number; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 469,
        "length": 8,
        "line": 32,
        "character": 18
      },
      {
        "code": 2353,
        "category": "Error",
        "message": "Object literal may only specify known properties, and 'name' does not exist in type '{ id: number; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 506,
        "length": 4,
        "line": 33,
        "character": 18
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "Action[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 180,
        "length": 2,
        "line": 14,
        "character": 5,
        "name": "x1"
      },
      {
        "kind": "binding",
        "typeText": "Action[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 270,
        "length": 2,
        "line": 19,
        "character": 5,
        "name": "x2"
      },
      {
        "kind": "binding",
        "typeText": "Action[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 336,
        "length": 2,
        "line": 24,
        "character": 5,
        "name": "x3"
      },
      {
        "kind": "binding",
        "typeText": "{ id: number; }[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 421,
        "length": 2,
        "line": 30,
        "character": 5,
        "name": "z1"
      },
      {
        "kind": "binding",
        "typeText": "{ id: number; }[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 536,
        "length": 2,
        "line": 36,
        "character": 5,
        "name": "z2"
      },
      {
        "kind": "binding",
        "typeText": "{ id: number; }[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralTypeInference.ts",
        "start": 627,
        "length": 2,
        "line": 42,
        "character": 5,
        "name": "z3"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class Action {\r\n    id: number;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class ActionA extends Action {\r\n    value: string;\r\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class ActionB extends Action {\r\n    trueNess: boolean;\r\n}",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x1: Action[] = [\r\n    { id: 2, trueness: false },\r\n    { id: 3, name: \"three\" }\r\n]",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x2: Action[] = [\r\n    new ActionA(),\r\n    new ActionB()\r\n]",
        "line": 19,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x3: Action[] = [\r\n    new Action(),\r\n    new ActionA(),\r\n    new ActionB()\r\n]",
        "line": 24,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var z1: { id: number }[] =\r\n    [\r\n        { id: 2, trueness: false },\r\n        { id: 3, name: \"three\" }\r\n    ]",
        "line": 30,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var z2: { id: number }[] =\r\n    [\r\n        new ActionA(),\r\n        new ActionB()\r\n    ]",
        "line": 36,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var z3: { id: number }[] =\r\n    [\r\n        new Action(),\r\n        new ActionA(),\r\n        new ActionB()\r\n    ]",
        "line": 42,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Action {\r\n    id: number;\r\n}\r\n\r\nclass ActionA extends Action {\r\n    value: string;\r\n}\r\n\r\nclass ActionB extends Act",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x2: Action[] = [\r\n    new ActionA(),\r\n    new ActionB()\r\n]",
        "line": 19,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var x2: Action[] = [\r\n    new ActionA(),\r\n    new ActionB()\r\n]",
        "line": 19,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 269..272
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
